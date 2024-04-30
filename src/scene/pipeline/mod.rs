pub mod cube;

mod buffer;
mod uniforms;
mod vertex;

use glam::{Mat4, Vec3};
pub use uniforms::{FragUniforms, LightUniforms, Uniforms};

use buffer::Buffer;
use vertex::Vertex;

use crate::wgpu;
use crate::wgpu::util::DeviceExt;

use iced::{Color, Rectangle, Size};

use super::transforms;

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    vertices: wgpu::Buffer,
    cube: Buffer,
    uniform: wgpu::Buffer,
    uniform_group: wgpu::BindGroup,
    view_mat: Mat4,
    project_mat: Mat4,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        format: wgpu::TextureFormat,
        target_size: Size<u32>,
    ) -> Self {
        //vertices of one cube
        let vertices = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("cubes vertex buffer"),
            contents: bytemuck::cast_slice(&cube::Raw::vertices()),
            usage: wgpu::BufferUsages::VERTEX,
        });

        //cube instance data
        let cubes_buffer = Buffer::new(
            device,
            "Polyhedron instance buffer",
            std::mem::size_of::<cube::Raw>() as u64,
            wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        );

        let uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("Uniforms buf"),
            size: std::mem::size_of::<Uniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let frag_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("FragUniforms buf"),
            size: std::mem::size_of::<FragUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        let camera_position: glam::Vec3 = (3.0, 1.5, 3.0).into();
        let look_direction = (0.0, 0.0, 0.0).into();
        let up_direction = Vec3::new(0.0, 1.0, 0.0);
        let (view_mat, project_mat, _) =
            transforms::create_view_projection(camera_position, look_direction, up_direction, 1.0);
        let light_position: &[f32; 3] = camera_position.as_ref();
        let eye_position: &[f32; 3] = camera_position.as_ref();
        queue.write_buffer(&frag_uniform, 0, bytemuck::cast_slice(light_position));
        queue.write_buffer(&frag_uniform, 16, bytemuck::cast_slice(eye_position));

        let light_uniform = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some("LightUniforms buf"),
            size: std::mem::size_of::<LightUniforms>() as u64,
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            mapped_at_creation: false,
        });
        queue.write_buffer(
            &light_uniform,
            0,
            bytemuck::cast_slice(&[LightUniforms::new(Color::WHITE, Color::WHITE)]),
        );

        // Uniform layout for Vertex Shader
        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("Uniforms bgl"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::VERTEX,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 2,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Buffer {
                        ty: wgpu::BufferBindingType::Uniform,
                        has_dynamic_offset: false,
                        min_binding_size: None,
                    },
                    count: None,
                },
            ],
        });

        let uniform_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniforms bg"),
            layout: &uniform_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: frag_uniform.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform.as_entire_binding(),
                },
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("Polyhedron layout"),
            bind_group_layouts: &[&uniform_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("Polyhedron shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "../../shaders/shader.wgsl"
            ))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Polyhedron Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[Vertex::desc(), cube::Raw::desc()],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: None,
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: Some(wgpu::BlendState {
                        color: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::SrcAlpha,
                            dst_factor: wgpu::BlendFactor::OneMinusSrcAlpha,
                            operation: wgpu::BlendOperation::Add,
                        },
                        alpha: wgpu::BlendComponent {
                            src_factor: wgpu::BlendFactor::One,
                            dst_factor: wgpu::BlendFactor::One,
                            operation: wgpu::BlendOperation::Max,
                        },
                    }),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            pipeline,
            cube: cubes_buffer,
            uniform,
            uniform_group,
            vertices,
            project_mat,
            view_mat,
        }
    }

    pub fn update(
        &mut self,
        _device: &wgpu::Device,
        queue: &wgpu::Queue,
        _target_size: Size<u32>,
        uniforms: &Uniforms,
        frag_uniforms: &FragUniforms,
        light_uniforms: &LightUniforms,
        cube: &cube::Raw,
        dt: std::time::Duration,
    ) {
        // update uniform buffer
        let dt = 1.0 * dt.as_secs_f32();
        let model_mat = transforms::create_transforms(
            [0.0, 0.0, 0.0],
            [dt.sin(), dt.cos(), 0.0],
            [1.0, 1.0, 1.0],
        );
        let view_project_mat = self.project_mat * self.view_mat;

        let normal_mat = (model_mat.inverse()).transpose();

        let model_ref: &[f32; 16] = model_mat.as_ref();
        let view_projection_ref: &[f32; 16] = view_project_mat.as_ref();
        let normal_ref: &[f32; 16] = normal_mat.as_ref();

        queue.write_buffer(&self.uniform, 0, bytemuck::bytes_of(uniforms));

        queue.write_buffer(&self.uniform, 0, bytemuck::cast_slice(model_ref));
        //queue.write_buffer(&self.uniform, 64, bytemuck::cast_slice(view_projection_ref));
        queue.write_buffer(&self.uniform, 128, bytemuck::cast_slice(normal_ref));
        // update uniforms
        //queue.write_buffer(&self.frag_uniform, 0, bytemuck::bytes_of(frag_uniforms));
        //queue.write_buffer(&self.light_uniform, 0, bytemuck::bytes_of(light_uniforms));

        //always write new cube data since they are constantly rotating
        queue.write_buffer(&self.cube.raw, 0, bytemuck::bytes_of(cube));
    }

    pub fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("cubes.pipeline.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.uniform_group, &[]);
            pass.set_vertex_buffer(0, self.vertices.slice(..));
            pass.set_vertex_buffer(1, self.cube.raw.slice(..));
            pass.draw(0..36, 0..1);
        }
    }
}
