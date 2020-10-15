#version 330 core

layout (location = 0) in vec3 aPos;
layout (location = 1) in float highlightFactor;
layout (location = 2) in vec3 normal;
layout (location = 3) in float palette_index;

out vec4 colour;

uniform mat4 transMat;

void main() {
    gl_Position = transMat * vec4(aPos.x, aPos.y, aPos.z, 1.0);

    vec3 vert_colour = vec3(0.9, 0.9, 0.9);

    if(palette_index == 0.0)
    {
        vert_colour = vec3(0.3, 0.7, 0.3);
    }
    if(palette_index == 1.0)
    {
        vert_colour = vec3(0.7, 0.3, 0.3);
    }
    if(palette_index == 2.0)
    {
        vert_colour = vec3(0.3, 0.3, 0.7);
    }

    colour = vec4(vert_colour, 1.0);
}
