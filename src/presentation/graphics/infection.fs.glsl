#version 330
in vec4 v_color;
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
void main() {
    color = texture(tex, v_tex_coords) * v_color;
}