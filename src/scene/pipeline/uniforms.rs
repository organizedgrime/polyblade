use crate::scene::Camera;

use iced::{Color, Rectangle};

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    pub(crate) model_mat: glam::Mat4,
    pub(crate) view_projection_mat: glam::Mat4,
    pub(crate) normal_mat: glam::Mat4,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct FragUniforms {
    pub(crate) light_position: glam::Vec4,
    pub(crate) eye_position: glam::Vec4,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct LightUniforms {
    color: glam::Vec4,
    specular_color: glam::Vec4,
    ambient_intensity: f32,
    diffuse_intensity: f32,
    specular_intensity: f32,
    specular_shininess: f32,
}

impl FragUniforms {
    pub fn new() -> Self {
        Self {
            light_position: todo!(),
            eye_position: todo!(),
        }
    }
}

impl LightUniforms {
    pub fn new(light_color: Color, specular_color: Color) -> Self {
        Self {
            color: glam::Vec4::from(light_color.into_linear()),
            specular_color: glam::Vec4::from(light_color.into_linear()),
            ambient_intensity: 0.1,
            diffuse_intensity: 0.6,
            specular_intensity: 0.3,
            specular_shininess: 30.0,
        }
    }
}
