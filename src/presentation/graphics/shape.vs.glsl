#version 330
// Input attributes
in vec2 position;
in vec2 tex_coords;
in vec4 color;

// Output
out vec2 v_tex_coords;
out vec4 v_color;

// Application data
uniform mat4 matrix;

void main() {
  v_tex_coords = tex_coords;
  v_color = color;
  //v_color = vec3(0.0, 0.0, 0.0);
  gl_Position = matrix * vec4(position, 0.0, 1.0);
}
