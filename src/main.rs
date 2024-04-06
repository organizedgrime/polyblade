use egui::{Checkbox, TopBottomPanel};
use egui_gl_glfw as egui_backend;

use std::time::Instant;

use egui_backend::egui::{vec2, Color32, Image, Pos2, Rect};
use egui_gl_glfw::glfw::Context;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 600;
const PIC_WIDTH: i32 = 320;
const PIC_HEIGHT: i32 = 192;

use polyblade::prelude::*;

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 2));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));

    let (mut window, events) = glfw
        .create_window(
            SCREEN_WIDTH,
            SCREEN_HEIGHT,
            "Egui in GLFW!",
            glfw::WindowMode::Windowed,
        )
        .expect("Failed to create GLFW window.");

    window.set_char_polling(true);
    window.set_cursor_pos_polling(true);
    window.set_key_polling(true);
    window.set_mouse_button_polling(true);
    window.make_current();
    glfw.set_swap_interval(glfw::SwapInterval::Sync(1));

    gl::load_with(|symbol| window.get_proc_address(symbol) as *const _);

    let mut painter = egui_backend::Painter::new(&mut window);
    let egui_ctx = egui::Context::default();

    let (width, height) = window.get_framebuffer_size();
    let native_pixels_per_point = window.get_content_scale().0;

    let mut egui_input_state = egui_backend::EguiInputState::new(
        egui::RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            ..Default::default()
        },
        native_pixels_per_point,
    );

    let start_time = Instant::now();
    let srgba = vec![Color32::BLACK; (PIC_HEIGHT * PIC_WIDTH) as usize];

    let plot_tex_id = painter.new_user_texture(
        (PIC_WIDTH as usize, PIC_HEIGHT as usize),
        &srgba,
        egui::TextureFilter::Linear,
    );

    let mut sine_shift = 0f32;
    let mut amplitude = 50f32;
    let mut test_str =
        "A text box to write in. Cut, copy, paste commands are available.".to_owned();

    let triangle = Triangle::new();
    let mut quit = false;
    let mut rotating = true;

    while !window.should_close() {
        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());
        egui_input_state.pixels_per_point = native_pixels_per_point;

        unsafe {
            gl::ClearColor(0.455, 0.302, 0.663, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        triangle.draw();

        let mut srgba: Vec<Color32> = Vec::new();
        let mut angle = 0f32;

        for y in 0..PIC_HEIGHT {
            for x in 0..PIC_WIDTH {
                srgba.push(Color32::BLACK);
                if y == PIC_HEIGHT - 1 {
                    let y = amplitude * (angle * std::f32::consts::PI / 180f32 + sine_shift).sin();
                    let y = PIC_HEIGHT as f32 / 2f32 - y;
                    srgba[(y as i32 * PIC_WIDTH + x) as usize] = Color32::YELLOW;
                    angle += 360f32 / PIC_WIDTH as f32;
                }
            }
        }
        sine_shift += 0.1f32;

        //This updates the previously initialized texture with new data.
        //If we weren't updating the texture, this call wouldn't be required.
        painter.update_user_texture_data(&plot_tex_id, &srgba);

        TopBottomPanel::bottom("dog").show(&egui_ctx, |ui| {
            //ui.heading(shape.name.clone());
            ui.heading("Ctabas");
            ui.add(Checkbox::new(&mut rotating, "rotating"));
            ui.horizontal(|ui| {
                ui.label("Seeds:");
                // Buttons to revert to platonic solids
                if ui.button("Tetrahedron").clicked() {
                    //shape = PolyGraph::tetrahedron();
                }
                if ui.button("Cube").clicked() {
                    //shape = PolyGraph::cube();
                }
                if ui.button("Octahedron").clicked() {
                    //shape = PolyGraph::octahedron();
                }
                if ui.button("Dodecahedron").clicked() {
                    //shape = PolyGraph::dodecahedron();
                }
                if ui.button("Icosahedron").clicked() {
                    //shape = PolyGraph::icosahedron();
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Operations:");
                //
                if ui.button("s0").clicked() {
                    //shape.split_vertex(&0);
                    //shape.recompute_qualities();
                }
                if ui.button("Truncate").clicked() {
                    //shape.truncate();
                }
                if ui.button("Ambo").clicked() {
                    //shape.ambo();
                }
                if ui.button("Bevel").clicked() {
                    //shape.bevel();
                }
                if ui.button("Expand").clicked() {
                    //shape.expand();
                }
                if ui.button("Snub").clicked() {
                    //shape.snub();
                }
            });
        });
        //egui_ctx.top
        let egui::FullOutput {
            platform_output,
            textures_delta,
            shapes,
            pixels_per_point,
            viewport_output: _,
        } = egui_ctx.end_frame();

        //Handle cut, copy text from egui
        if !platform_output.copied_text.is_empty() {
            egui_backend::copy_to_clipboard(&mut egui_input_state, platform_output.copied_text);
        }

        //Note: passing a bg_color to paint_jobs will clear any previously drawn stuff.
        //Use this only if egui is being used for all drawing and you aren't mixing your own Open GL
        //drawing calls with it.
        //Since we are custom drawing an OpenGL Triangle we don't need egui to clear the background.

        let clipped_shapes = egui_ctx.tessellate(shapes, pixels_per_point);
        painter.paint_and_update_textures(1.0, &clipped_shapes, &textures_delta);

        for (_, event) in glfw::flush_messages(&events) {
            match event {
                glfw::WindowEvent::Close => window.set_should_close(true),
                _ => {
                    egui_backend::handle_event(event, &mut egui_input_state);
                }
            }
        }
        window.swap_buffers();
        glfw.poll_events();

        if quit {
            break;
        }
    }
}
