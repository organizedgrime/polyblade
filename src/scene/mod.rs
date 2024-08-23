mod camera;
mod pipeline;
use self::polyhedron::Descriptor;
use crate::polyhedra::PolyGraph;
use crate::{wgpu, RGB};
use camera::Camera;
use iced::mouse;
use iced::time::Duration;
use iced::widget::shader;
use iced::{Color, Rectangle, Size};
use pipeline::Pipeline;
pub use pipeline::*;
use std::f32::consts::PI;
use std::time::Instant;
use ultraviolet::{Mat4, Vec4};

#[derive(Clone)]
pub struct Scene {
    pub start: Instant,
    pub size: f32,
    pub clear_face: Option<usize>,
    pub rotation: Mat4,
    pub polyhedron: PolyGraph,
    pub camera: Camera,
    pub light_color: Color,
    pub palette: Vec<wgpu::Color>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            size: 1.0,
            clear_face: None,
            rotation: Mat4::default(),
            polyhedron: PolyGraph::icosahedron(),
            camera: Camera::default(),
            palette: vec![
                RGB::new(72, 132, 90),
                RGB::new(163, 186, 112),
                RGB::new(51, 81, 69),
                RGB::new(254, 240, 134),
                RGB::new(95, 155, 252),
                RGB::new(244, 164, 231),
                RGB::new(170, 137, 190),
            ]
            .into_iter()
            .map(Into::<wgpu::Color>::into)
            .collect(),
            light_color: Color::WHITE,
        }
    }

    pub fn update(&mut self, schlegel: bool, time: Duration) {
        self.polyhedron.update();
        let time = time.as_secs_f32();
        self.rotation = Mat4::default();
        if !schlegel {
            self.rotation = Mat4::from_scale(self.size)
                * Mat4::from_rotation_x(time / PI)
                * Mat4::from_rotation_y(time / PI * 1.1);
        }
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
        Primitive::new(
            &self.polyhedron,
            &self.palette,
            &self.rotation,
            &self.camera,
        )
    }
}

#[derive(Debug)]
pub struct Primitive {
    descriptor: Descriptor,
    camera: Camera,
    rotation: Mat4,
    data: PolyData,
}

impl Primitive {
    pub fn new(pg: &PolyGraph, palette: &[wgpu::Color], rotation: &Mat4, camera: &Camera) -> Self {
        Self {
            descriptor: pg.into(),
            camera: *camera,
            rotation: *rotation,
            data: PolyData {
                positions: pg.positions(),
                vertices: pg.vertices(palette),
                raw: rotation.into(),
            },
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
            storage.store(Pipeline::new(device, format, target_size, &self.descriptor));
        }
        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        // update uniform buffer
        let model_mat = self.rotation;
        let view_projection_mat = self.camera.build_view_proj_mat(bounds);
        let normal_mat = (model_mat.inversed()).transposed();
        let uniforms = AllUniforms {
            model: ModelUniforms {
                model_mat,
                view_projection_mat,
                normal_mat,
            },
            frag: FragUniforms {
                light_position: self.camera.position(),
                eye_position: self.camera.position() + Vec4::new(2.0, 2.0, 1.0, 0.0),
            },
            light: LightUniforms::new(
                Color::new(1.0, 1.0, 1.0, 1.0),
                Color::new(1.0, 1.0, 1.0, 1.0),
            ),
        };

        //upload data to GPU
        pipeline.update(
            device,
            queue,
            target_size,
            &self.descriptor,
            &uniforms,
            &self.data,
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
