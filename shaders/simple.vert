#version 450 core

in layout(location=0) vec3 position;
in layout(location=1) vec4 color;
layout(location=2) uniform mat4 transformation;
out layout(location=0) vec4 outColor;

void main() {
    gl_Position = transformation * vec4(position, 1.0);
    outColor = color;
}