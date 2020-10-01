#version 330 core

layout (location = 0) in vec3 vert_pos;

uniform mat4 model_mat;
uniform mat4 cam_mat;

void main() {
    gl_Position = cam_mat * model_mat * vec4(vert_pos.x, vert_pos.y, vert_pos.z, 1.0);
}
