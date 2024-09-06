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

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    /// Indices
    index_buf: Buffer,
    /// Raw XYZ position data for each vertex
    vertex_buf: Buffer,
    /// Other vertex data
    // vertices: Buffer,
    /// Uniform Buffers
    model: Buffer,
    frag: Buffer,
    /// Depth Texture
    depth_texture: Texture,
    uniform_group: wgpu::BindGroup,
    /// Number of vertices to skip in case of schlegel
    starting_vertex: usize,
    index_count: u64,
    initialized: bool,
}

unsafe impl Send for Pipeline {}

impl Pipeline {
    pub fn new(device: &wgpu::Device, format: wgpu::TextureFormat, target_size: Size<u32>) -> Self {
        println!("NEW PIPELINE");
        let index_usage = wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST;
        let vertex_usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let uniform_usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;

        let indices = Buffer::new::<u16>(device, "Index Buffer", index_usage);
        let vertex_buf = Buffer::new::<MomentVertex>(device, "Vertex Buffer", vertex_usage);
        // let vertices = Buffer::new::<ShapeVertex>(device, "Vertex buffer", vertex_usage);

        // Create Uniform Buffers
        let model = Buffer::new::<ModelUniforms>(device, "ModelUniforms", uniform_usage);
        let frag = Buffer::new::<FragUniforms>(device, "FragUniforms", uniform_usage);
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
                    resource: model.raw.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: frag.raw.as_entire_binding(),
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
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<MomentVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            // position
                            0 => Float32x3,
                            // color
                            1 => Float32x4,
                        ],
                    },
                    /* wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<ShapeVertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            // normal
                            2 => Float32x4,
                            // barycentric
                            3 => Float32x4,
                            // sides
                            4 => Float32x4,
                        ],
                    }, */
                ],
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
            index_buf: indices,
            vertex_buf,
            // vertices,
            model,
            frag,
            depth_texture,
            uniform_group,
            starting_vertex: 0,
            index_count: 1,
            initialized: false,
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

        let (moment_vertices, indices) = primitive.moment_vertices();
        self.index_count = indices.len() as u64;

        // Resize buffer if required
        if self.index_buf.count != self.index_count || !self.initialized {
            println!("resizing buffer!");
            self.index_buf.resize(device, self.index_count);
            // Resize the vertex buffer
            self.vertex_buf.resize(device, moment_vertices.len() as u64);
            // Resize the vertex buffer
            //self.vertex_buf.resize(device, moment_vertices.len() as u64);
            // Count that
            // Write the vertices
            // queue.write_buffer(
            //     &self.vertices.raw,
            //     0,
            //     bytemuck::cast_slice(&primitive.vertices()),
            // );
            // Initialized
            self.initialized = true;
        }

        // Write all position and color data
        queue.write_buffer(&self.index_buf.raw, 0, bytemuck::cast_slice(&indices));
        queue.write_buffer(
            &self.vertex_buf.raw,
            0,
            bytemuck::cast_slice(&moment_vertices),
        );

        // Write uniforms
        queue.write_buffer(&self.model.raw, 0, bytemuck::bytes_of(&uniforms.model));
        queue.write_buffer(&self.frag.raw, 0, bytemuck::bytes_of(&uniforms.frag));
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
            pass.set_index_buffer(self.index_buf.raw.slice(..), wgpu::IndexFormat::Uint16);
            pass.set_vertex_buffer(0, self.vertex_buf.raw.slice(..));
            //pass.set_vertex_buffer(1, self.vertices.raw.slice(..));
            pass.draw_indexed(
                self.starting_vertex as u32..self.index_count as u32,
                0,
                0..1,
            );
        }
    }
}
