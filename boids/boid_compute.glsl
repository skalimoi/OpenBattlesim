// empty shader file
#[compute]
#version 450

layout(local_size_x = 2, local_size_y = 1, local_size_z = 1) in;

layout(set = 0, binding = 0, std430) restrict buffer velocity_buffer {
    vec3 data[];
}
VelocityBuffer;

layout(set = 0, binding = 1, std430) restrict buffer position_buffer {
    vec3 data[];
}
PositionBuffer;

layout(set = 0, binding = 2, std430) restrict buffer boids_num {
    float current_boids;
}
BoidsNumber;

void main() {

	
    // align
    float distance_limit_1 = 500.0;
    vec3 steering_1 = vec3(0.0,0.0,0.0);
    int total_1 = 0;
    vec3 pos_1 = PositionBuffer.data[gl_GlobalInvocationID.x];

    for (int b = 0; b < BoidsNumber.current_boids; b++) {
         if (b != gl_GlobalInvocationID.x) {
        float distance_1 = distance(pos_1, PositionBuffer.data[b]);
        if (distance_1 < distance_limit_1) {
            steering_1 += VelocityBuffer.data[b];
            total_1 +=1;
        }
        if (total_1 > 0) {
            steering_1 /= vec3(total_1, total_1, total_1);
            steering_1 -= VelocityBuffer.data[gl_GlobalInvocationID.x];
        }
      }
    }
    // separate
    float distance_limit_2 = 100.0;
    vec3 steering_2 = vec3(0.0,0.0,0.0);
    int total_2 = 0;
    vec3 pos_2 = PositionBuffer.data[gl_GlobalInvocationID.x];

    for (int b = 0; b < BoidsNumber.current_boids; b++) {
         if (b != gl_GlobalInvocationID.x) {
        float distance_2 = distance(pos_2, PositionBuffer.data[b]);
        if (distance_2 < distance_limit_2) {
            vec3 difference = PositionBuffer.data[gl_GlobalInvocationID.x] - PositionBuffer.data[b];
            difference /= distance_2 * distance_2;
            steering_2 += difference;
            total_2 +=1;
        }
        if (total_2 > 0) {
            steering_2 /= vec3(total_2, total_2, total_2);
            steering_2 -= PositionBuffer.data[gl_GlobalInvocationID.x];
            steering_2 -= VelocityBuffer.data[gl_GlobalInvocationID.x];
        }
      }
    }
    //cohesion
    float distance_limit_3 = 500.0;
    vec3 steering_3 = vec3(0.0,0.0,0.0);
    int total_3 = 0;
    vec3 pos_3 = PositionBuffer.data[gl_GlobalInvocationID.x];

    for (int b = 0; b < BoidsNumber.current_boids; b++) {
         if (b != gl_GlobalInvocationID.x) {
        float distance_3 = distance(pos_3, PositionBuffer.data[b]);
        if (distance_3 < distance_limit_2) {
            steering_3 += PositionBuffer.data[b];
            total_3 +=1;
        }
        if (total_3 > 0) {
            steering_3 /= vec3(total_3, total_3, total_3);
            steering_3 -= PositionBuffer.data[gl_GlobalInvocationID.x];
            steering_3 -= VelocityBuffer.data[gl_GlobalInvocationID.x];
        }
      }
    }
    vec3 new = VelocityBuffer.data[gl_GlobalInvocationID.x] + steering_1 + steering_2 + steering_3;
    VelocityBuffer.data[gl_GlobalInvocationID.x] = new;
}