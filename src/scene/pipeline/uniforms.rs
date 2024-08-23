use iced::Color;
use ultraviolet::{Mat4, Vec4};

pub struct AllUniforms {
    pub model: ModelUniforms,
    pub frag: FragUniforms,
    pub light: LightUniforms,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct ModelUniforms {
    pub(crate) model_mat: Mat4,
    pub(crate) view_projection_mat: Mat4,
    pub(crate) normal_mat: Mat4,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct FragUniforms {
    pub(crate) light_position: Vec4,
    pub(crate) eye_position: Vec4,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct LightUniforms {
    color: Vec4,
    specular_color: Vec4,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

impl LightUniforms {
    pub fn new(light_color: Color, specular_color: Color) -> Self {
        Self {
            color: Vec4::from(light_color.into_linear()),
            specular_color: Vec4::from(specular_color.into_linear()),
            ambient_intensity: 0.1,
            diffuse_intensity: 0.6,
            specular_intensity: 0.3,
            specular_shininess: 30.0,
        }
    }
}
