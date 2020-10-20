#version 330 core

in vec4 colour;
in vec3 vert_normal;

out vec4 fragColour;

void main() {
    float intensity = max(dot(vert_normal, normalize(vec3(1.0, 0.0, 1.0))), 0);

    /*if(intensity < 0.35)
    {
        intensity = 0.35;
    }
    else
    {
        intensity = 0.9;
    }*/

    intensity += 0.2;

    fragColour = intensity * colour;
}