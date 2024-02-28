precision mediump float;
uniform mat4 projection;
attribute vec3 position;
attribute vec2 barycentric;
varying vec2 b;
void main () {
	b = barycentric;
	gl_Position = projection * vec4(position, 1);
}
