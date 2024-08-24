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
