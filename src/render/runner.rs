use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Engine, Renderer};
use iced_winit::conversion;
use iced_winit::core::mouse;
use iced_winit::core::renderer;
use iced_winit::core::{Color, Font, Pixels, Size, Theme};
use iced_winit::futures;
use iced_winit::runtime::program;
use iced_winit::runtime::Debug;
use iced_winit::winit;
use iced_winit::winit::dpi::PhysicalSize;
use iced_winit::winit::window::Window;
use iced_winit::Clipboard;

use winit::{event::WindowEvent, event_loop::ControlFlow, keyboard::ModifiersState};

use crate::render::{
    controls::Controls,
    message::{ConwayMessage, PolybladeMessage, PresetMessage},
    pipeline::{FragUniforms, ModelUniforms, PolyhedronPrimitive, Scene},
};

#[cfg(target_arch = "wasm32")]
pub use iced::time::Instant;
use std::iter;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

pub struct Graphics<'a> {
    surface: wgpu::Surface<'a>,
    device: wgpu::Device,
    queue: wgpu::Queue,
    config: wgpu::SurfaceConfiguration,
    //size: winit::dpi::PhysicalSize<u32>,
    viewport: Viewport,
    engine: Engine,
    renderer: Renderer,
    window: &'a Window,
}

impl<'a> Graphics<'a> {
    pub async fn new(window: &'a Window) -> Graphics<'a> {
        let size = window.inner_size();
        let physical_size = Size::new(size.width.max(1), size.height.max(1));
        let viewport = Viewport::with_physical_size(physical_size, window.scale_factor());

        let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
            #[cfg(not(target_arch = "wasm32"))]
            backends: wgpu::Backends::PRIMARY,
            #[cfg(target_arch = "wasm32")]
            backends: wgpu::Backends::GL,
            ..Default::default()
        });

        let surface = instance.create_surface(window).unwrap();

        let adapter = instance
            .request_adapter(&wgpu::RequestAdapterOptions {
                power_preference: wgpu::PowerPreference::default(),
                compatible_surface: Some(&surface),
                force_fallback_adapter: false,
            })
            .await
            .unwrap();

        let (device, queue) = adapter
            .request_device(
                &wgpu::DeviceDescriptor {
                    label: None,
                    required_features: wgpu::Features::empty(),
                    // WebGL doesn't support all of wgpu's features, so if
                    // we're building for the web we'll have to disable some.
                    required_limits: if cfg!(target_arch = "wasm32") {
                        wgpu::Limits::downlevel_webgl2_defaults()
                    } else {
                        wgpu::Limits::default()
                    },
                },
                // Some(&std::path::Path::new("trace")), // Trace path
                None,
            )
            .await
            .unwrap();

        let surface_caps = surface.get_capabilities(&adapter);

        let surface_format = surface_caps
            .formats
            .iter()
            .copied()
            .find(|f| f.is_srgb())
            .unwrap_or(surface_caps.formats[0]);

        let config = wgpu::SurfaceConfiguration {
            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
            format: surface_format,
            width: size.width,
            height: size.height,
            present_mode: surface_caps.present_modes[0],
            alpha_mode: surface_caps.alpha_modes[0],
            desired_maximum_frame_latency: 2,
            view_formats: vec![],
        };

        let engine = Engine::new(&adapter, &device, &queue, surface_format, None);
        let renderer = Renderer::new(&device, &engine, Font::default(), Pixels::from(16));

        Self {
            surface,
            device,
            queue,
            config,
            viewport,
            engine,
            renderer,
            window,
        }
    }

    pub fn resize(&mut self, physical_size: Size<u32>) {
        if physical_size.width > 0 && physical_size.height > 0 {
            self.viewport =
                Viewport::with_physical_size(physical_size, self.window().scale_factor());
            self.config.width = physical_size.width;
            self.config.height = physical_size.height;
            self.surface.configure(&self.device, &self.config);
        }
    }

    fn window(&self) -> &Window {
        &self.window
    }
}

pub struct App<'a> {
    pub graphics: Graphics<'a>,
    pub data: Option<AppData>,
    pub surface_configured: bool,
}

pub struct AppData {
    scene: Scene,
    state: program::State<Controls>,
    debug: Debug,
}

impl<'a> App<'a> {
    pub fn render(&mut self) -> Result<(), wgpu::SurfaceError> {
        let Some(AppData {
            scene,
            state,
            debug,
        }) = &mut self.data
        else {
            return Ok(());
        };

        state.queue_message(PolybladeMessage::Tick(Instant::now()));

        let program = state.program();
        let program_state = &program.state;

        {
            let state = &state.program().state;
            let model = state.model.clone();
            let render = state.render.clone();
            let primitive = PolyhedronPrimitive::new(model, render);

            let moments = primitive.moment_vertices();
            if scene.moment_buf.count != moments.len() as u32 {
                scene
                    .moment_buf
                    .resize(&self.graphics.device, moments.len() as u32);

                let shapes = primitive.shape_vertices();
                scene
                    .shape_buf
                    .resize(&self.graphics.device, shapes.len() as u32);
                self.graphics.queue.write_buffer(
                    &scene.shape_buf.raw,
                    0,
                    bytemuck::cast_slice(&shapes),
                );
            }

            self.graphics.queue.write_buffer(
                &scene.moment_buf.raw,
                0,
                bytemuck::cast_slice(&moments),
            );

            let model = ModelUniforms {
                model_mat: primitive.model.transform,
                view_projection_mat: primitive
                    .render
                    .camera
                    .build_view_proj_mat(self.graphics.viewport.logical_size()),
            };
            let frag = FragUniforms {
                line_thickness: primitive.render.line_thickness,
                line_mode: 1.0,
                ..Default::default()
            };
            scene.model_buf.write_data(&self.graphics.queue, &model);
            scene.frag_buf.write_data(&self.graphics.queue, &frag);
            self.graphics.window.request_redraw();
        }

        let output = self.graphics.surface.get_current_texture()?;
        let view = output
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder =
            self.graphics
                .device
                .create_command_encoder(&wgpu::CommandEncoderDescriptor {
                    label: Some("Render Encoder"),
                });

        //let program = inner_state.pro
        {
            {
                // We clear the frame
                let mut render_pass = scene.clear(&view, &mut encoder, program.background_color());

                // // Ignore the whole first polygon if we're in schlegel mode
                // let starting_vertex = if program.state.render.schlegel {
                //     match program.state.model.polyhedron.cycles[0].len() {
                //         3 => 3,
                //         4 => 6,
                //         n => n * 3,
                //     }
                // } else {
                //     0
                // } as u32;

                // Draw the scene
                scene.draw(0, &mut render_pass);
            }

            // And then iced on top
            self.graphics.renderer.present(
                &mut self.graphics.engine,
                &self.graphics.device,
                &self.graphics.queue,
                &mut encoder,
                None,
                output.texture.format(),
                &view,
                &self.graphics.viewport,
                &debug.overlay(),
            );

            // output.present();

            // Then we submit the work
            self.graphics.engine.submit(&self.graphics.queue, encoder);
            // self.graphics.queue.submit(iter::once(encoder.finish()));
            output.present();
        }

        Ok(())

        // if *resized {
        //     let size = window.inner_size();
        //     let physical_size = Size::new(size.width.max(1), size.height.max(1));
        //
        //     **viewport = Viewport::with_physical_size(physical_size, window.scale_factor());
        //
        //     // Update depth
        //     if physical_size != scene.depth_texture.size {
        //         scene.depth_texture =
        //             crate::render::pipeline::Texture::create_depth_texture(device, &physical_size);
        //     }
        //
        //     surface.configure(
        //         device,
        //         &wgpu::SurfaceConfiguration {
        //             format: *format,
        //             usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
        //             width: size.width.max(1),
        //             height: size.height.max(1),
        //             present_mode: wgpu::PresentMode::AutoVsync,
        //             alpha_mode: wgpu::CompositeAlphaMode::Auto,
        //             view_formats: vec![],
        //             desired_maximum_frame_latency: 2,
        //         },
        //     );
        //     *resized = false;
        // }
        //
        //
        // match surface.get_current_texture() {
        //     Ok(frame) => {
        //         let mut encoder =
        //             device.create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });
        //
        //         let program = state.program();
        //
        //         let view = frame
        //             .texture
        //             .create_view(&wgpu::TextureViewDescriptor::default());
        //
        //         {
        //             // We clear the frame
        //             let mut render_pass =
        //                 scene.clear(&view, &mut encoder, program.background_color());
        //
        //             // Ignore the whole first polygon if we're in schlegel mode
        //             let starting_vertex = if program.state.render.schlegel {
        //                 match program.state.model.polyhedron.cycles[0].len() {
        //                     3 => 3,
        //                     4 => 6,
        //                     n => n * 3,
        //                 }
        //             } else {
        //                 0
        //             } as u32;
        //
        //             // Draw the scene
        //             scene.draw(starting_vertex, &mut render_pass);
        //         }
        //
        //         // And then iced on top
        //         renderer.present(
        //             engine,
        //             device,
        //             queue,
        //             &mut encoder,
        //             None,
        //             frame.texture.format(),
        //             &view,
        //             viewport,
        //             &debug.overlay(),
        //         );
        //
        //         // Then we submit the work
        //         engine.submit(queue, encoder);
        //         frame.present();
        //
        //         // Update the mouse cursor
        //         window.set_cursor(iced_winit::conversion::mouse_interaction(
        //             state.mouse_interaction(),
        //         ));
        //     }
        //     Err(error) => match error {
        //         wgpu::SurfaceError::OutOfMemory => {
        //             panic!(
        //                 "Swapchain error: {error}. \
        //                         Rendering cannot continue."
        //             )
        //         }
        //         _ => {
        //             // Try rendering again next frame.
        //             window.request_redraw();
        //         }
        //     },
        // }
    }
}

impl<'a> winit::application::ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Graphics {
            surface,
            device,
            queue,
            config,
            viewport,
            engine,
            renderer,
            window,
        } = &mut self.graphics
        {
            // Initialize scene and GUI controls
            let scene = Scene::new(&device, config.format, &viewport.physical_size());
            let controls = Controls::new();
            // Initialize iced
            let mut debug = Debug::new();
            let state =
                program::State::new(controls, viewport.logical_size(), renderer, &mut debug);
            // You should change this if you want to render continuously
            event_loop.set_control_flow(ControlFlow::Poll);
            self.data = Some(AppData {
                scene,
                state,
                debug,
            });
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        {
            match &event {
                WindowEvent::RedrawRequested => {
                    self.graphics.window().request_redraw();
                    if !self.surface_configured {
                        return;
                    }

                    //self.graphics.update();
                    match self.render() {
                        Ok(_) => {}
                        // Reconfigure the surface if it's lost or outdated
                        Err(wgpu::SurfaceError::Lost | wgpu::SurfaceError::Outdated) => {
                            self.graphics.resize(self.graphics.viewport.physical_size())
                        }
                        // The system is out of memory, we should probably quit
                        Err(wgpu::SurfaceError::OutOfMemory) => {
                            log::error!("OutOfMemory");
                            event_loop.exit();
                        }

                        // This happens when the a frame takes too long to present
                        Err(wgpu::SurfaceError::Timeout) => {
                            log::warn!("Surface timeout")
                        }
                    }
                }
                WindowEvent::Resized(physical_size) => {
                    let size = Size {
                        width: physical_size.width,
                        height: physical_size.height,
                    };
                    self.graphics.resize(size.clone());
                    if let Some(data) = &mut self.data {
                        data.scene.depth_texture =
                            crate::render::pipeline::Texture::create_depth_texture(
                                &self.graphics.device,
                                &size,
                            );
                    }
                    self.surface_configured = true;
                }
                WindowEvent::CloseRequested => {
                    event_loop.exit();
                }
                WindowEvent::KeyboardInput { event, .. } => {
                    if event.state.is_pressed() {
                        if let Some(key) = &event.text {
                            let message = if key.as_str() == key.to_uppercase().as_str() {
                                use PresetMessage::*;
                                match key.to_lowercase().as_str() {
                                    // Presets
                                    "t" => Some(Pyramid(3)),
                                    "c" => Some(Prism(4)),
                                    "o" => Some(Octahedron),
                                    "d" => Some(Dodecahedron),
                                    "i" => Some(Icosahedron),
                                    _ => None,
                                }
                                .map(PolybladeMessage::Preset)
                            } else {
                                use ConwayMessage::*;
                                match key.as_str() {
                                    // Operations
                                    "e" => Some(Expand),
                                    "d" => Some(Dual),
                                    "s" => Some(Snub),
                                    "k" => Some(Kis),
                                    "j" => Some(Join),
                                    "a" => Some(Ambo),
                                    "t" => Some(Truncate),
                                    "b" => Some(Bevel),
                                    _ => None,
                                }
                                .map(PolybladeMessage::Conway)
                            };

                            if let Some(message) = message {
                                if let Some(data) = &mut self.data {
                                    data.state.queue_message(message);
                                }
                            }
                        }
                    }
                }
                _ => {}
            }
        }

        if let Some(data) = &mut self.data {
            // Map window event to iced event
            if let Some(event) = iced_winit::conversion::window_event(
                event,
                self.graphics.window().scale_factor(),
                ModifiersState::default(),
            ) {
                data.state.queue_event(event);
            }

            // If there are events pending
            if !data.state.is_queue_empty() {
                // We update iced
                let _ = data.state.update(
                    self.graphics.viewport.logical_size(),
                    mouse::Cursor::Unavailable,
                    // cursor_position
                    //     .map(|p| conversion::cursor_position(p, graphics.viewport.scale_factor()))
                    //     .map(mouse::Cursor::Available)
                    //     .unwrap_or(mouse::Cursor::Unavailable),
                    &mut self.graphics.renderer,
                    &Theme::Dark,
                    &renderer::Style {
                        text_color: Color::WHITE,
                    },
                    &mut Clipboard::unconnected(),
                    &mut data.debug,
                );

                // and request a redraw
                self.graphics.window.request_redraw();
            }
        }
    }
}
