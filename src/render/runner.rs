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
use iced_winit::Clipboard;

use winit::{event::WindowEvent, event_loop::ControlFlow, keyboard::ModifiersState};

use crate::render::{
    controls::Controls,
    message::{ConwayMessage, PolybladeMessage, PresetMessage},
    pipeline::{FragUniforms, ModelUniforms, PolyhedronPrimitive, Scene},
};

#[cfg(target_arch = "wasm32")]
pub use iced::time::Instant;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

pub enum Runner {
    Loading,
    Ready {
        window: Arc<winit::window::Window>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
        format: wgpu::TextureFormat,
        engine: Engine,
        renderer: Renderer,
        scene: Scene,
        state: program::State<Controls>,
        cursor_position: Option<winit::dpi::PhysicalPosition<f64>>,
        clipboard: Clipboard,
        viewport: Box<Viewport>,
        modifiers: ModifiersState,
        resized: bool,
        debug: Box<Debug>,
    },
}

impl winit::application::ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        if let Self::Loading = self {
            let window = Arc::new(
                event_loop
                    .create_window(winit::window::WindowAttributes::default())
                    .expect("Create window"),
            );

            let physical_size = window.inner_size();
            let viewport = Box::new(Viewport::with_physical_size(
                Size::new(physical_size.width.max(1), physical_size.height.max(1)),
                window.scale_factor(),
            ));
            let clipboard = Clipboard::connect(window.clone());

            let backend = wgpu::util::backend_bits_from_env().unwrap_or_default();

            let instance = wgpu::Instance::new(wgpu::InstanceDescriptor {
                backends: backend,
                ..Default::default()
            });
            let surface = instance
                .create_surface(window.clone())
                .expect("Create window surface");

            let (format, adapter, device, queue) = futures::futures::executor::block_on(async {
                let adapter =
                    wgpu::util::initialize_adapter_from_env_or_default(&instance, Some(&surface))
                        .await
                        .expect("Create adapter");

                let adapter_features = adapter.features();

                let capabilities = surface.get_capabilities(&adapter);

                #[cfg(target_arch = "wasm32")]
                let (device, queue) = adapter
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            label: None,
                            required_features: adapter_features & wgpu::Features::all_webgpu_mask(),
                            required_limits: wgpu::Limits::downlevel_webgl2_defaults(),
                        },
                        None,
                    )
                    .await
                    .expect("Request Device");

                #[cfg(not(target_arch = "wasm32"))]
                let (device, queue) = adapter
                    .request_device(
                        &wgpu::DeviceDescriptor {
                            label: None,
                            required_features: adapter_features & wgpu::Features::default(),
                            required_limits: wgpu::Limits::default(),
                        },
                        None,
                    )
                    .await
                    .expect("Request Device");

                (
                    capabilities
                        .formats
                        .iter()
                        .copied()
                        .find(wgpu::TextureFormat::is_srgb)
                        .or_else(|| capabilities.formats.first().copied())
                        .expect("Get preferred format"),
                    adapter,
                    device,
                    queue,
                )
            });

            surface.configure(
                &device,
                &wgpu::SurfaceConfiguration {
                    usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                    format,
                    width: physical_size.width.max(1),
                    height: physical_size.height.max(1),
                    present_mode: wgpu::PresentMode::AutoVsync,
                    alpha_mode: wgpu::CompositeAlphaMode::Auto,
                    view_formats: vec![],
                    desired_maximum_frame_latency: 2,
                },
            );

            println!(
                "vp: {:?}; {:?}",
                viewport.physical_size(),
                viewport.logical_size()
            );
            // Initialize scene and GUI controls
            let scene = Scene::new(&device, format, &viewport.physical_size());
            let controls = Controls::new();

            // Initialize iced
            let mut debug = Box::new(Debug::new());
            let engine = Engine::new(&adapter, &device, &queue, format, None);
            let mut renderer = Renderer::new(&device, &engine, Font::default(), Pixels::from(16));

            let state =
                program::State::new(controls, viewport.logical_size(), &mut renderer, &mut debug);

            // You should change this if you want to render continuously
            event_loop.set_control_flow(ControlFlow::Poll);

            *self = Self::Ready {
                window,
                device,
                queue,
                surface,
                format,
                engine,
                renderer,
                scene,
                state,
                cursor_position: None,
                modifiers: ModifiersState::default(),
                clipboard,
                viewport,
                resized: false,
                debug,
            };
        }
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        _window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        let Self::Ready {
            window,
            device,
            queue,
            surface,
            format,
            engine,
            renderer,
            scene,
            state,
            viewport,
            cursor_position,
            modifiers,
            clipboard,
            resized,
            debug,
        } = self
        else {
            return;
        };

        match &event {
            WindowEvent::RedrawRequested => {
                window.request_redraw();
                if *resized {
                    let size = window.inner_size();
                    let physical_size = Size::new(size.width.max(1), size.height.max(1));

                    **viewport = Viewport::with_physical_size(physical_size, window.scale_factor());

                    // Update depth
                    if physical_size != scene.depth_texture.size {
                        scene.depth_texture =
                            crate::render::pipeline::Texture::create_depth_texture(
                                device,
                                &physical_size,
                            );
                    }

                    surface.configure(
                        device,
                        &wgpu::SurfaceConfiguration {
                            format: *format,
                            usage: wgpu::TextureUsages::RENDER_ATTACHMENT,
                            width: size.width.max(1),
                            height: size.height.max(1),
                            present_mode: wgpu::PresentMode::AutoVsync,
                            alpha_mode: wgpu::CompositeAlphaMode::Auto,
                            view_formats: vec![],
                            desired_maximum_frame_latency: 2,
                        },
                    );
                    *resized = false;
                }

                {
                    state.queue_message(PolybladeMessage::Tick(Instant::now()));
                    let state = &state.program().state;
                    let model = state.model.clone();
                    let render = state.render.clone();
                    let primitive = PolyhedronPrimitive::new(model, render);

                    let moments = primitive.moment_vertices();
                    if scene.moment_buf.count != moments.len() as u32 {
                        scene.moment_buf.resize(device, moments.len() as u32);

                        let shapes = primitive.shape_vertices();
                        scene.shape_buf.resize(device, shapes.len() as u32);
                        queue.write_buffer(&scene.shape_buf.raw, 0, bytemuck::cast_slice(&shapes));
                    }

                    queue.write_buffer(&scene.moment_buf.raw, 0, bytemuck::cast_slice(&moments));

                    let model = ModelUniforms {
                        model_mat: primitive.model.transform,
                        view_projection_mat: primitive
                            .render
                            .camera
                            .build_view_proj_mat(viewport.logical_size()),
                    };
                    let frag = FragUniforms {
                        line_thickness: primitive.render.line_thickness,
                        line_mode: 1.0,
                        ..Default::default()
                    };
                    scene.model_buf.write_data(queue, &model);
                    scene.frag_buf.write_data(queue, &frag);
                    window.request_redraw();
                }

                match surface.get_current_texture() {
                    Ok(frame) => {
                        let mut encoder =
                            device.create_command_encoder(&wgpu::CommandEncoderDescriptor {
                                label: None,
                            });

                        let program = state.program();

                        let view = frame
                            .texture
                            .create_view(&wgpu::TextureViewDescriptor::default());

                        {
                            // We clear the frame
                            let mut render_pass =
                                scene.clear(&view, &mut encoder, program.background_color());

                            // Ignore the whole first polygon if we're in schlegel mode
                            let starting_vertex = if program.state.render.schlegel {
                                match program.state.model.polyhedron.cycles[0].len() {
                                    3 => 3,
                                    4 => 6,
                                    n => n * 3,
                                }
                            } else {
                                0
                            } as u32;

                            // Draw the scene
                            scene.draw(starting_vertex, &mut render_pass);
                        }

                        // And then iced on top
                        renderer.present(
                            engine,
                            device,
                            queue,
                            &mut encoder,
                            None,
                            frame.texture.format(),
                            &view,
                            viewport,
                            &debug.overlay(),
                        );

                        // Then we submit the work
                        engine.submit(queue, encoder);
                        frame.present();

                        // Update the mouse cursor
                        window.set_cursor(iced_winit::conversion::mouse_interaction(
                            state.mouse_interaction(),
                        ));
                    }
                    Err(error) => match error {
                        wgpu::SurfaceError::OutOfMemory => {
                            panic!(
                                "Swapchain error: {error}. \
                                Rendering cannot continue."
                            )
                        }
                        _ => {
                            // Try rendering again next frame.
                            window.request_redraw();
                        }
                    },
                }
            }
            WindowEvent::CursorMoved { position, .. } => {
                *cursor_position = Some(*position);
            }
            WindowEvent::ModifiersChanged(new_modifiers) => {
                *modifiers = new_modifiers.state();
            }
            WindowEvent::Resized(_) => {
                *resized = true;
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
                            state.queue_message(message);
                        }
                    }
                }
            }
            _ => {}
        }

        // Map window event to iced event
        if let Some(event) =
            iced_winit::conversion::window_event(event, window.scale_factor(), *modifiers)
        {
            state.queue_event(event);
        }

        // If there are events pending
        if !state.is_queue_empty() {
            // We update iced
            let _ = state.update(
                viewport.logical_size(),
                cursor_position
                    .map(|p| conversion::cursor_position(p, viewport.scale_factor()))
                    .map(mouse::Cursor::Available)
                    .unwrap_or(mouse::Cursor::Unavailable),
                renderer,
                &Theme::Dark,
                &renderer::Style {
                    text_color: Color::WHITE,
                },
                clipboard,
                debug,
            );

            // and request a redraw
            window.request_redraw();
        }
    }
}
