use crate::{bones::PolyGraph, render::camera::Camera};
use iced::widget::shader::{self, wgpu};
use iced::{Color, Rectangle, Size};
use ultraviolet::{Mat4, Vec4};

use super::{AllUniforms, FragUniforms, LightUniforms, ModelUniforms, Pipeline, PolyData};

#[derive(Debug)]
pub struct Polygon {
    vertex_count: u64,
    data: PolyData,
}

impl Polygon {
    pub fn new(pg: &PolyGraph, palette: &[wgpu::Color], transform: &Mat4, camera: &Camera) -> Self {
        Self {
            vertex_count: pg.vertex_count(),
            data: PolyData {
                positions: pg.positions(),
                vertices: pg.vertices(palette),
                transform: *transform,
                camera: *camera,
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
            storage.store(Pipeline::new(
                device,
                format,
                target_size,
                self.vertex_count,
            ));
        }
        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        // update uniform buffer
        let model_mat = self.data.transform;
        let view_projection_mat = self.data.camera.build_view_proj_mat(bounds);
        let uniforms = AllUniforms {
            model: ModelUniforms {
                model_mat,
                view_projection_mat,
            },
            frag: FragUniforms {
                light_position: self.data.camera.position(),
                eye_position: self.data.camera.position() + Vec4::new(2.0, 2.0, 1.0, 0.0),
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
            self.vertex_count,
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
        // At this point our pipeline should always be initialized
        let pipeline = storage.get::<Pipeline>().unwrap();

        // Render primitive
        pipeline.render(target, encoder, viewport);
    }
}
