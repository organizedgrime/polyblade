use crate::scene::Camera;

use iced::{Color, Rectangle};

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Uniforms {
    model_mat: glam::Mat4,
    view_projection_mat: glam::Vec4,
    normal_mat: glam::Vec4,
}

#[derive(Copy, Default, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct FragUniforms {
    light_position: glam::Vec4,
    eye_position: glam::Vec4,
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

impl Uniforms {
    pub fn new(camera: &Camera) -> Self {
        //let camera_proj = camera.build_view_proj_matrix(bounds);
        Self {
            model_mat: todo!(),
            view_projection_mat: todo!(),
            normal_mat: todo!(),
        }
    }
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
    pub fn new(camera: &Camera, bounds: Rectangle, light_color: Color) -> Self {
        Self {
            color: glam::Vec4::from(light_color.into_linear()),
            specular_color: todo!(),
            ambient_intensity: todo!(),
            diffuse_intensity: todo!(),
            specular_intensity: todo!(),
            specular_shininess: todo!(),
        }
    }
}
