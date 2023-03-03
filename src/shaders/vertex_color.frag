#version 300 es

precision highp float;
in vec3 v_color;
in float v_alpha;
out vec4 fragColor;

void main() {
    fragColor = vec4(mix(vec3(0.0, 0.0, 0.0), v_color, v_alpha), 1);
}
