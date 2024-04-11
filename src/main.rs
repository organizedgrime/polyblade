use egui_winit::winit;
use polyblade::prelude::{create_display, UserEvent};

fn main() {
    let mut clear_color = [0.1, 0.1, 0.1];

    let event_loop = winit::event_loop::EventLoopBuilder::<UserEvent>::with_user_event()
        .build()
        .unwrap();
    let (gl_window, gl) = create_display(&event_loop);
    let gl = std::sync::Arc::new(gl);

    let mut egui_glow = egui_glow::EguiGlow::new(&event_loop, gl.clone(), None, None);

    let event_loop_proxy = egui::mutex::Mutex::new(event_loop.create_proxy());
    egui_glow
        .egui_ctx
        .set_request_repaint_callback(move |info| {
            event_loop_proxy
                .lock()
                .send_event(UserEvent::Redraw(info.delay))
                .expect("Cannot send event");
        });

    let mut repaint_delay = std::time::Duration::MAX;

    let _ = event_loop.run(move |event, event_loop_window_target| {
        let mut redraw = || {
            let mut quit = false;

            egui_glow.run(gl_window.window(), |egui_ctx| {
                egui::SidePanel::left("my_side_panel").show(egui_ctx, |ui| {
                    ui.heading("Hello World!");
                    if ui.button("Quit").clicked() {
                        quit = true;
                    }
                    ui.color_edit_button_rgb(&mut clear_color);
                });
            });

            if quit {
                event_loop_window_target.exit();
            } else {
                event_loop_window_target.set_control_flow(if repaint_delay.is_zero() {
                    gl_window.window().request_redraw();
                    winit::event_loop::ControlFlow::Poll
                } else if let Some(repaint_after_instant) =
                    std::time::Instant::now().checked_add(repaint_delay)
                {
                    winit::event_loop::ControlFlow::WaitUntil(repaint_after_instant)
                } else {
                    winit::event_loop::ControlFlow::Wait
                });
            }

            {
                unsafe {
                    use glow::HasContext as _;
                    gl.clear_color(clear_color[0], clear_color[1], clear_color[2], 1.0);
                    gl.clear(glow::COLOR_BUFFER_BIT);
                }

                // draw things behind egui here

                egui_glow.paint(gl_window.window());

                // draw things on top of egui here

                gl_window.swap_buffers().unwrap();
                gl_window.window().set_visible(true);
            }
        };

        match event {
            winit::event::Event::WindowEvent { event, .. } => {
                use winit::event::WindowEvent;
                if matches!(event, WindowEvent::CloseRequested | WindowEvent::Destroyed) {
                    event_loop_window_target.exit();
                    return;
                }

                if matches!(event, WindowEvent::RedrawRequested) {
                    redraw();
                    return;
                }

                if let winit::event::WindowEvent::Resized(physical_size) = &event {
                    gl_window.resize(*physical_size);
                }

                let event_response = egui_glow.on_window_event(gl_window.window(), &event);

                if event_response.repaint {
                    gl_window.window().request_redraw();
                }
            }

            winit::event::Event::UserEvent(UserEvent::Redraw(delay)) => {
                repaint_delay = delay;
            }
            winit::event::Event::LoopExiting => {
                egui_glow.destroy();
            }
            winit::event::Event::NewEvents(winit::event::StartCause::ResumeTimeReached {
                ..
            }) => {
                gl_window.window().request_redraw();
            }

            _ => (),
        }
    });
}
