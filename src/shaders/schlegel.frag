#extension GL_OES_standard_derivatives : enable
precision mediump float;
#pragma glslify: grid = require(glsl-solid-wireframe/barycentric/scaled)
varying vec2 b;
void main () {
	gl_FragColor = vec4(vec3(grid(b, 1.0)), 1);
}
