#version 450 core

in layout(location=0) vec4 inColor;
out vec4 color;

void main() {
    color = inColor;
}