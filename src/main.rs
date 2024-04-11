use egui::mutex::Mutex;
use egui_glow::EguiGlow;
use egui_winit::winit::{
    event::{Event, StartCause, WindowEvent},
    event_loop::{ControlFlow, EventLoopBuilder},
};
use glow::HasContext;
use std::{sync::Arc, time::Instant};

use polyblade::prelude::{create_display, UserEvent};
const shader_version: &str = "#version 410";

fn main() {
    unsafe {
        let event_loop = EventLoopBuilder::<UserEvent>::with_user_event()
            .build()
            .unwrap();
        let (gl_window, gl) = create_display(&event_loop);
        let gl = Arc::new(gl);

        let mut egui_glow = EguiGlow::new(&event_loop, gl.clone(), None, None);

        let event_loop_proxy = Mutex::new(event_loop.create_proxy());
        egui_glow
            .egui_ctx
            .set_request_repaint_callback(move |info| {
                event_loop_proxy
                    .lock()
                    .send_event(UserEvent::Redraw(info.delay))
                    .expect("Cannot send event");
            });

        let mut repaint_delay = std::time::Duration::MAX;

        let vertex_array = gl
            .create_vertex_array()
            .expect("Cannot create vertex array");
        gl.bind_vertex_array(Some(vertex_array));

        let program = gl.create_program().expect("Cannot create program");

        let (vertex_shader_source, fragment_shader_source) = (
            r#"const vec2 verts[3] = vec2[3](
                vec2(0.5f, 1.0f),
                vec2(0.0f, 0.0f),
                vec2(1.0f, 0.0f)
            );
            out vec2 vert;
            void main() {
                vert = verts[gl_VertexID];
                gl_Position = vec4(vert - 0.5, 0.0, 1.0);
            }"#,
            r#"precision mediump float;
            in vec2 vert;
            out vec4 color;
            void main() {
                color = vec4(vert, 0.5, 1.0);
            }"#,
        );

        let shader_sources = [
            (glow::VERTEX_SHADER, vertex_shader_source),
            (glow::FRAGMENT_SHADER, fragment_shader_source),
        ];

        let mut shaders = Vec::with_capacity(shader_sources.len());

        for (shader_type, shader_source) in shader_sources.iter() {
            let shader = gl
                .create_shader(*shader_type)
                .expect("Cannot create shader");
            gl.shader_source(shader, &format!("{}\n{}", shader_version, shader_source));
            gl.compile_shader(shader);
            if !gl.get_shader_compile_status(shader) {
                panic!("{}", gl.get_shader_info_log(shader));
            }
            gl.attach_shader(program, shader);
            shaders.push(shader);
        }

        gl.link_program(program);
        if !gl.get_program_link_status(program) {
            panic!("{}", gl.get_program_info_log(program));
        }

        for shader in shaders {
            gl.detach_shader(program, shader);
            gl.delete_shader(shader);
        }

        gl.use_program(Some(program));
        gl.clear_color(0.1, 0.2, 0.3, 1.0);

        let _ = event_loop.run(move |event, event_loop_window_target| {
            match event {
                Event::WindowEvent { event, .. } => {
                    match event {
                        WindowEvent::CloseRequested | WindowEvent::Destroyed => {
                            event_loop_window_target.exit();
                            return;
                        }
                        WindowEvent::RedrawRequested => {
                            let mut quit = false;

                            /*
                            egui_glow.run(gl_window.window(), |egui_ctx| {
                                egui::TopBottomPanel::bottom("my_side_panel").show(
                                    egui_ctx,
                                    |ui| {
                                        ui.heading("Hello World!");
                                        if ui.button("big chil").clicked() {
                                            quit = true;
                                        }
                                    },
                                );
                            });
                            */

                            if quit {
                                event_loop_window_target.exit();
                            } else {
                                event_loop_window_target.set_control_flow(
                                    if repaint_delay.is_zero() {
                                        gl_window.window().request_redraw();
                                        ControlFlow::Poll
                                    } else if let Some(repaint_after_instant) =
                                        Instant::now().checked_add(repaint_delay)
                                    {
                                        ControlFlow::WaitUntil(repaint_after_instant)
                                    } else {
                                        ControlFlow::Wait
                                    },
                                );
                            }

                            gl.clear_color(0.4, 0.4, 0.4, 1.0);
                            gl.clear(glow::COLOR_BUFFER_BIT);

                            // draw things behind egui here

                            //egui_glow.paint(gl_window.window());
                            gl.draw_arrays(glow::TRIANGLES, 0, 3);

                            // draw things on top of egui here

                            gl_window.swap_buffers().unwrap();
                            gl_window.window().set_visible(true);
                        }
                        WindowEvent::Resized(physical_size) => {
                            gl_window.resize(physical_size);
                        }
                        _ => {}
                    }
                    let event_response = egui_glow.on_window_event(gl_window.window(), &event);
                    if event_response.repaint {
                        gl_window.window().request_redraw();
                    }
                }
                Event::UserEvent(UserEvent::Redraw(delay)) => {
                    repaint_delay = delay;
                }
                Event::LoopExiting => {
                    egui_glow.destroy();
                }
                Event::NewEvents(StartCause::ResumeTimeReached { .. }) => {
                    gl_window.window().request_redraw();
                }

                _ => (),
            }
        });
    }
}
