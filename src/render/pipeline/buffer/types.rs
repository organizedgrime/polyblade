use ultraviolet::{Mat4, Vec3, Vec4};

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub position: Vec3,
    pub color: Vec4,
}

impl Vertex {
    pub fn new(position: Vec3, color: Vec4) -> Self {
        Self { position, color }
    }
}

/* #[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ShapeVertex {
    pub barycentric: Vec4,
    pub sides: Vec4,
} */

pub struct AllUniforms {
    pub model: ModelUniforms,
    pub frag: FragUniforms,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ModelUniforms {
    pub(crate) model_mat: Mat4,
    pub(crate) view_projection_mat: Mat4,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct FragUniforms {
    pub(crate) line_thickness: f32,
}
