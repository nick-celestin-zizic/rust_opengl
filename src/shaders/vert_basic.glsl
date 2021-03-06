#version 330
in vec4 position;
in vec3 normal;
in vec3 texture;
in uint texture_id;

uniform mat4 matrix;

out vec3 v_normal;

void main() {
  v_normal = normal;
  gl_Position = matrix * position;
}
