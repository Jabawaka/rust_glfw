#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in float highlightFactor;
layout (location = 2) in vec3 normal;
layout (location = 3) in int palette_index;

out vec4 colour;

uniform mat4 transMat;

void main() {
    gl_Position = transMat * vec4(aPos.x, aPos.y, aPos.z, 1.0);

    vec3 vert_colour;

    if(palette_index == 0)
    {
        vert_colour = vec3(1.0, 0.5, 0.0);
    }
    if(palette_index == 1)
    {
        vert_colour = vec3(0.0, 0.5, 1.0);
    }

    colour = vec4(vert_colour, 1.0);
}
