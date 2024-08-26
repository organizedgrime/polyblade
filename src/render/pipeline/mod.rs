mod buffer;
mod polygon;
mod texture;
mod uniforms;
mod vertex;

use buffer::Buffer;
use iced::{
    widget::shader::wgpu::{self, RenderPassDepthStencilAttachment},
    Rectangle, Size,
};
use texture::Texture;
use ultraviolet::Vec3;

pub use polygon::*;
pub use uniforms::*;
pub use vertex::*;

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    /// Raw XYZ position data for each vertex
    positions: Buffer,
    /// Other vertex data
    vertices: Buffer,
    /// Uniform Buffers
    model: Buffer,
    frag: Buffer,
    light: Buffer,
    /// Depth Texture
    depth_texture: Texture,
    uniform_group: wgpu::BindGroup,
    /// Actual number of vertices when drawn using Triangles
    vertex_count: u64,
    initialized: bool,
}

impl Pipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        target_size: Size<u32>,
        vertex_count: u64,
    ) -> Self {
        println!("NEW PIPELINE");
        let vertex_usage = wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST;
        let uniform_usage = wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST;
        let positions = Buffer::new::<Vec3>(
            device,
            "Polyhedron position buffer",
            vertex_count,
            vertex_usage,
        );

        let vertices = Buffer::new::<Vertex>(
            device,
            "Polyhedron vertex buffer",
            vertex_count,
            vertex_usage,
        );

        // Create Uniform Buffers
        let model = Buffer::new::<ModelUniforms>(device, "ModelUniforms", 1, uniform_usage);
        let frag = Buffer::new::<FragUniforms>(device, "FragUniforms", 1, uniform_usage);
        let light = Buffer::new::<LightUniforms>(device, "LightUniforms", 1, uniform_usage);
        //depth buffer
        let depth_texture = Texture::create_depth_texture(device, &target_size, "meow");
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
                    resource: model.raw.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: frag.raw.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light.raw.as_entire_binding(),
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
                        array_stride: std::mem::size_of::<Vec3>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            // position
                            0 => Float32x3,
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            // normal
                            1 => Float32x4,
                            // barycentric
                            2 => Float32x4,
                            // sides
                            3 => Float32x4,
                            // color
                            4 => Float32x4,
                        ],
                    },
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
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            pipeline,
            positions,
            vertices,
            model,
            frag,
            light,
            depth_texture,
            uniform_group,
            vertex_count,
            initialized: false,
        }
    }

    /* fn update_depth_texture(&mut self, device: &wgpu::Device, size: Size<u32>) {
        if self.depth_texture.size.height != size.height
            || self.depth_texture.size.width != size.width
        {

            /* let text = device.create_texture(&wgpu::TextureDescriptor {
                label: Some("depth texture"),
                size: wgpu::Extent3d {
                    width: size.width,
                    height: size.height,
                    depth_or_array_layers: 1,
                },
                mip_level_count: 1,
                sample_count: 1,
                dimension: wgpu::TextureDimension::D2,
                format: wgpu::TextureFormat::Depth32Float,
                usage: wgpu::TextureUsages::RENDER_ATTACHMENT
                    | wgpu::TextureUsages::TEXTURE_BINDING,
                view_formats: &[],
            });

            self.depth_view = text.create_view(&wgpu::TextureViewDescriptor::default());
            self.depth_texture_size = size;

            self.depth_pipeline.update(device, &text); */
        }
    } */

    pub fn update(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        target_size: Size<u32>,
        vertex_count: u64,
        uniforms: &AllUniforms,
        data: &PolyData,
    ) {
        // Update depth
        //self.update_depth_texture(device, target_size);
        self.depth_texture = Texture::create_depth_texture(device, &target_size, "depth texture");

        // Resize buffer if required
        if self.positions.count != vertex_count || !self.initialized {
            println!(
                "positions count: {:?}; vtc: {}",
                self.positions.count, vertex_count
            );
            println!("resizing buffer!");
            // Resize the position buffer
            self.positions.resize(device, vertex_count);
            // Resize the vertex buffer
            self.vertices.resize(device, vertex_count);
            // Count that
            self.vertex_count = vertex_count;
            // Write the vertices
            queue.write_buffer(&self.vertices.raw, 0, bytemuck::cast_slice(&data.vertices));
            // Initialized
            self.initialized = true;
        }

        // Write all position data
        queue.write_buffer(
            &self.positions.raw,
            0,
            bytemuck::cast_slice(&data.positions),
        );
        // Write rotation data
        //queue.write_buffer(&self.transform.raw, 0, bytemuck::bytes_of(&data.transform));

        // Write uniforms
        queue.write_buffer(&self.model.raw, 0, bytemuck::bytes_of(&uniforms.model));
        queue.write_buffer(&self.frag.raw, 0, bytemuck::bytes_of(&uniforms.frag));
        queue.write_buffer(&self.light.raw, 0, bytemuck::bytes_of(&uniforms.light));
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
            pass.set_vertex_buffer(0, self.positions.raw.slice(..));
            pass.set_vertex_buffer(1, self.vertices.raw.slice(..));
            pass.draw(0..self.vertex_count as u32, 0..1);
        }
    }
}
