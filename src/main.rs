// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License in the LICENSE-APACHE file or at:
//     https://www.apache.org/licenses/LICENSE-2.0

//! Polyblade example
//!
//! Demonstrates use of a custom draw pipe.
//!

mod polyhedra;
use polyhedra::*;
mod color;
use color::*;

use kas::draw::{Draw, DrawIface, PassId};
use kas::event::{self, Command, Key};
use kas::prelude::*;
use kas::widgets::adapt::Reserve;
use kas::widgets::{format_data, format_value, Adapt, Button, Slider, Text};
use kas_wgpu::draw::{CustomPipe, CustomPipeBuilder, CustomWindow, DrawCustom, DrawPipe};
use kas_wgpu::wgpu;
use std::mem::size_of;
use ultraviolet::{Mat4, Vec4};
use wgpu::util::DeviceExt;
use wgpu::{Buffer, ShaderModule};

const SHADER: &str = include_str!("./shaders/shader.wgsl");

struct Shaders {
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
struct Position {
    pub position: Vec4,
}
unsafe impl bytemuck::Zeroable for Position {}
unsafe impl bytemuck::Pod for Position {}

#[repr(C)]
#[derive(Clone, Copy, Debug)]
struct Vertex {
    pub barycentric: Vec4,
    pub sides: Vec4,
    pub color: Vec4,
}
unsafe impl bytemuck::Zeroable for Vertex {}
unsafe impl bytemuck::Pod for Vertex {}

#[repr(C)]
#[derive(Clone, Default, Copy, Debug)]
struct Transforms {
    pub model: Mat4,
    pub view_projection: Mat4,
    pub normal: Mat4,
}
unsafe impl bytemuck::Zeroable for Transforms {}
unsafe impl bytemuck::Pod for Transforms {}

struct PipeBuilder;

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

struct Pipe {
    render_pipeline: wgpu::RenderPipeline,
    uniform_layout: wgpu::BindGroupLayout,
}

struct PipeWindow {
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

#[derive(Clone, Debug)]
struct ViewUpdate;

impl_scope! {
    #[widget]
    #[derive(Clone)]
    struct Polyblade {
        core: widget_core!(),
        polyhedron: PolyGraph,
        size: f32,
    }

    impl Polyblade {
        fn new() -> Self {
            Polyblade {
                core: Default::default(),
                polyhedron: PolyGraph::icosahedron(),
                size: 1.0,
            }
        }

        fn reset_view(&mut self) {
            self.size = 1.0;
        }
    }

    impl Layout for Polyblade {
        fn size_rules(&mut self, sizer: SizeCx, axis: AxisInfo) -> SizeRules {
            kas::layout::LogicalSize(800.0, 800.0)
                .to_rules_with_factor(axis, sizer.scale_factor(), 4.0)
                .with_stretch(Stretch::High)
        }

        #[inline]
        fn set_rect(&mut self, _: &mut ConfigCx, rect: Rect) {
            self.core.rect = rect;
        }

        fn draw(&mut self, mut draw: DrawCx) {
            let draw = draw.draw_device();
            let draw = DrawIface::<DrawPipe<Pipe>>::downcast_from(draw).unwrap();
            draw.draw.custom(draw.get_pass(), self.core.rect, self.clone());
        }
    }

    impl Events for Polyblade {
        type Data = i32;

        fn configure(&mut self, cx: &mut ConfigCx) {
            cx.register_nav_fallback(self.id());
        }

        fn update(&mut self, _: &mut ConfigCx, data: &i32) {
            self.polyhedron.update();
        }

        fn navigable(&self) -> bool {
            true
        }

        fn handle_event(&mut self, cx: &mut EventCx, _: &i32, event: Event) -> IsUsed {
            println!("nya: {event:?}");
            match event {
                Event::Key(event, is_synthetic) => {
                    println!("key: {event:?}");
                    let s: Key = Key::Character("s".into());
                    match event.logical_key {
                        s => {
                            self.polyhedron = PolyGraph::dodecahedron();
                        },
                        _ => {}
                    }
                }
                Event::Command(cmd, _) => {
                    match cmd {
                        Command::Home | Command::End => self.reset_view(),
                        Command::PageUp => {},
                        Command::PageDown => {},
                        cmd => {
                        }
                    }
                    cx.push(ViewUpdate);
                }
                Event::Scroll(delta) => {
                    let factor = match delta {
                        event::ScrollDelta::LineDelta(_, y) => -0.5 * y as f64,
                        event::ScrollDelta::PixelDelta(coord) => -0.01 * coord.1 as f64,
                    };
                    cx.push(ViewUpdate);
                }
                Event::Pan { alpha, delta } => {
                    cx.push(ViewUpdate);
                }
                Event::PressStart { press } => {
                    return press.grab(self.id())
                        .with_mode(event::GrabMode::PanFull)
                        .with_icon(event::CursorIcon::Grabbing)
                        .with_cx(cx);
                }
                _ => return Unused,
            }
            Used
        }
    }
}

impl_scope! {
    #[widget{
        layout = grid! {
            (1, 0) => self.label,
            (0, 1) => align!(center, self.zoom_label),
            (0, 2) => self.slider,
            (1..3, 1..4) => self.pblade,
        };
    }]
    struct PolybladeUI {
        core: widget_core!(),
        #[widget(&self.pblade)]
        label: Text<Polyblade, String>,
        #[widget(&self.zoom)]
        zoom_label: Reserve<Text<i32, String>>,
        #[widget(&self.zoom)]
        slider: Slider<i32, i32, kas::dir::Up>,
        #[widget(&self.zoom)]
        pblade: Polyblade,
        zoom: i32,
    }

    impl PolybladeUI {
        fn new() -> PolybladeUI {
            PolybladeUI {
                core: Default::default(),
                label: format_data!(pblade: &Polyblade, "{}", pblade.polyhedron.name),
                zoom_label: format_value!("{}").with_min_size_em(3.0, 0.0),
                slider: Slider::up(0..=1000, |_, zoom| *zoom).with_msg(|zoom| zoom),
                pblade: Polyblade::new(),
                zoom: 100,
            }
        }
    }

    impl Events for PolybladeUI {
        type Data = ();

        fn handle_messages(&mut self, cx: &mut EventCx, data: &()) {
            if let Some(zoom) = cx.try_pop() {
                self.zoom = zoom;
                self.pblade.size = self.zoom as f32 / 100.0;
                println!("updated zoom");
                cx.update(self.as_node(data));
            } else if let Some(ViewUpdate) = cx.try_pop() {
                cx.redraw(self.pblade.id());
            } else {
                println!("meowing");
                return;
            }
            cx.update(self.as_node(data));
        }
    }
}
#[derive(Clone, Debug)]
struct Increment(i32);
fn counter() -> impl Widget<Data = ()> {
    let tree = kas::column![
        align!(center, format_value!("{}")),
        kas::row![
            Button::label_msg("−", Increment(-1)),
            Button::label_msg("+", Increment(1)),
        ]
        .map_any(),
    ];

    Adapt::new(tree, 0).on_message(|_, count, Increment(add)| *count += add)
}

fn main() -> kas::app::Result<()> {
    env_logger::init();

    let window = Window::new(PolybladeUI::new(), "Polyblade");
    let theme = kas::theme::FlatTheme::new().with_colours("dark");
    kas::app::WgpuBuilder::new(PipeBuilder)
        .with_theme(theme)
        .build(())?
        .with(window)
        .run()
}
