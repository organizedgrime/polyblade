mod buffer;
mod polyhedron_primitive;
mod texture;

use buffer::Buffer;
use iced::{
    widget::shader::wgpu::{self, RenderPassDepthStencilAttachment},
    Rectangle, Size,
};
use texture::Texture;

pub use buffer::*;
pub use polyhedron_primitive::*;
use ultraviolet::{Vec3, Vec4};

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    position_buf: IndexBuffer,
    color_buf: IndexBuffer,
    barycentric_buf: IndexBuffer,
    sides_buf: IndexBuffer,
    /// Uniform Buffers
    model_buf: Buffer,
    frag_buf: Buffer,
    /// Depth Texture
    depth_texture: Texture,
    uniform_group: wgpu::BindGroup,
    /// Number of vertices to skip in case of schlegel
    starting_vertex: usize,
}

unsafe impl Send for Pipeline {}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, target_size: Size<u32>) -> Self {
        println!("NEW PIPELINE");
        let vertex_usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let uniform_usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;

        let position_buf = IndexBuffer::new::<Vec4>(device, "position_buf", vertex_usage);
        let color_buf = IndexBuffer::new::<Vec4>(device, "color_buf", vertex_usage);
        let barycentric_buf = IndexBuffer::new::<Vec4>(device, "barycentric_buf", vertex_usage);
        let sides_buf = IndexBuffer::new::<Vec4>(device, "sides_buf", vertex_usage);

        // Create Uniform Buffers
        let model_buf = Buffer::new::<ModelUniforms>(device, "ModelUniforms", uniform_usage);
        let frag_buf = Buffer::new::<FragUniforms>(device, "FragUniforms", uniform_usage);
        //depth buffer
        let depth_texture = Texture::create_depth_texture(device, &target_size);
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
            ],
        });

        let uniform_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("Uniforms bg"),
            layout: &uniform_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: model_buf.raw.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: frag_buf.raw.as_entire_binding(),
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
                "../shaders/shader.wgsl"
            ))),
        });

        let pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("Polyhedron Pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[wgpu::VertexBufferLayout {
                    array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                    step_mode: wgpu::VertexStepMode::Vertex,
                    attributes: &wgpu::vertex_attr_array![
                        // position
                        0 => Float32x4,
                        // color
                        1 => Float32x4,
                        // barycentric
                        2 => Float32x4,
                        // sides
                        3 => Float32x4,
                    ],
                }],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: Texture::DEPTH_FORMAT,
                depth_write_enabled: true,
                depth_compare: wgpu::CompareFunction::Less,
                stencil: wgpu::StencilState::default(),
                bias: wgpu::DepthBiasState::default(),
            }),
            multisample: wgpu::MultisampleState::default(),
            fragment: Some(wgpu::FragmentState {
                module: &shader,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format,
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            pipeline,
            position_buf,
            color_buf,
            barycentric_buf,
            sides_buf,
            model_buf,
            frag_buf,
            depth_texture,
            uniform_group,
            starting_vertex: 0,
        }
    }

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_size: Size<u32>,
        uniforms: &AllUniforms,
        primitive: &PolyhedronPrimitive,
    ) {
        // Update depth
        if target_size != self.depth_texture.size {
            self.depth_texture = Texture::create_depth_texture(device, &target_size);
        }

        if primitive.render.schlegel {
            self.starting_vertex = primitive.face_sides_buffer(0).len();
        } else {
            self.starting_vertex = 0;
        }

        self.position_buf
            .write(device, queue, primitive.position_buf());
        self.color_buf.write(device, queue, primitive.color_buf());
        self.barycentric_buf
            .write(device, queue, primitive.barycentric_buf());
        self.sides_buf.write(device, queue, primitive.sides_buf());

        // Write uniforms
        self.model_buf.write(queue, &uniforms.model);
        self.frag_buf.write(queue, &uniforms.frag);
    }

    pub fn render(
        &self,
        target: &wgpu::TextureView,
        encoder: &mut wgpu::CommandEncoder,
        viewport: Rectangle<u32>,
    ) {
        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("poly.pipeline.pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: target,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Load,
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: Some(RenderPassDepthStencilAttachment {
                    view: &self.depth_texture.view,
                    depth_ops: Some(wgpu::Operations {
                        load: wgpu::LoadOp::Clear(1.0),
                        store: wgpu::StoreOp::Store,
                    }),
                    stencil_ops: None,
                }),
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_scissor_rect(viewport.x, viewport.y, viewport.width, viewport.height);
            pass.set_pipeline(&self.pipeline);
            pass.set_bind_group(0, &self.uniform_group, &[]);

            // Draw Positions
            pass.set_vertex_buffer(0, self.position_buf.data_raw.slice(..));
            pass.set_vertex_buffer(1, self.color_buf.data_raw.slice(..));
            pass.set_vertex_buffer(2, self.barycentric_buf.data_raw.slice(..));
            pass.set_vertex_buffer(3, self.sides_buf.data_raw.slice(..));

            pass.set_index_buffer(
                self.position_buf.index_raw.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            pass.draw_indexed(0..self.position_buf.index_count, 0, 0..1);

            // Draw Colors
            pass.set_index_buffer(
                self.color_buf.index_raw.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            pass.draw_indexed(0..self.color_buf.index_count, 0, 0..1);

            // Draw Barycentric
            pass.set_index_buffer(
                self.barycentric_buf.index_raw.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            pass.draw_indexed(0..self.barycentric_buf.index_count, 0, 0..1);

            pass.set_index_buffer(
                self.sides_buf.index_raw.slice(..),
                wgpu::IndexFormat::Uint32,
            );
            pass.draw_indexed(0..self.sides_buf.index_count, 0, 0..1);
        }
    }
}
