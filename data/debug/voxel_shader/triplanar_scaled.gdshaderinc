
//This is an updated triplanar function, that produces non-mirrored UVs in negative coords
vec4 texture_array_triplanar_scaled(sampler2DArray tex, vec3 world_pos, vec3 signs, vec3 blend, float i, float scale) {
	vec3 tr_world_pos = world_pos * signs;//Corrected coords for flipped sides
	vec4 xaxis = texture(tex, vec3(tr_world_pos.zy*scale, i));
    vec4 yaxis = texture(tex, vec3(-world_pos.xz*scale, i));
    vec4 zaxis = texture(tex, vec3(tr_world_pos.xy*scale, i));

    return xaxis * blend.x + yaxis * blend.y + zaxis * blend.z;
}