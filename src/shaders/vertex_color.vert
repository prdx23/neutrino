#version 300 es

in vec4 a_position;
in vec3 a_color;
out vec3 v_color;
out float v_alpha;
out float v_collide;

uniform objectData {
    mat4 u_matrix;
    float u_collide;
};

void main() {
    gl_Position = u_matrix * a_position;
    v_color = a_color;
    v_collide = u_collide;
    v_alpha = 1.0 - (gl_Position.z / (800.0 * 2.0));
}
