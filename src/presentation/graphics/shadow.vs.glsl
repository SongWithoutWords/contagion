#version 330
in vec2 position;
in vec2 tex_coords;
out vec2 v_tex_coords;
uniform float height; // height from which the shadow is cast
uniform mat4 matrix;
void main() {
  v_tex_coords = tex_coords;
  gl_Position = matrix * vec4(position + height * vec2(-0.25, 0.25), 0.0, 1.0);
}
