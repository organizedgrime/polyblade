use crate::polyhedra::PolyGraph;
use crate::wgpu;
use iced::widget::shader;
use iced::{Color, Rectangle, Size};
use ultraviolet::{Mat4, Vec4};

use super::camera::Camera;
use super::pipeline::Pipeline;
use super::polyhedron::Descriptor;
use super::{AllUniforms, FragUniforms, LightUniforms, ModelUniforms, PolyData};

#[derive(Debug)]
pub struct Polygon {
    descriptor: Descriptor,
    transform: Mat4,
    data: PolyData,
}

impl Polygon {
    pub fn new(pg: &PolyGraph, palette: &[wgpu::Color], transform: &Mat4) -> Self {
        Self {
            descriptor: pg.into(),
            transform: *transform,
            data: PolyData {
                positions: pg.positions(),
                vertices: pg.vertices(palette),
                transform: *transform,
            },
        }
    }
}

impl shader::Primitive for Polygon {
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
        let model_mat = self.transform;
        let camera = Camera::default();
        let view_projection_mat = camera.build_view_proj_mat(bounds);
        let uniforms = AllUniforms {
            model: ModelUniforms {
                model_mat,
                view_projection_mat,
            },
            frag: FragUniforms {
                light_position: camera.position(),
                eye_position: camera.position() + Vec4::new(2.0, 2.0, 1.0, 0.0),
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
