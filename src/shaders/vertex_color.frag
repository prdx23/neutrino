#version 300 es

precision highp float;
in vec3 v_color;
in float v_alpha;
in float v_collide;
out vec4 fragColor;

void main() {
    vec3 color = mix(v_color, vec3(1.0, 0.0, 0.0), v_collide);
    fragColor = vec4(mix(vec3(0.0, 0.0, 0.0), color, v_alpha), 1);
}
