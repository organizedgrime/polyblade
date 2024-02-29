in vec3 position;
in vec3 barycentric;
in vec3 edge;
in vec4 color;
uniform mat4 model;
uniform mat4 projection;
out vec4 v_color;
varying vec3 vbc;
varying vec3 edj;

void main() {
	vbc = barycentric;
	edj = edge;
    gl_Position = projection * model * vec4(position, 1.0);
    v_color = color;
}
