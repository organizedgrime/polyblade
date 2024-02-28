/*
use three_d::*;


pub fn main() {
    let window = Window::new(WindowSettings {
        title: "Shapes 2D!".to_string(),
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();
    let context = window.gl();
    let scale_factor = window.device_pixel_ratio();
    let (width, height) = window.size();

    let mut rectangle = Gm::new(
        Rectangle::new(
            &context,
            vec2(200.0, 200.0) * scale_factor,
            degrees(45.0),
            100.0 * scale_factor,
            200.0 * scale_factor,
        ),
        ColorMaterial {
            color: Srgba::RED,
            ..Default::default()
        },
    );
    let mut circle = Gm::new(
        Circle::new(
            &context,
            vec2(500.0, 500.0) * scale_factor,
            200.0 * scale_factor,
        ),
        ColorMaterial {
            color: Srgba::BLUE,
            ..Default::default()
        },
    );
    let mut line = Gm::new(
        Line::new(
            &context,
            vec2(0.0, 0.0) * scale_factor,
            vec2(width as f32, height as f32) * scale_factor,
            5.0 * scale_factor,
        ),
        ColorMaterial {
            color: Srgba::GREEN,
            ..Default::default()
        },
    );

    window.render_loop(move |frame_input| {
        for event in frame_input.events.iter() {
            if let Event::MousePress {
                button,
                position,
                modifiers,
                ..
            } = *event
            {
                if button == MouseButton::Left && !modifiers.ctrl {
                    rectangle.set_center(position);
                }
                if button == MouseButton::Right && !modifiers.ctrl {
                    circle.set_center(position);
                }
                if button == MouseButton::Left && modifiers.ctrl {
                    let ep = line.end_point1();
                    line.set_endpoints(position, ep);
                }
                if button == MouseButton::Right && modifiers.ctrl {
                    let ep = line.end_point0();
                    line.set_endpoints(ep, position);
                }
            }
        }
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(
                &Camera::new_2d(frame_input.viewport),
                line.into_iter().chain(&rectangle).chain(&circle),
                &[],
            );

        FrameOutput::default()
    });
}
*/

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
use paper_blade::prelude::*;

#[cfg(not(target_arch = "wasm32"))]
pub fn main() {
    use three_d::*;
    let shape = Polyhedron::dodecahedron();
    // shape.render_form();
    shape.render_schlegel();
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
