#version 330
in vec3 v_normal;
uniform mat4 matrix;
out vec4 color;

void main() {
  color = vec4(v_normal, 1.0);
}
