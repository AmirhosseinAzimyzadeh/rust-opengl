#version 140
in vec3 position;
in vec3 normal;
out vec3 v_normal;
uniform mat4 matrix;
uniform mat4 perspective;
uniform mat4 transform;
void main() {
  v_normal = transpose(inverse(mat3(matrix))) * normal;
  gl_Position =  perspective * matrix * transform * vec4(position, 1.0);
}