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
    use cgmath::{Vector3, Zero};
    use polyblade::prelude::PolyGraph;
    use three_d::core::{degrees, radians, vec3, ClearState, Context, Mat4, Program, RenderStates};
    use three_d::window::{FrameOutput, Window, WindowSettings};
    use three_d::Camera;

    let window = Window::new(WindowSettings {
        title: "polyblade".to_string(),
        #[cfg(not(target_arch = "wasm32"))]
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context: Context = window.gl();
    let mut shape = PolyGraph::cube();
    let program = Program::from_source(
        &context,
        include_str!("shaders/basic.vert"),
        include_str!("shaders/basic.frag"),
    )
    .unwrap();

    let mut camera = Camera::new_perspective(
        three_d::Viewport::new_at_origo(1, 1),
        vec3(0.0, 0.0, 4.0),
        Vector3::zero(),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    window.render_loop(move |frame_input| {
        shape.update();
        camera.set_viewport(frame_input.viewport);

        frame_input
            .screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .write(|| {
                let (positions, colors, barycentric) = shape.triangle_buffers(&context);
                let time = frame_input.accumulated_time as f32;
                let model = Mat4::from_angle_y(radians(0.001 * time))
                    * Mat4::from_angle_x(radians(0.0004 * time));
                program.use_uniform("model", model);
                program.use_uniform("projection", camera.projection() * camera.view());
                program.use_vertex_attribute("position", &positions);
                program.use_vertex_attribute("color", &colors);
                program.use_vertex_attribute("barycentric", &barycentric);
                program.draw_arrays(
                    RenderStates::default(),
                    frame_input.viewport,
                    positions.vertex_count(),
                );
            });

        FrameOutput::default()
    });
}
