#version 330
in vec2 v_tex_coords;
out vec4 color;
uniform sampler2D tex;
void main() {
  vec4 texel = texture(tex, v_tex_coords);
  color = vec4(0.0, 0.0, 0.0, 0.5 * texel.w);
}
