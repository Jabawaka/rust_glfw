#version 330

// Input vertex data
layout (location = 0) in vec4 vertex; // vec2 position, vec2 uv

// Output data
out vec2 UV;

// Layer at which to render the camera, the higher the more in the back
uniform int layer;


void main()
{
    float zPos = (layer - 200) / 100;

    // Output position of the vertex
    gl_Position = vec4(vertex.xy, zPos, 1.0);

    // Color of each vertex to be interpolated
    UV = vertex.zw;
}