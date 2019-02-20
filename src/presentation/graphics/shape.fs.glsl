#version 330

// From vertex shader
in vec2 v_tex_coords;
in vec3 v_color;

// Output
out vec4 color;

void main() {
  color = vec4(v_color, 1.0);
}
