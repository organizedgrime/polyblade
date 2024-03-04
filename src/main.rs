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
    use paper_blade::prelude::{Polyhedron, WindowScene};
    use std::collections::HashMap;
    use three_d::renderer::*;

    //let shape = Polyhedron::dodecahedron();
    // shape.render_form();
    //shape.render_schlegel();
    let mut scenes = HashMap::new();

    let event_loop = winit::event_loop::EventLoop::new();

    let camera1 = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(0.0, 0.0, 6.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    let scene1 = WindowScene::new("model", &event_loop, camera1, Srgba::WHITE, "basic");
    scenes.insert(scene1.window.id(), scene1);

    let camera2 = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(0.0, 0.0, 6.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(170.0),
        0.1,
        10.0,
    );
    let _scene2 = WindowScene::new("schlegel", &event_loop, camera2, Srgba::WHITE, "basic");
    //scenes.insert(scene2.window.id(), scene2);

    let mut shape = Polyhedron::dodecahedron();
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

                if &scene.title == "model" {
                    shape.render_model(scene, &frame_input);
                } else {
                    //shape.render_schlegel(scene, &frame_input);
                    //Polyhedron::dodecahedron().render_model(scene, &frame_input);
                }

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
}
