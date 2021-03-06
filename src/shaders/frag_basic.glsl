#version 330
uniform vec3 u_color;
uniform vec3 u_light;

in vec3 v_position;
in vec3 v_normal;

out vec4 color;

const vec3 specular_color = vec3(1.0, 1.0, 1.0);

void main() {
  vec3 ambient_color = u_color * 0.2;
  vec3 diffuse_color = u_color * 0.6;


  vec3 camera_dir = normalize(-v_position);
  vec3 half_direction = normalize(normalize(u_light) + camera_dir);

  float diffuse  = max(dot(normalize(v_normal), normalize(u_light)), 0.0);  
  float specular = pow(max(dot(half_direction, normalize(v_normal)),
                           0.0), 16.0);

  color = vec4(ambient_color +
               diffuse  * diffuse_color +
               specular * specular_color, 1.0);
}
