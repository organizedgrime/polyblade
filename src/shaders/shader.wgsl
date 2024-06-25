struct Uniforms {
    model_mat: mat4x4<f32>,
    view_project_mat: mat4x4<f32>,
    normal_mat: mat4x4<f32>,
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_normal: vec4<f32>,
    @location(2) v_barycentric: vec4<f32>,
    @location(3) v_sides: vec4<f32>,
    @location(4) v_color: vec4<f32>,
};

@vertex
fn vs_main(
    @location(0) position: vec4<f32>,
    @location(1) normal: vec4<f32>,
    @location(2) barycentric: vec4<f32>,
    @location(3) sides: vec4<f32>,
    @location(4) color: vec4<f32>,
) -> Output {
    var output: Output;
    let m_position: vec4<f32> = uniforms.model_mat * position;
    output.v_position = m_position;
    output.v_normal = uniforms.normal_mat * normal;
    output.v_barycentric = barycentric;
    output.v_sides = sides;
    output.v_color = color;

    output.position = uniforms.view_project_mat * m_position;
    return output;
}

struct FragUniforms {
    light_position: vec4<f32>,
    eye_position: vec4<f32>,
};
@binding(1) @group(0) var<uniform> frag_uniforms : FragUniforms;

struct LightUniforms {
    color: vec4<f32>,
    specular_color: vec4<f32>,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
};
@binding(2) @group(0) var<uniform> light_uniforms : LightUniforms;

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
    @location(1) v_normal: vec4<f32>,
    @location(2) v_barycentric: vec4<f32>,
    @location(3) v_sides: vec4<f32>,
    @location(4) v_color: vec4<f32>,
) -> @location(0) vec4<f32> {
    let N: vec3<f32> = normalize(v_normal.xyz);
    let L: vec3<f32> = normalize(frag_uniforms.light_position.xyz - v_position.xyz);
    let V: vec3<f32> = normalize(frag_uniforms.eye_position.xyz - v_position.xyz);
    let H: vec3<f32> = normalize(L + V);
    let diffuse: f32 = light_uniforms.diffuse_intensity * max(dot(N, L), 0.0);
    let specular: f32 = light_uniforms.specular_intensity * pow(max(dot(N, H), 0.0), light_uniforms.specular_shininess);
    let ambient: f32 = light_uniforms.ambient_intensity;
    let reflection_color = light_uniforms.color * (ambient + diffuse) + light_uniforms.specular_color * specular;
    //let lit_color = normalize(v_color + reflection_color * 0.2);
    let edge_color = edge_factor(v_barycentric.xyz, v_sides.xyz);
    return vec4(min(edge_color, v_color.xyz), v_color.w);
}
