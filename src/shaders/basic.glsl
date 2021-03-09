#shader_vertex
#version 330
uniform mat4 u_model;
uniform mat4 u_view;
uniform mat4 u_perspective;

in vec4 position;
in vec3 normal;
in vec3 uvw;

out vec3 v_position;
out vec3 v_normal;
out vec2 v_uv;

void main() {
  mat4 view_model = u_view * u_model;
  gl_Position = u_perspective * view_model * position;
  
  v_uv       = vec2(uvw);
  v_normal   = transpose(inverse(mat3(view_model))) * normal;
  v_position = gl_Position.xyz / gl_Position.w;
  
}

#shader_fragment
#version 330
uniform vec3      u_light;
uniform sampler2D u_texture;

in vec3 v_position;
in vec3 v_normal;
in vec2 v_uv;

out vec4 color;

const vec3 specular_color = vec3(1.0, 1.0, 1.0);

mat3 cotangent_frame(vec3 normal, vec3 pos, vec2 uv) {
  vec3 dp1 = dFdx(pos);
  vec3 dp2 = dFdy(pos);

  vec2 duv1 = dFdx(uv);
  vec2 duv2 = dFdy(uv);

  vec3 dp1perp = cross(normal, dp1);
  vec3 dp2perp = cross(dp2, normal);

  vec3 T = dp2perp *  duv1.x + dp1perp * duv2.x;
  vec3 B = dp2perp * duv1.y + dp1perp * duv2.y;

  float invmax = inversesqrt(max(dot(T, T), dot(B, B)));

  return mat3(T * invmax, B * invmax, normal);
}

void main() {
  vec3 diffuse_color = texture(u_texture, v_uv).rgb;
  vec3 ambient_color = diffuse_color * 0.1;

  vec3 camera_dir = normalize(-v_position);
  vec3 half_direction = normalize(normalize(u_light) + camera_dir);

  float diffuse  = max(dot(normalize(v_normal), normalize(u_light)), 0.0);  
  float specular = pow(max(dot(half_direction, normalize(v_normal)),
                           0.0), 16.0);

  color = vec4(ambient_color +
               diffuse  * diffuse_color +
               specular * specular_color, 1.0);
}
