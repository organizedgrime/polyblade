mod camera;
mod pipeline;

use bytemuck::Zeroable;
use camera::Camera;
use pipeline::Pipeline;

use crate::wgpu;
use pipeline::cube::{self, Cube};

use iced::mouse;
use iced::time::Duration;
use iced::widget::shader;
use iced::{Color, Rectangle, Size};

use glam::{vec3, vec4, Mat4, Vec3, Vec4};
use rand::Rng;
use std::cmp::Ordering;
use std::f32::consts::PI;
use std::iter;
use std::ops::Sub;
use std::time::Instant;

pub const MAX: u32 = 500;

#[derive(Clone)]
pub struct Scene {
    pub start: Instant,
    pub size: f32,
    pub cube: Cube,
    pub camera: Camera,
    pub light_color: Color,
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Self {
            start: Instant::now(),
            size: 0.2,
            cube: Cube::default(),
            camera: Camera::default(),
            light_color: Color::WHITE,
        };

        scene
    }

    pub fn update(&mut self, time: Duration) {
        self.cube.update(self.size, time.as_secs_f32());
    }
}

impl<Message> shader::Program<Message> for Scene {
    type State = ();
    type Primitive = Primitive;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        Primitive::new(self.start, &self.cube, &self.camera)
    }
}

/// A collection of `Cube`s that can be rendered.
#[derive(Debug)]
pub struct Primitive {
    cube: cube::Raw,
    camera: Camera,
    start: Instant,
}

impl Primitive {
    pub fn new(start: Instant, cube: &Cube, camera: &Camera) -> Self {
        Self {
            cube: cube::Raw::from_cube(cube),
            camera: *camera,
            start,
        }
    }
}

impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: Rectangle,
        target_size: Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(device, queue, format, target_size));
        }

        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        //self.uniforms.model_mat = ;
        let dt = Instant::now() - self.start;

        // update uniform buffer
        let dt = 2.0 * dt.as_secs_f32();
        let model_mat = Mat4::from_rotation_x(dt / PI) * Mat4::from_rotation_y(dt / PI * 1.1);
        let view_projection_mat = self.camera.build_view_proj_mat(bounds);
        let normal_mat = (model_mat.inverse()).transpose();

        let uniforms = pipeline::Uniforms {
            model_mat,
            view_projection_mat,
            normal_mat,
        };

        let frag_uniforms = pipeline::FragUniforms {
            light_position: self.camera.position(),
            eye_position: self.camera.position() + vec4(1.0, 1.0, 1.0, 0.0),
        };

        //upload data to GPU
        pipeline.update(
            device,
            queue,
            target_size,
            &uniforms,
            &frag_uniforms,
            &self.cube,
        );
    }

    fn render(
        &self,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        _target_size: Size<u32>,
        viewport: Rectangle<u32>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        //at this point our pipeline should always be initialized
        let pipeline = storage.get::<Pipeline>().unwrap();

        //render primitive
        pipeline.render(target, encoder, viewport);
    }
}
