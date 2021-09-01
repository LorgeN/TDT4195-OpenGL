#version 450 core

layout(location=2) uniform vec2 rotation; 
in layout(location=0) vec3 position;

void main() {
    gl_Position = vec4(position, 1.0f) * mat4(
        rotation.y, -rotation.x, 0, 0,
        rotation.x, rotation.y, 0, 0,
        0, 0, 1, 0,
        0, 0, 0, 1
    );
}