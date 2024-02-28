// Entry point for wasm
#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[cfg(target_arch = "wasm32")]
#[wasm_bindgen(start)]
pub async fn start() -> Result<(), JsValue> {
    console_log::init_with_level(log::Level::Debug).unwrap();

    use log::info;
    info!("Logging works!");

    std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    main::run().await;
    Ok(())
}

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use crate::*;
    use paper_blade::prelude::WindowScene;
    use std::{collections::HashMap, sync::Arc};
    use three_d::{renderer::*, WindowedContext};

    //let shape = Polyhedron::dodecahedron();
    // shape.render_form();
    //shape.render_schlegel();
    let mut scenes = HashMap::new();

    let event_loop = winit::event_loop::EventLoop::new();

    let camera = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(0.0, 0.0, 2.0 + 1 as f32 * 4.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    let scene1 = WindowScene::new(&event_loop, camera.clone(), Srgba::WHITE, Some("basic"));
    scenes.insert(scene1.window.id(), scene1);
    let scene2 = WindowScene::new(&event_loop, camera, Srgba::RED, None);
    scenes.insert(scene2.window.id(), scene2);

    event_loop.run(move |event, _, control_flow| match &event {
        winit::event::Event::MainEventsCleared => {
            for (_, scene) in scenes.iter() {
                scene.window.request_redraw();
            }
        }
        winit::event::Event::RedrawRequested(window_id) => {
            if let Some(scene) = scenes.get_mut(window_id) {
                // Open
                scene.context.make_current().unwrap();
                let frame_input = scene.frame_input_generator.generate(&scene.context);
                scene.camera.set_viewport(frame_input.viewport);
                let color = scene.background.to_linear_srgb();
                frame_input.screen().clear(ClearState::color_and_depth(
                    color.x, color.y, color.z, 1.0, 1.0,
                ));

                // Render
                scene.render(frame_input);

                //frame_input.screen()

                // Close
                scene.context.swap_buffers().unwrap();
                control_flow.set_poll();
                scene.window.request_redraw();
            }
        }
        winit::event::Event::WindowEvent { event, window_id } => {
            if let Some(scene) = scenes.get_mut(window_id) {
                scene.frame_input_generator.handle_winit_window_event(event);
                match event {
                    winit::event::WindowEvent::Resized(physical_size) => {
                        scene.context.resize(*physical_size);
                    }
                    winit::event::WindowEvent::ScaleFactorChanged { new_inner_size, .. } => {
                        scene.context.resize(**new_inner_size);
                    }
                    winit::event::WindowEvent::CloseRequested => {
                        if let Some(scene) = scenes.get_mut(window_id) {
                            scene.context.make_current().unwrap();
                        }

                        scenes.remove(window_id);

                        if scenes.is_empty() {
                            control_flow.set_exit();
                        }
                    }
                    _ => (),
                }
            }
        }
        _ => {}
    });

    /*
        window.render_loop(move |frame_input| {
            camera.set_viewport(frame_input.viewport);
            /*
            frame_input
                .screen()
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(
                    &Camera::new_2d(frame_input.viewport),
                    shape.render_schlegel(&context).into_iter(),
                    &[],
                );

                */
            frame_input
                .screen()
                // Clear the color and depth of the screen render target
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .write(|| {
                    let time = frame_input.accumulated_time as f32;
                    program.use_uniform("model", Mat4::from_angle_y(degrees(72.0 / 2.0)));
                    program.use_uniform("viewProjection", camera.projection() * camera.view());
                    shape.render_form(&program, &context, frame_input.viewport);
                });
            FrameOutput::default()
        });
    */
}
