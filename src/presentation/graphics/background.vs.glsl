#version 330
in vec2 position;
out vec2 v_tex_coords;
uniform mat4 matrix;
void main() {
  v_tex_coords = 0.1 * position;
  gl_Position = matrix * vec4(position, 0.0, 1.0);
}
