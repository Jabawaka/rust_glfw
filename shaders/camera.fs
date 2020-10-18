#version 330

// Interpolated values from the vertex shaders
in vec2 UV;

// Ouput data
out vec4 colour;

// Values that stay constant for the whole mesh
uniform sampler2D textureSampler;

void main()
{
    colour = texture(textureSampler, UV);
}