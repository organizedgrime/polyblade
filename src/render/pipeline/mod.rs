mod buffer;
mod polygon;
mod uniforms;
mod vertex;

use buffer::Buffer;
use iced::{
    widget::shader::wgpu::{self, RenderPassDepthStencilAttachment},
    Rectangle, Size,
};
use ultraviolet::{Mat4, Vec3};

pub use polygon::*;
pub use uniforms::*;
pub use vertex::*;

pub struct Pipeline {
    pipeline: wgpu::RenderPipeline,
    positions: Buffer,
    vertices: Buffer,
    polyhedron: Buffer,
    model_uniform: Buffer,
    frag_uniform: Buffer,
    light_uniform: Buffer,
    uniform_group: wgpu::BindGroup,
    depth_texture_size: Size<u32>,
    depth_view: wgpu::TextureView,
    depth_pipeline: DepthPipeline,
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

        // Polyhedron instance data
        let polyhedron = Buffer::new::<Mat4>(device, "Polyhedron instance buffer", 1, vertex_usage);
        let model_uniform =
            Buffer::new::<ModelUniforms>(device, "Uniforms Buffer", 1, uniform_usage);

        let frag_uniform =
            Buffer::new::<FragUniforms>(device, "FragUniforms Buffer", 1, uniform_usage);
        let light_uniform =
            Buffer::new::<LightUniforms>(device, "LightUniforms Buffer", 1, uniform_usage);
        //depth buffer
        let depth_texture = device.create_texture(&wgpu::TextureDescriptor {
            label: Some("Depth texture"),
            size: wgpu::Extent3d {
                width: target_size.width,
                height: target_size.height,
                depth_or_array_layers: 1,
            },
            mip_level_count: 1,
            sample_count: 1,
            dimension: wgpu::TextureDimension::D2,
            format: wgpu::TextureFormat::Depth32Float,
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT | wgpu::TextureUsages::TEXTURE_BINDING,
            view_formats: &[],
        });
        let depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

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
                    resource: model_uniform.raw.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: frag_uniform.raw.as_entire_binding(),
                },
                wgpu::BindGroupEntry {
                    binding: 2,
                    resource: light_uniform.raw.as_entire_binding(),
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
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Mat4>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![
                            //cube transformation matrix
                            5 => Float32x4,
                            6 => Float32x4,
                            7 => Float32x4,
                            8 => Float32x4,
                        ],
                    },
                ],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
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

        let depth_pipeline = DepthPipeline::new(
            device,
            format,
            depth_texture.create_view(&wgpu::TextureViewDescriptor::default()),
        );

        Self {
            pipeline,
            polyhedron,
            model_uniform,
            frag_uniform,
            light_uniform,
            uniform_group,
            positions,
            vertices,
            depth_view,
            depth_texture_size: target_size,
            depth_pipeline,
            vertex_count,
            initialized: false,
        }
    }

    fn update_depth_texture(&mut self, device: &wgpu::Device, size: Size<u32>) {
        if self.depth_texture_size.height != size.height
            || self.depth_texture_size.width != size.width
        {
            let text = device.create_texture(&wgpu::TextureDescriptor {
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

            self.depth_pipeline.update(device, &text);
        }
    }

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
        self.update_depth_texture(device, target_size);

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
        queue.write_buffer(&self.polyhedron.raw, 0, bytemuck::bytes_of(&data.transform));

        // Write uniforms
        queue.write_buffer(
            &self.model_uniform.raw,
            0,
            bytemuck::bytes_of(&uniforms.model),
        );
        queue.write_buffer(
            &self.frag_uniform.raw,
            0,
            bytemuck::bytes_of(&uniforms.frag),
        );
        queue.write_buffer(
            &self.light_uniform.raw,
            0,
            bytemuck::bytes_of(&uniforms.light),
        );
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
                    view: &self.depth_view,
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
            pass.set_vertex_buffer(2, self.polyhedron.raw.slice(..));
            pass.draw(0..self.vertex_count as u32, 0..1);
        }
    }
}

struct DepthPipeline {
    bind_group_layout: wgpu::BindGroupLayout,
    bind_group: wgpu::BindGroup,
    sampler: wgpu::Sampler,
    depth_view: wgpu::TextureView,
}

impl DepthPipeline {
    pub fn new(
        device: &wgpu::Device,
        format: wgpu::TextureFormat,
        depth_texture: wgpu::TextureView,
    ) -> Self {
        let sampler = device.create_sampler(&wgpu::SamplerDescriptor {
            label: Some("cubes.depth_pipeline.sampler"),
            ..Default::default()
        });

        let bind_group_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("cubes.depth_pipeline.bind_group_layout"),
            entries: &[
                wgpu::BindGroupLayoutEntry {
                    binding: 0,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Sampler(wgpu::SamplerBindingType::NonFiltering),
                    count: None,
                },
                wgpu::BindGroupLayoutEntry {
                    binding: 1,
                    visibility: wgpu::ShaderStages::FRAGMENT,
                    ty: wgpu::BindingType::Texture {
                        sample_type: wgpu::TextureSampleType::Float { filterable: false },
                        view_dimension: wgpu::TextureViewDimension::D2,
                        multisampled: false,
                    },
                    count: None,
                },
            ],
        });

        let bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cubes.depth_pipeline.bind_group"),
            layout: &bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&depth_texture),
                },
            ],
        });

        let layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: Some("cubes.depth_pipeline.layout"),
            bind_group_layouts: &[&bind_group_layout],
            push_constant_ranges: &[],
        });

        let shader = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: Some("cubes.depth_pipeline.shader"),
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(include_str!(
                "../shaders/depth.wgsl"
            ))),
        });

        let _pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: Some("polyhedron.depth_pipeline.pipeline"),
            layout: Some(&layout),
            vertex: wgpu::VertexState {
                module: &shader,
                entry_point: "vs_main",
                buffers: &[],
            },
            primitive: wgpu::PrimitiveState::default(),
            depth_stencil: Some(wgpu::DepthStencilState {
                format: wgpu::TextureFormat::Depth32Float,
                depth_write_enabled: false,
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
                    blend: Some(wgpu::BlendState::REPLACE),
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Self {
            bind_group_layout,
            bind_group,
            sampler,
            depth_view: depth_texture,
        }
    }

    pub fn update(&mut self, device: &wgpu::Device, depth_texture: &wgpu::Texture) {
        self.depth_view = depth_texture.create_view(&wgpu::TextureViewDescriptor::default());

        self.bind_group = device.create_bind_group(&wgpu::BindGroupDescriptor {
            label: Some("cubes.depth_pipeline.bind_group"),
            layout: &self.bind_group_layout,
            entries: &[
                wgpu::BindGroupEntry {
                    binding: 0,
                    resource: wgpu::BindingResource::Sampler(&self.sampler),
                },
                wgpu::BindGroupEntry {
                    binding: 1,
                    resource: wgpu::BindingResource::TextureView(&self.depth_view),
                },
            ],
        });
    }
}
