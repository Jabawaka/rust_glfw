#version 330 core

layout (location = 0) in vec3 vert_pos;
layout (location = 1) in float highlightFactor;

out vec4 colour;

uniform mat4 transMat;

void main() {
    gl_Position = transMat * vec4(vert_pos.x, vert_pos.y, vert_pos.z, 1.0);

    if(highlightFactor > 0.0)
    {
        colour = vec4(0.7, 0.7, 0.0, 1.0);
    }
    else
    {
        colour = vec4(0.0, 0.5, 0.0, 1.0);
    }
}
