struct Uniforms {
    model_mat: mat4x4<f32>,
    view_project_mat: mat4x4<f32>,
};
@binding(0) @group(0) var<uniform> uniforms : Uniforms;

struct Output {
    @builtin(position) position: vec4<f32>,
    @location(0) v_position: vec4<f32>,
    @location(1) v_color: vec4<f32>,
    @location(2) v_barycentric: vec4<f32>,
    // @location(3) v_sides: vec4<f32>,
};

@vertex
fn vs_main(
    @builtin(vertex_index) v_index: u32,
    @location(0) position: vec4<f32>,
    @location(1) color: vec4<f32>,
    // @location(2) barycentric: vec4<f32>,
    // @location(3) sides: vec4<f32>,
) -> Output {
    var output: Output;
    let m_position: vec4<f32> = uniforms.model_mat * position;

    output.v_position = m_position;
    output.v_color = color;

    if v_index % 3 == 0 {
        output.v_barycentric = vec4(1.0, 0.0, 0.0, 0.0);
    } else if v_index % 3 == 1 {
        output.v_barycentric = vec4(0.0, 1.0, 0.0, 0.0);
    } else {
        output.v_barycentric = vec4(0.0, 1.0, 1.0, 0.0);
    }

    // output.v_sides = sides;

    output.position = uniforms.view_project_mat * m_position;
    return output;
}

struct FragUniforms {
    line_thickness: f32
};
@binding(1) @group(0) var<uniform> frag_uniforms : FragUniforms;

fn edge_factor(v_barycentric: vec3<f32>, v_sides: vec3<f32>) -> f32 {
    let face: vec3<f32> = v_barycentric * v_sides;
    let r: vec3<f32> = fwidthFine(face) * frag_uniforms.line_thickness;
    let f: vec3<f32> = step(r, face);
    return min(min(f.x, f.y), f.z);
}

@fragment
fn fs_main(
    @location(0) v_position: vec4<f32>,
    @location(1) v_color: vec4<f32>,
    @location(2) v_barycentric: vec4<f32>,
    // @location(3) v_sides: vec4<f32>,
) -> @location(0) vec4<f32> {
    let edge_color = edge_factor(v_barycentric.xyz, vec3(1.0, 1.0, 1.0));

    if edge_color == 0.0 {
        return vec4(0.0, 0.0, 0.0, 1.0);
    } else {
        return v_color;
    }
}
