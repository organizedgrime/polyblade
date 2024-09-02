use iced::Color;
use ultraviolet::{Mat4, Vec3, Vec4};

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct MomentVertex {
    pub position: Vec3,
    pub color: Vec4,
}

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ShapeVertex {
    pub normal: Vec4,
    pub barycentric: Vec4,
    pub sides: Vec4,
}

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
    pub(crate) light_position: Vec4,
    pub(crate) eye_position: Vec4,
}
