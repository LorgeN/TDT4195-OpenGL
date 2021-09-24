#version 450 core

layout(location=3) uniform vec3 hsv;
out vec4 color;

// Source: https://stackoverflow.com/questions/15095909/from-rgb-to-hsv-in-opengl-glsl
vec3 hsv2rgb(vec3 c) {
    vec4 K = vec4(1.0, 2.0 / 3.0, 1.0 / 3.0, 3.0);
    vec3 p = abs(fract(c.xxx + K.xyz) * 6.0 - K.www);
    return c.z * mix(K.xxx, clamp(p - K.xxx, 0.0, 1.0), c.y);
}

void main() {
    color = vec4(hsv2rgb(vec3(mod(hsv.x + (gl_FragCoord.x / 1000 + gl_FragCoord.y / 1000) / 2.0, 1.0), hsv.y, hsv.z)), 1.0f);
}