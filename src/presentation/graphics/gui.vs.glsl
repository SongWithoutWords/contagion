#version 330
in vec4 color;
in vec2 position;
out vec4 v_color;
void main() {
    v_color = color;
    gl_Position = vec4(position, 0.0, 1.0);
}