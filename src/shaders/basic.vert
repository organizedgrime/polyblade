in vec3 position;
in vec3 barycentric;
in vec3 edge;
in vec4 color;
uniform mat4 model;
uniform mat4 projection;
out vec4 v_color;
out vec3 vbc;

void main() {
    vbc = barycentric;
    gl_Position = projection * model * vec4(position, 1.0);
    v_color = color;
}
