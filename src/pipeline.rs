use kas::draw::PassId;
use kas::prelude::*;
use kas_wgpu::draw::{CustomPipe, CustomPipeBuilder, CustomWindow};
use kas_wgpu::wgpu;
use std::mem::size_of;
use ultraviolet::{Mat4, Vec4};
use wgpu::util::DeviceExt;
use wgpu::{Buffer, ShaderModule};

use crate::{Polyblade, RGB};

const SHADER: &str = include_str!("./shaders/shader.wgsl");

pub struct Shaders {
    wgsl: ShaderModule,
}

impl Shaders {
    fn new(device: &wgpu::Device) -> Self {
        let wgsl = device.create_shader_module(wgpu::ShaderModuleDescriptor {
            label: None,
            source: wgpu::ShaderSource::Wgsl(std::borrow::Cow::Borrowed(SHADER)),
        });
        Shaders { wgsl }
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub position: Vec4,
}
unsafe impl bytemuck::Zeroable for Position {}
unsafe impl bytemuck::Pod for Position {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub struct Vertex {
    pub barycentric: Vec4,
    pub sides: Vec4,
    pub color: Vec4,
}
unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

#[repr(C)]
#[derive(Clone, Default, Copy, Debug)]
pub struct Transforms {
    pub model: Mat4,
    pub view_projection: Mat4,
    pub normal: Mat4,
}
unsafe impl bytemuck::Zeroable for Transforms {}
unsafe impl bytemuck::Pod for Transforms {}

pub struct PipeBuilder;

impl CustomPipeBuilder for PipeBuilder {
    type Pipe = Pipe;

    fn device_descriptor() -> wgpu::DeviceDescriptor<'static> {
        wgpu::DeviceDescriptor {
            label: None,
            features: wgpu::Features::PUSH_CONSTANTS,
            limits: wgpu::Limits {
                ..Default::default()
            },
        }
    }

    fn build(
        &mut self,
        device: &wgpu::Device,
        bgl_common: &wgpu::BindGroupLayout,
        tex_format: wgpu::TextureFormat,
    ) -> Self::Pipe {
        let shaders = Shaders::new(device);

        let uniform_layout = device.create_bind_group_layout(&wgpu::BindGroupLayoutDescriptor {
            label: Some("uniforms_bgl"),
            entries: &[wgpu::BindGroupLayoutEntry {
                binding: 0,
                visibility: wgpu::ShaderStages::VERTEX,
                ty: wgpu::BindingType::Buffer {
                    ty: wgpu::BufferBindingType::Uniform,
                    has_dynamic_offset: false,
                    min_binding_size: None,
                },
                count: None,
            }],
        });
        let pipeline_layout = device.create_pipeline_layout(&wgpu::PipelineLayoutDescriptor {
            label: None,
            bind_group_layouts: &[bgl_common, &uniform_layout],
            push_constant_ranges: &[],
        });

        let render_pipeline = device.create_render_pipeline(&wgpu::RenderPipelineDescriptor {
            label: None,
            layout: Some(&pipeline_layout),
            vertex: wgpu::VertexState {
                module: &shaders.wgsl,
                entry_point: "vs_main",
                buffers: &[
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<Position>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            // position
                            0 => Float32x3
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: size_of::<Vertex>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Vertex,
                        attributes: &wgpu::vertex_attr_array![
                            // barycentric
                            1 => Float32x4,
                            // sides
                            2 => Float32x4,
                            // color
                            3 => Float32x4,
                        ],
                    },
                    wgpu::VertexBufferLayout {
                        array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
                        step_mode: wgpu::VertexStepMode::Instance,
                        attributes: &wgpu::vertex_attr_array![
                            //cube transformation matrix
                            4 => Float32x4,
                            5 => Float32x4,
                            6 => Float32x4,
                            7 => Float32x4,
                            //normal rotation matrix
                            8 => Float32x3,
                            9 => Float32x3,
                            10 => Float32x3,
                        ],
                    },
                ],
            },
            primitive: wgpu::PrimitiveState::default(),
            /*
            primitive: wgpu::PrimitiveState {
                topology: wgpu::PrimitiveTopology::TriangleList,
                strip_index_format: None,
                front_face: wgpu::FrontFace::Cw,
                cull_mode: Some(wgpu::Face::Back), // not required
                unclipped_depth: false,
                polygon_mode: wgpu::PolygonMode::Fill,
                conservative: false,
            },
            */
            // TODO depth stencil
            depth_stencil: None,
            // multisample: Default::default(),
            multisample: wgpu::MultisampleState {
                count: 1,
                mask: !0,
                alpha_to_coverage_enabled: false,
            },
            fragment: Some(wgpu::FragmentState {
                module: &shaders.wgsl,
                entry_point: "fs_main",
                targets: &[Some(wgpu::ColorTargetState {
                    format: tex_format,
                    // TODO add blend mode
                    blend: None,
                    write_mask: wgpu::ColorWrites::ALL,
                })],
            }),
            multiview: None,
        });

        Pipe {
            render_pipeline,
            uniform_layout,
        }
    }
}

pub struct Pipe {
    render_pipeline: wgpu::RenderPipeline,
    uniform_layout: wgpu::BindGroupLayout,
}

pub struct PipeWindow {
    positions: (Vec<Position>, Option<Buffer>),
    vertices: (Vec<Vertex>, Option<Buffer>),
    transforms: (Transforms, Option<Buffer>),
    uniform_group: Option<wgpu::BindGroup>,
}

impl CustomPipe for Pipe {
    type Window = PipeWindow;

    fn new_window(&self, _: &wgpu::Device) -> Self::Window {
        PipeWindow {
            positions: Default::default(),
            vertices: Default::default(),
            transforms: Default::default(),
            uniform_group: Default::default(),
        }
    }

    fn prepare(
        &self,
        window: &mut Self::Window,
        device: &wgpu::Device,
        _: &mut wgpu::util::StagingBelt,
        _: &mut wgpu::CommandEncoder,
    ) {
        if !window.positions.0.is_empty() {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vs_positions"),
                contents: bytemuck::cast_slice(&window.positions.0),
                usage: wgpu::BufferUsages::VERTEX,
            });
            window.positions.1 = Some(buffer);
        } else {
            window.positions.1 = None;
        }

        if !window.vertices.0.is_empty() {
            let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
                label: Some("vs_vertices"),
                contents: bytemuck::cast_slice(&window.vertices.0),
                usage: wgpu::BufferUsages::VERTEX,
            });
            window.vertices.1 = Some(buffer);
        } else {
            window.vertices.1 = None;
        }

        let buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("vs_uniforms"),
            contents: bytemuck::bytes_of(&window.transforms.0),
            usage: wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::VERTEX,
        });
        window.transforms.1 = Some(buffer);

        if window.uniform_group.is_none() {
            if let Some(transforms) = &window.transforms.1 {
                window.uniform_group = Some(device.create_bind_group(&wgpu::BindGroupDescriptor {
                    label: Some("uniforms_bg"),
                    layout: &self.uniform_layout,
                    entries: &[wgpu::BindGroupEntry {
                        binding: 0,
                        resource: transforms.as_entire_binding(),
                    }],
                }))
            }
        }
    }

    fn render_pass<'a>(
        &'a self,
        window: &'a mut Self::Window,
        _: &wgpu::Device,
        _: usize,
        rpass: &mut wgpu::RenderPass<'a>,
        bg_common: &'a wgpu::BindGroup,
    ) {
        rpass.set_pipeline(&self.render_pipeline);
        rpass.set_bind_group(0, bg_common, &[]);
        if let Some(uniform_group) = &window.uniform_group {
            rpass.set_bind_group(1, uniform_group, &[]);
        }
        if window.positions.1.is_some() {
            rpass.set_vertex_buffer(0, window.positions.1.as_ref().unwrap().slice(..));
        }
        if window.vertices.1.is_some() {
            rpass.set_vertex_buffer(1, window.vertices.1.as_ref().unwrap().slice(..));
        }
        if window.transforms.1.is_some() {
            rpass.set_vertex_buffer(2, window.transforms.1.as_ref().unwrap().slice(..));
        }
        rpass.draw(0..window.positions.0.len() as u32, 0..1);
    }
}

impl CustomWindow for PipeWindow {
    type Param = Polyblade;

    fn invoke(&mut self, pass: PassId, rect: Rect, pblade: Self::Param) {
        println!("render pass: {:?}", pass.pass());
        let palette: Vec<wgpu::Color> = vec![
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
        .collect();

        self.positions.0 = pblade
            .polyhedron
            .positions()
            .into_iter()
            .map(|v| Position {
                position: Vec4::new(v.x, v.y, v.z, 0.0),
            })
            .collect();
        self.vertices.0 = pblade.polyhedron.vertices(None, &palette);
        self.transforms.0.model = Mat4::from_scale(pblade.size);
        self.transforms.0.normal = self.transforms.0.model.inversed().transposed();
        self.transforms.0.view_projection = Mat4::identity();
    }
}
