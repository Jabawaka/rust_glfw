#version 330 core

layout (location = 0) in vec3 aPos;

uniform mat4 transMat;

void main() {
    gl_Position = transMat * vec4(aPos.x, aPos.y, aPos.z, 1.0);
}