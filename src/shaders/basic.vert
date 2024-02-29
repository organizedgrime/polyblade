in vec3 position;
in vec4 color;
uniform mat4 model;
uniform mat4 projection;
out vec4 v_color;

void main() {
    gl_Position = projection * model * vec4(position, 1.0);
    v_color = color;
}
