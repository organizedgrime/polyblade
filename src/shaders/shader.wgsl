struct Uniforms {
    model: mat4x4<f32>,
	view_projection: mat4x4<f32>,
    normal: mat4x4<f32>,
};
@binding(0) @group(1) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_barycentric: vec4<f32>,
    @location(2) v_sides: vec4<f32>,
    @location(3) v_color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) barycentric: vec4<f32>,
    @location(2) sides: vec4<f32>,
    @location(3) color: vec4<f32>,
) -> Output {
    var output: Output;
    let m_position: vec4<f32> = uniforms.model * position;
    output.v_position = m_position;
    output.v_barycentric = barycentric;
    output.v_sides = sides;
    output.v_color = color;

    output.position = uniforms.view_projection * position;
    return output;
}

fn edge_factor(v_barycentric: vec3<f32>, v_sides: vec3<f32>) -> vec3<f32> {
    let line_width = 2.0;
    let face: vec3<f32> = v_barycentric * v_sides;
    let r: vec3<f32> = fwidthFine(face) * line_width;
    let f: vec3<f32> = step(r, face);
    return vec3(min(min(f.x, f.y), f.z));
}

@fragment
fn fs_main(
    @location(0) v_position: vec4<f32>,
    @location(1) v_barycentric: vec4<f32>,
    @location(2) v_sides: vec4<f32>,
    @location(3) v_color: vec4<f32>,
) -> @location(0) vec4<f32> {
    // let N: vec3<f32> = normalize(v_position.xyz);
    // let L: vec3<f32> = normalize(frag_uniforms.light_position.xyz - v_position.xyz);
    // let V: vec3<f32> = normalize(frag_uniforms.eye_position.xyz - v_position.xyz);
    // let H: vec3<f32> = normalize(L + V);
    // let diffuse: f32 = light_uniforms.diffuse_intensity * max(dot(N, L), 0.0);
    // let specular: f32 = light_uniforms.specular_intensity * pow(max(dot(N, H), 0.0), light_uniforms.specular_shininess);
    // let ambient: f32 = light_uniforms.ambient_intensity;
    // let reflection_color = light_uniforms.color * (ambient + diffuse) + light_uniforms.specular_color * specular;
    // let lit_color = normalize(v_color.xyz + reflection_color.xyz * 0.2);
    // let edge_color = edge_factor(v_barycentric.xyz, v_sides.xyz);
    // return vec4(min(edge_color, lit_color), v_color.w);
    let edge_color = edge_factor(v_barycentric.xyz, v_sides.xyz);
    return vec4(min(edge_color, v_color.xyz), v_color.w);
}
