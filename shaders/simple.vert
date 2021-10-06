#version 450 core

in layout(location=0) vec3 inPosition;
in layout(location=1) vec4 inColor;
in layout(location=2) vec3 inNormal;
layout(location=3) uniform mat4 inTotalTransform; 
layout(location=4) uniform mat4 inModelTransform;
out layout(location=0) vec4 outColor;
out layout(location=1) vec3 outNormal;

void main() {
    gl_Position = inTotalTransform * vec4(inPosition, 1.0);
    outColor = inColor;
    outNormal = normalize(mat3(inModelTransform) * inNormal);
}