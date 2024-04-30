mod camera;
mod pipeline;

use camera::Camera;
use pipeline::Pipeline;

use crate::wgpu;
use pipeline::cube::{self, Cube};

use iced::mouse;
use iced::time::Duration;
use iced::widget::shader;
use iced::{Color, Rectangle, Size};

use glam::Vec3;
use rand::Rng;
use std::cmp::Ordering;
use std::iter;

pub const MAX: u32 = 500;

#[derive(Clone)]
pub struct Scene {
    pub size: f32,
    pub cube: Cube,
    pub camera: Camera,
    pub light_color: Color,
}

impl Scene {
    pub fn new() -> Self {
        let mut scene = Self {
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
        bounds: Rectangle,
    ) -> Self::Primitive {
        Primitive::new(&self.cube, &self.camera, bounds, self.light_color)
    }
}

/// A collection of `Cube`s that can be rendered.
#[derive(Debug)]
pub struct Primitive {
    cube: cube::Raw,
    uniforms: pipeline::Uniforms,
}

impl Primitive {
    pub fn new(cube: &Cube, camera: &Camera, bounds: Rectangle, light_color: Color) -> Self {
        let uniforms = pipeline::Uniforms::default();

        Self {
            cube: cube::Raw::from_cube(cube),
            uniforms,
        }
    }
}

impl shader::Primitive for Primitive {
    fn prepare(
        &self,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        _bounds: Rectangle,
        target_size: Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(device, queue, format, target_size));
        }

        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        //upload data to GPU
        pipeline.update(device, queue, target_size, &self.uniforms, &self.cube);
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

fn rnd_origin() -> Vec3 {
    Vec3::new(
        rand::thread_rng().gen_range(-4.0..4.0),
        rand::thread_rng().gen_range(-4.0..4.0),
        rand::thread_rng().gen_range(-4.0..2.0),
    )
}
