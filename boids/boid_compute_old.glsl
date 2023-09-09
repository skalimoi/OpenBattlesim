#[compute]
#version 450

layout(local_size_x = 100, local_size_y = 1, local_size_z = 1) in;

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

layout(set = 0, binding = 3, std430) restrict buffer delta {
    float delta;
}
Delta;

void main() {

    // We calculate the steering factor for every behavior and then add them to the velocity of the boid.
    // align

    float min_velocity = 10;
    float max_velocity = 100;
    float distance_limit = 100;

    float desired_separation = 25.0;
    vec3 pos = PositionBuffer.data[gl_GlobalInvocationID.x];
    
    vec3 sep_vec = vec3(0,0,0);
    vec3 midpoint = vec3(0,0,0);
    vec3 average_velocity = vec3(0,0,0);
    int total_1 = 0;

    for (int b = 0; b < BoidsNumber.current_boids; b++) {
         if (b != gl_GlobalInvocationID.x) {
        float distance_1 = distance(pos, PositionBuffer.data[b]);
        if (distance_1 < distance_limit) {
            average_velocity += VelocityBuffer.data[b];
            midpoint += PositionBuffer.data[b];
            total_1 +=1;
        }
        if (total_1 > 0) {
            average_velocity /= total_1;
            VelocityBuffer.data[gl_GlobalInvocationID.x] += normalize(average_velocity - VelocityBuffer.data[gl_GlobalInvocationID.x]) * 1;
            midpoint /= total_1;
            VelocityBuffer.data[gl_GlobalInvocationID.x] += normalize(midpoint - pos) * 1;
        }
        if (length(VelocityBuffer.data[gl_GlobalInvocationID.x]) < min_velocity) {
            VelocityBuffer.data[gl_GlobalInvocationID.x] = normalize(VelocityBuffer.data[gl_GlobalInvocationID.x]) * min_velocity;
        }
        if (length(VelocityBuffer.data[gl_GlobalInvocationID.x]) > max_velocity) {
            VelocityBuffer.data[gl_GlobalInvocationID.x] = normalize(VelocityBuffer.data[gl_GlobalInvocationID.x]) * min_velocity;
        }
        PositionBuffer.data[gl_GlobalInvocationID.x] += VelocityBuffer.data[gl_GlobalInvocationID.x] * Delta.delta;
      }
    }
}