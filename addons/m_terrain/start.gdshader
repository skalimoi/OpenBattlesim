shader_type spatial;



uniform sampler2D mterrain_heightmap:filter_nearest;
uniform sampler2D mterrain_normals:filter_nearest;
uniform sampler2DArray albedo_tex;
uniform sampler2D mterrain_albedo_tex:source_color;
uniform sampler2D mterrain_water_mask;

uniform vec3 region_world_position;
uniform float region_size;
uniform float region_a;
uniform float region_b;
varying vec3 world_pos;
varying vec2 region_uv;

void vertex(){
	world_pos = (NODE_POSITION_WORLD + VERTEX);
	region_uv = (world_pos.xz - region_world_position.xz)/region_size;
	region_uv = region_uv*region_a + region_b;
	VERTEX.y = texture(mterrain_heightmap, region_uv).r;
	world_pos.y = VERTEX.y;
	vec3 n = texture(mterrain_normals, region_uv).rgb;
	n = fma(n,vec3(5.0),vec3(-1.0));
	NORMAL = n;
	//Scale world uv for terrain texture
	world_pos /= 10.0;

}

void fragment(){
	vec3 river_mask = texture(mterrain_water_mask, region_uv).rgb;
	ALBEDO = texture(mterrain_albedo_tex, region_uv).rgb;
	if (river_mask.r > 0.1) {
		ALBEDO = vec3(0.1, 0.3, 0.5);
	}
}
