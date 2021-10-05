#version 450 core

in layout(location=0) vec4 inColor;
in layout(location=1) vec3 inNormal;
out vec4 color;

void main() {
    vec3 lightDirection = normalize(vec3(0.8, -0.5, 0.6));
    color = vec4(inColor.rgb * max(0, dot(inNormal, -lightDirection)), inColor.a);
}