#version 450 core

out vec4 color;

void main() {
    float flag = float(mod(((floor(gl_FragCoord.x / 25) + floor(gl_FragCoord.y / 25))), 2.0));
    color = vec4(flag, flag, flag, 1.0f);
}