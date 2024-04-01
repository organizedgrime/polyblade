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
    use three_d::{Camera, OrbitControl, Viewport};

    let window = Window::new(WindowSettings {
        title: "polyblade".to_string(),
        #[cfg(not(target_arch = "wasm32"))]
        max_size: Some((600, 600)),
        ..Default::default()
    })
    .unwrap();

    // Get the graphics context from the window
    let context: Context = window.gl();
    let mut gui = three_d::GUI::new(&context);
    let program = Program::from_source(
        &context,
        include_str!("shaders/basic.vert"),
        include_str!("shaders/basic.frag"),
    )
    .unwrap();

    let mut camera = Camera::new_perspective(
        Viewport::new_at_origo(1, 1),
        vec3(0.0, 0.0, 4.0),
        Vector3::zero(),
        vec3(0.0, 1.0, 0.0),
        degrees(45.0),
        0.1,
        10.0,
    );
    let mut control = OrbitControl::new(vec3(0.0, 0.0, 0.0), 1.0, 1000.0);

    let mut shape = PolyGraph::cube();
    let mut rotating = true;
    window.render_loop(move |mut frame_input| {
        shape.update();

        // Gui panel to control the number of cubes and whether or not instancing is turned on.
        let mut panel_height = 0.0;
        gui.update(
            &mut frame_input.events,
            frame_input.accumulated_time,
            frame_input.viewport,
            frame_input.device_pixel_ratio,
            |gui_context| {
                use three_d::egui::*;
                TopBottomPanel::top("controls").show(gui_context, |ui| {
                    use three_d::egui::*;
                    ui.heading("Debug Panel");
                    ui.add(Button::new("cube"));
                    ui.add(Checkbox::new(&mut rotating, "rotating"));
                    ui.add(Label::new(
                        "Increase the cube count until the cubes don't rotate \
                                       smoothly anymore, then toggle on instancing. The rotations \
                                       should become smooth again.",
                    ));

                    // Buttons to revert to platonic solids
                    if ui.button("T").clicked() {
                        shape = PolyGraph::tetrahedron();
                    }
                    if ui.button("C").clicked() {
                        shape = PolyGraph::cube();
                    }
                    if ui.button("O").clicked() {
                        shape = PolyGraph::octahedron();
                    }
                    if ui.button("D").clicked() {
                        shape = PolyGraph::dodecahedron();
                    }
                    if ui.button("I").clicked() {
                        shape = PolyGraph::icosahedron();
                    }

                    //
                    if ui.button("t").clicked() {
                        shape.truncate();
                    }
                    if ui.button("a").clicked() {
                        shape.ambo();
                    }
                });
                panel_height = gui_context.used_rect().height();
            },
        );

        let viewport = Viewport {
            x: 0,
            y: (panel_height * frame_input.device_pixel_ratio) as i32,
            width: frame_input.viewport.width,
            height: frame_input.viewport.height
                - (panel_height * frame_input.device_pixel_ratio) as u32,
        };
        camera.set_viewport(viewport);

        // Camera control must be after the gui update.
        control.handle_events(&mut camera, &mut frame_input.events);

        camera.set_viewport(frame_input.viewport);

        frame_input
            .screen()
            // Clear the color and depth of the screen render target
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .write(|| {
                let (positions, colors, barycentric) = shape.triangle_buffers(&context);
                let time = frame_input.accumulated_time as f32;
                let model = if rotating {
                    Mat4::from_angle_y(radians(0.001 * time))
                        * Mat4::from_angle_x(radians(0.0004 * time))
                } else {
                    Mat4::from_angle_y(radians(0.0)) * Mat4::from_angle_x(radians(0.0))
                };

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
            })
            .write(|| gui.render());

        FrameOutput::default()
    });
}
