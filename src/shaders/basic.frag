in vec4 v_color;
out vec4 fragColor;
in vec3 vbc;

const float lineWidth = 2.5;

vec3 srgb_from_linear_srgb(vec3 rgb) {
    vec3 a = vec3(0.055, 0.055, 0.055);
    vec3 ap1 = vec3(1.0, 1.0, 1.0) + a;
    vec3 g = vec3(2.4, 2.4, 2.4);
    vec3 ginv = 1.0 / g;
    vec3 select = step(vec3(0.0031308, 0.0031308, 0.0031308), rgb);
    vec3 lo = rgb * 12.92;
    vec3 hi = ap1 * pow(rgb, ginv) - a;
    return mix(lo, hi, select);
}

float edgeFactor() {
	vec3 face = vbc * vec3(0.0, 1.0, 0.0);
	vec3 r = fwidth(face) * lineWidth;
	vec3 f = step(r, face);
	return min(min(f.x, f.y), f.z);
}

void main() {
    fragColor = v_color;
	fragColor.rgb = min(vec3(edgeFactor()), srgb_from_linear_srgb(fragColor.rgb));
}
