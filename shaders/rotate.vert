#version 450 core

// Would've preferred to just pass the matrix in here but couldn't figure out
// how to do that with rust without additional packages
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