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

    // Create a window (a canvas on web)
    let window = Window::new(WindowSettings {
        title: "Core Triangle!".to_string(),
        #[cfg(not(target_arch = "wasm32"))]
        max_size: Some((1280, 720)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context: Context = window.gl();

    // Define triangle vertices and colors
    let positions = Polyhedron::cube().render(&context);

    let colors = VertexBuffer::new_with_data(
        &context,
        &[
            Srgba::RED.to_linear_srgb(),   // bottom right
            Srgba::GREEN.to_linear_srgb(), // bottom left
            Srgba::BLUE.to_linear_srgb(),  // top
        ],
    );

    let program = Program::from_source(
        &context,
        include_str!("triangle.vert"),
        include_str!("triangle.frag"),
    )
    .unwrap();

    let mut camera = Camera::new_perspective(
        window.viewport(),
        vec3(0.0, 0.0, 2.0),
        vec3(0.0, 0.0, 0.0),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );

    window.render_loop(move |frame_input| {
        camera.set_viewport(frame_input.viewport);

        frame_input
            .screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .write(|| {
                let time = frame_input.accumulated_time as f32;
                program.use_uniform("model", Mat4::from_angle_y(radians(time * 0.005)));
                program.use_uniform("viewProjection", camera.projection() * camera.view());
                program.use_vertex_attribute("position", &positions);
                program.use_vertex_attribute("color", &colors);
                program.draw_arrays(
                    RenderStates::default(),
                    frame_input.viewport,
                    positions.vertex_count(),
                );
            });

        FrameOutput::default()
    });
}
