mod camera;
mod pipeline;


use camera::Camera;
use pipeline::Pipeline;

use crate::polyhedra::PolyGraph;
use crate::wgpu;

use iced::mouse;
use iced::time::Duration;
use iced::widget::shader;
use iced::{Color, Rectangle, Size};

use glam::{vec4, Mat4};


use std::f32::consts::PI;


use std::time::Instant;

pub const MAX: u32 = 500;

#[derive(Clone)]
pub struct Scene {
    pub start: Instant,
    pub size: f32,
    pub rotation: Mat4,
    pub polyhedron: PolyGraph,
    pub camera: Camera,
    pub light_color: Color,
}

impl Scene {
    pub fn new() -> Self {
        let scene = Self {
            start: Instant::now(),
            size: 1.0,
            rotation: Mat4::IDENTITY,
            polyhedron: PolyGraph::dodecahedron(),
            camera: Camera::default(),
            light_color: Color::WHITE,
        };

        scene
    }

    pub fn update2(&mut self, time: Duration) {
        self.polyhedron.update();
        let time = time.as_secs_f32();
        self.rotation = Mat4::from_rotation_x(time / PI) * Mat4::from_rotation_y(time / PI * 1.1);
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
        Primitive::new(&self.polyhedron, &self.rotation, &self.camera)
    }
}

/// A collection of `Cube`s that can be rendered.
#[derive(Debug)]
pub struct Primitive {
    polyhedron: PolyGraph,
    rotation: Mat4,
    camera: Camera,
}

impl Primitive {
    pub fn new(pg: &PolyGraph, rotation: &Mat4, camera: &Camera) -> Self {
        Self {
            polyhedron: pg.clone(),
            rotation: rotation.clone(),
            camera: *camera,
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
            storage.store(Pipeline::new(
                device,
                queue,
                format,
                target_size,
                &self.polyhedron,
            ));
        }

        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        // update uniform buffer
        let model_mat = self.rotation;
        let view_projection_mat = self.camera.build_view_proj_mat(bounds);
        let normal_mat = (model_mat.inverse()).transpose();

        let uniforms = pipeline::Uniforms {
            model_mat,
            view_projection_mat,
            normal_mat,
        };

        let frag_uniforms = pipeline::FragUniforms {
            light_position: self.camera.position(),
            eye_position: self.camera.position() + vec4(2.0, 2.0, 1.0, 0.0),
        };

        //upload data to GPU
        pipeline.update(
            device,
            queue,
            target_size,
            &uniforms,
            &frag_uniforms,
            &self.polyhedron,
            &self.rotation,
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
