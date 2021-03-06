#version 330
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_perspective;

in vec4 position;
in vec3 normal;
in vec3 texture;
in uint texture_id;

out vec3 v_position;
out vec3 v_normal;


void main() {
  mat4 view_model = u_view * u_model;
  
  v_normal = transpose(inverse(mat3(view_model))) * normal;
  gl_Position = u_perspective * view_model * position;
  v_position = gl_Position.xyz / gl_Position.w;
}
