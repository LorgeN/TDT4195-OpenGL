#version 450 core

layout(location=2) uniform mat4 rotation; 
in layout(location=0) vec3 position;

void main() {
    gl_Position = vec4(position, 1.0f) * rotation;
}