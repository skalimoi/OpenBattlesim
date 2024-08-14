shader_type spatial;


//#define DEBUG_MODE_NORMAL
//#define DEBUG_MODE_VERTEX
//#define DEBUG_MODE_TEXTURE

render_mode blend_mix,depth_draw_opaque,cull_back,diffuse_burley,specular_schlick_ggx;

#include "transvoxel.gdshaderinc"
#include "virtual_texturing.gdshaderinc"
#include "triplanar.gdshaderinc"
#include "triplanar_scaled.gdshaderinc"
#include "lod_fade.gdshaderinc"
#include "voxel_texturing.gdshaderinc"


group_uniforms side_texture;
uniform float u_side_uv_scales[16];
uniform sampler2DArray u_side_albedo_array : source_color, filter_nearest_mipmap;
uniform sampler2DArray u_side_normal_array : hint_roughness_normal, filter_nearest_mipmap;


group_uniforms top_texture;
uniform float top_normal_affect : hint_range(-0.2, 0.2, 0.01) = 0.0;
uniform float top_depth_affect : hint_range(0.0, 0.2, 0.01) = 0.0;
uniform float top_noise_affect : hint_range(-0.2, 0.2, 0.01) = 0.0;
uniform float u_top_uv_scales[16];
uniform sampler2DArray u_top_texture_array : source_color, filter_nearest_mipmap;
uniform sampler2DArray u_top_normal_array : hint_roughness_normal, filter_nearest_mipmap;
group_uniforms;

uniform sampler2D u_blend_noise2d : source_color; // Used to perturb top-side texture mix
uniform sampler2D u_test_uv_texture : source_color; // You can plug in a checkered texture to test UV mapping

uniform float u_specular = 0.5;

uniform bool u_depth_blending_atlas = true;
uniform bool u_depth_blending_layer = true;
//group_uniforms voxel;

varying vec3 v_world_pos;
varying vec3 v_vertex_pos_model;

varying vec4 v_material_indices;
varying vec4 v_material_weights;

varying vec3 v_auto_side_indices;
varying vec3 v_auto_side_weights;

varying vec3 v_top_indices;
varying vec3 v_top_weights;


// taken from https://github.com/Zylann/godot_heightmap_plugin/blob/master/addons/zylann.hterrain/shaders/multisplat16.gdshader
// Turned out to not be that necessary if we use builtin NORMAL_MAP frag input
vec3 unpack_normal(vec4 rgba) {
	// If we consider texture space starts from top-left corner and Y goes down,
	// then Y+ in pixel space corresponds to Z+ in terrain space,
	// while X+ also corresponds to X+ in terrain space.
	vec3 n = rgba.xzy * 2.0 - vec3(1.0);
	// Had to negate Z because it comes from Y in the normal map,
	// and OpenGL-style normal maps are Y-up.
	n.z *= -1.0;
	return n;
}

// Returns weights modified according to material's bump map. Amount controls strength of this effect.
// This implementation is different than one found in HTerrain shader
// Function comes in 3 versions for diff number of inputs
vec4 get_depth_blended_weights4(vec4 weights, vec4 heights, float amount) {
	vec4 h = weights + heights * weights;
	float max_h = max(max(max(h.r, h.g), h.b), h.a) - amount;
	vec4 d = max(h - max_h, vec4(0.0));

	return d / (d.r + d.g + d.b + d.a) * (weights.r + weights.g + weights.b + weights.a);
}

vec3 get_depth_blended_weights3(vec3 weights, vec3 heights, float amount) {
	vec3 h = weights+heights*weights;
	float max_h = max(max(h.r, h.g), h.b) - amount;
	vec3 d = max(h-max_h, vec3(0.0));

	return d / (d.r+d.g+d.b)*(weights.r+weights.g+weights.b);
}

vec2 get_depth_blended_weights2(vec2 weights, vec2 heights, float amount) {
	vec2 h = weights + heights * weights;
	float max_h = max(h.r, h.g) - amount;
	vec2 d = max(h - max_h, vec2(0.0));

	return d / (d.r + d.g) * (weights.r + weights.g);
}


void vertex() {
	VERTEX = get_transvoxel_position(VERTEX, CUSTOM0);
	v_vertex_pos_model = VERTEX;

	// The tangent space calculations are lifted directly from Godot Standard Shader code
	// and modified to fit the updated triplanar function
	vec3 normal = MODEL_NORMAL_MATRIX * NORMAL;

	TANGENT = vec3(0.0, 0.0, -1.0) * normal.x; //de-absolutized, fixes x-axis lighting
	TANGENT += vec3(-1.0, 0.0, 0.0) * abs(normal.y); //flipped
	TANGENT += vec3(1.0, 0.0, 0.0) * normal.z; //de-absolutized, fixes z-axis lighting
	TANGENT = inverse(MODEL_NORMAL_MATRIX) * normalize(TANGENT);

	BINORMAL = vec3(0.0, 1.0, 0.0) * abs(normal.x);
	BINORMAL += vec3(0.0, 0.0, 1.0) * abs(normal.y); //flipped to fix y axis lighting
	BINORMAL += vec3(0.0, 1.0, 0.0) * abs(normal.z);
	BINORMAL = inverse(MODEL_NORMAL_MATRIX) * normalize(BINORMAL);

	//We'll need this later in Fragment. It's better to calculate it here
	v_world_pos = (MODEL_MATRIX * vec4(VERTEX, 1.0)).xyz;


	 // Indices are integer values so we can decode them as-is
    v_material_indices = decode_8bit_vec4(CUSTOM1.x);

	// Weights must be in [0..1] so we divide them
    v_material_weights = decode_8bit_vec4(CUSTOM1.y) / 255.0;
	
	//This is a crucial step because non-normalized weights cause trouble
	//The smol number is to ensure there's never a divide by zero
	v_material_weights /= v_material_weights.x + v_material_weights.y + v_material_weights.z + v_material_weights.w + 0.000001;

	v_auto_side_indices = vec3(1.0, 2.0, 5.0);

	//Automaterial function for side texture goes here
	v_auto_side_weights[0] = smoothstep(100.0, 150.0, v_world_pos.y); //cliff

	v_auto_side_weights[2] = 1.0-smoothstep(5.0, 8.0, v_world_pos.y); //sand

	v_auto_side_weights[1] = 1.0-v_auto_side_weights[2]-v_auto_side_weights[0]; //dirt

	//normalize auto-material weights
	v_auto_side_weights /= v_auto_side_weights.x+v_auto_side_weights.y+v_auto_side_weights.z+0.0001;




	//Automaterial logic for top tex choice goes here

	//Test implementation places textures in 4 quadrants around world origin
	//float grass_weight = smoothstep(-2.0, 2.0, v_world_pos.x)*smoothstep(-2.0, 2.0, v_world_pos.z);
	//float moss_weight = smoothstep(2.0, -2.0, v_world_pos.x)*smoothstep(2.0, -2.0, v_world_pos.z);
	//float forest_weight =  smoothstep(-2.0, 2.0, v_world_pos.x)*smoothstep(2.0, -2.0, v_world_pos.z);
	//float sand_weight =  smoothstep(2.0, -2.0, v_world_pos.x)*smoothstep(2.0, -2.0, v_world_pos.z);

	//Very simple height-based side texture placement
	float grass_weight = smoothstep(5.0, 8.0, v_world_pos.y)- smoothstep(100.0, 150.0, v_world_pos.y);
	float forest_weight =  smoothstep(100.0, 150.0, v_world_pos.y);
	float sand_weight =  1.0-smoothstep(5.0, 8.0, v_world_pos.y);
	float ice_weight;

	v_top_indices = vec3(1.0, 4.0, 5.0); //your indices for grass, forest and sand go here. TODO An advanced logic.
	v_top_weights = vec3(grass_weight, forest_weight, sand_weight);
	//v_top_weights = vec3(0.0);
	 //commented out because not necessary in simple 3 weight example, but might be necessary w advanced func
	//v_top_weights /= max(1.0, v_top_weights.x+v_top_weights.y+v_top_weights.z);
}

void fragment() {
	
	NORMAL = get_voxel_normal_view(v_vertex_pos_model, NORMAL, VIEW_MATRIX * MODEL_MATRIX);

	vec3 world_normal = (INV_VIEW_MATRIX * vec4(NORMAL, 0.0)).xyz;
	vec3 world_pos = v_world_pos;

	// can be used for distance fading of larger UVs and the like
	float dist = length((INV_VIEW_MATRIX*vec4(VERTEX, 1.0)).xyz-CAMERA_POSITION_WORLD);

	//How much of an overlap the three projected textures have
	float triplanar_sharpness = 8.0;
	vec3 blending = get_triplanar_blend(world_normal, triplanar_sharpness);

	// Sample the 4 blending textures, all with triplanar mapping.
    // We can re-use the same triplanar blending factors for all of them so separating that part
    // of the function improves performance a little.
	vec4 ab[4];
	vec4 nr[4];
	float auto_top_weight = 0.0; //In this implementation we will remove top texture in non-automaterial fragments and use this var


	//We do some sign flipping to fix textures appearing flipped in negative coordinates
	float x_sign = sign(dot(world_normal, vec3(0.0, 0.0, 1.0)));
	float z_sign = sign(dot(world_normal, vec3(-1.0, 0.0, 0.0)));
	vec3 tr_signs = vec3(x_sign, -1.0, z_sign);

	for (int i=0; i<4; i++){
		//might save a few array reads, idk
		float index = min(v_material_indices[i], 7.0);
		if (v_material_indices[i] != 0.0){
			ab[i] = texture_array_triplanar_scaled(u_side_albedo_array, world_pos, tr_signs, blending, index, u_side_uv_scales[int(index)]);
			nr[i] = texture_array_triplanar_scaled(u_side_normal_array, world_pos, tr_signs, blending, index, u_side_uv_scales[int(index)]);
		}
		else{
			vec4 auto_ab0 = texture_array_triplanar_scaled(u_side_albedo_array, world_pos, tr_signs, blending, v_auto_side_indices[0], u_side_uv_scales[int(v_auto_side_indices[0])]);
			vec4 auto_ab1 = texture_array_triplanar_scaled(u_side_albedo_array, world_pos, tr_signs, blending, v_auto_side_indices[1], u_side_uv_scales[int(v_auto_side_indices[1])]);
			vec4 auto_ab2 = texture_array_triplanar_scaled(u_side_albedo_array, world_pos, tr_signs, blending, v_auto_side_indices[2], u_side_uv_scales[int(v_auto_side_indices[2])]);

			vec4 auto_nr0 = texture_array_triplanar_scaled(u_side_normal_array, world_pos, tr_signs, blending, v_auto_side_indices[0], u_side_uv_scales[int(v_auto_side_indices[0])]);
			vec4 auto_nr1 = texture_array_triplanar_scaled(u_side_normal_array, world_pos, tr_signs, blending, v_auto_side_indices[1], u_side_uv_scales[int(v_auto_side_indices[1])]);
			vec4 auto_nr2 = texture_array_triplanar_scaled(u_side_normal_array, world_pos, tr_signs, blending, v_auto_side_indices[2], u_side_uv_scales[int(v_auto_side_indices[2])]);

			vec3 auto_weights = v_auto_side_weights;
			
			if (u_depth_blending_atlas){
				get_depth_blended_weights3(v_auto_side_weights, vec3(auto_nr0.a, auto_nr1.a, auto_nr2.a), 0.5);
			}

			ab[i] = auto_ab0 * auto_weights.x + auto_ab1 * auto_weights.y + auto_ab2 * auto_weights.z;
			nr[i] = auto_nr0 * auto_weights.x + auto_nr1 * auto_weights.y + auto_nr2 * auto_weights.z;
			auto_top_weight = v_material_weights[i];
		}
	}
	

	vec3 albedo_side0 = ab[0].rgb;
    vec3 albedo_side1 = ab[1].rgb;
    vec3 albedo_side2 = ab[2].rgb;
    vec3 albedo_side3 = ab[3].rgb;

	vec3 normal_side0 = nr[0].rgb;
	vec3 normal_side1 = nr[1].rgb;
	vec3 normal_side2 = nr[2].rgb;
	vec3 normal_side3 = nr[3].rgb;

	//Top texture sampling
	vec4 ab_t0 = texture_array_triplanar_scaled(u_top_texture_array, world_pos, tr_signs, blending, v_top_indices.x, 1.0);
	vec4 ab_t1 = texture_array_triplanar_scaled(u_top_texture_array, world_pos, tr_signs, blending, v_top_indices.y, 1.0);
	vec4 ab_t2 = texture_array_triplanar_scaled(u_top_texture_array, world_pos, tr_signs, blending, v_top_indices.z, 1.0);

	vec3 albedo_top0 = ab_t0.rgb;
	vec3 albedo_top1 = ab_t1.rgb;
	vec3 albedo_top2 = ab_t2.rgb;

	float height_top0 = ab_t0.a;
	float height_top1 = ab_t1.a;
	float height_top2 = ab_t2.a;
	
	vec4 nr_t0 = texture_array_triplanar_scaled(u_top_normal_array, world_pos, tr_signs, blending, v_top_indices.x, 1.0);
	vec4 nr_t1 = texture_array_triplanar_scaled(u_top_normal_array, world_pos, tr_signs, blending, v_top_indices.y, 1.0);
	vec4 nr_t2 = texture_array_triplanar_scaled(u_top_normal_array, world_pos, tr_signs, blending, v_top_indices.z, 1.0);

	vec3 normal_top0 = nr_t0.rgb;
	vec3 normal_top1 = nr_t1.rgb;
	vec3 normal_top2 = nr_t2.rgb;

	float rough_top0 = nr_t0.a;
	float rough_top1 = nr_t1.a;
	float rough_top2 = nr_t2.a;

    // Get weights and make sure they are normalized.
    // We may add a tiny safety margin so we can afford some degree of error.
    vec4 side_weights = v_material_weights;
	vec3 top_weights = v_top_weights;
	
	if (u_depth_blending_atlas) {
		side_weights = get_depth_blended_weights4(side_weights, vec4(ab[0].a, ab[1].a, ab[2].a, ab[3].a), 0.1);
		top_weights = get_depth_blended_weights3(v_top_weights, vec3(height_top0, height_top1, height_top2), 0.07);
	}


    // Calculate side colors
    vec3 albedo_side =
        albedo_side0 * side_weights.r +
        albedo_side1 * side_weights.g +
        albedo_side2 * side_weights.b +
        albedo_side3 * side_weights.a;

	float height_side =
		ab[0].a * side_weights.r +
		ab[1].a * side_weights.g +
		ab[2].a * side_weights.b +
		ab[3].a * side_weights.a;

	vec3 normal_side =
		normal_side0 * side_weights.r +
        normal_side1 * side_weights.g +
        normal_side2 * side_weights.b +
        normal_side3 * side_weights.a;

	float rough_side =
		nr[0].a * side_weights.r +
		nr[1].a * side_weights.g +
		nr[2].a * side_weights.b +
		nr[3].a * side_weights.a;

	// Now colors for top texture
	vec3 albedo_top =
		albedo_top0 * top_weights.x +
		albedo_top1 * top_weights.y +
		albedo_top2 * top_weights.z;

	float height_top =
		height_top0*top_weights.x +
		height_top1*top_weights.y +
		height_top2*top_weights.z;

	vec3 normal_top =
		normal_top0 * top_weights.x +
		normal_top1 * top_weights.y +
		normal_top2 * top_weights.z;

	float rough_top = 
		rough_top0*top_weights.x +
		rough_top1*top_weights.y +
		rough_top2*top_weights.z;


	// Now blend between top and side textures
	float blend_noise_sm = texture(u_blend_noise2d,world_pos.xz*0.1).r;
	float blend_noise = texture(u_blend_noise2d,world_pos.xz*0.01).r;
	float blend_noise_coarse = texture(u_blend_noise2d,world_pos.xz*0.001).r;

	float topness_midlevel = 0.5+top_normal_affect+top_depth_affect*(height_side-height_top)+top_noise_affect*blend_noise_sm;
	float topness_smoothness = 0.04;
	float topness_min = topness_midlevel - topness_smoothness;
	float topness_max = topness_midlevel + topness_smoothness;
	float topness = smoothstep(topness_min, topness_max, world_normal.y);
	
	// If sum of top weights is < 1.0, it should reveal side texture
	topness *= top_weights.x+top_weights.y+top_weights.z;
	topness *= max(0.0, auto_top_weight);
	
	if (u_depth_blending_layer){
		vec2 top_weight = get_depth_blended_weights2(vec2(topness, 1.0-topness), vec2(height_top, height_side), 0.1);
		ALBEDO = albedo_top * top_weight.x + albedo_side * top_weight.y;
		ROUGHNESS = rough_top * top_weight.x + rough_side * top_weight.y;
		
		NORMAL = (VIEW_MATRIX*vec4(world_normal, 0.0)).xyz;
		NORMAL_MAP = normal_top * top_weight.x + normal_side * top_weight.y;
		NORMAL_MAP_DEPTH = 1.0;
	}
	else{
	    ALBEDO = mix(albedo_side, albedo_top, topness);
		ROUGHNESS = mix(rough_side, rough_top, topness);

		//Previous approach to normal mixing, lifted from HTerrain shader
		//world_normal = normalize(vec3(
	        //world_normal.x + normal_side.x,
	        //world_normal.y,
	        //world_normal.z + normal_side.z
	    //));

		NORMAL = (VIEW_MATRIX*vec4(world_normal, 0.0)).xyz;
		NORMAL_MAP = mix(normal_side, normal_top, topness);
		NORMAL_MAP_DEPTH = 1.0;
	}



	#ifdef DEBUG_MODE_NORMAL
			ALBEDO = ((mat3(INV_VIEW_MATRIX) * NORMAL * vec3(1.0,1.0,1.0)));
			NORMAL = (VIEW_MATRIX*vec4(world_normal, 0.0)).xyz;
			ROUGHNESS = 0.7;
	#endif
	#ifdef DEBUG_MODE_VERTEX

			ALBEDO = COLOR.rgb;
			NORMAL = world_normal;
			ROUGHNESS = 0.7;
	#endif
	#ifdef DEBUG_MODE_TEXTURE

			ALBEDO = COLOR.rgb;
			NORMAL = world_normal;
			ROUGHNESS = 0.7;
	#endif

	// Discard has to be last to workaround https://github.com/godotengine/godot/issues/34966
	if (get_lod_fade_discard(SCREEN_UV)) {
		discard;
	}
}