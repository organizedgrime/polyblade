in vec3 xyz;
in vec3 rgb;
in vec3 bsc;
in vec3 tri;
out vec3 v_Rgb;
out vec3 v_Bsc;
out vec3 v_Tri;
uniform mat4 model;
uniform mat4 projection;

void main() {
    gl_Position = projection * model * vec4(xyz, 1.0);
    v_Rgb = rgb;
    v_Bsc = bsc;
    v_Tri = tri;
}
