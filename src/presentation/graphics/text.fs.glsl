#version 330

in vec2 v_tex_coords;
out vec4 f_color;
 uniform vec4 color;
 uniform sampler2D tex;
 void main() {
     vec4 c = vec4(color.rgb, color.a * texture(tex, v_tex_coords));
     if (c.a <= 0.01) {
         discard;
     } else {
         f_color = c;
     }
 }