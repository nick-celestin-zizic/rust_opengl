#shader_vertex
#version 330
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_projection;

in vec4 position;
in vec4 color;
in vec3 normal;
in vec3 uv;

out vec4 v_position;
out vec4 v_color;

void main() {
  gl_Position = (u_projection * u_view * u_model) * position;
  
  v_color = color;
  v_position = position;
}

#shader_fragment
#version 330
uniform vec3      u_light;
uniform sampler2D u_texture;

in vec4 v_position;
in vec4 v_color;

out vec4 color;

void main() {
  color = v_color;
}
