use cgmath::{Matrix4, Rad};
use egui::{Checkbox, TopBottomPanel};
use egui_gl_glfw as egui_backend;

use std::time::Instant;

use egui_backend::egui::{vec2, Pos2, RawInput, Rect};
use egui_gl_glfw::glfw::Context;

const SCREEN_WIDTH: u32 = 800;
const SCREEN_HEIGHT: u32 = 900;
const VS_SRC: &str = include_str!("./shaders/basic.vert.glsl");
const FS_SRC: &str = include_str!("./shaders/basic.frag.glsl");

use polyblade::{prelude::*, verify};

fn main() {
    let mut glfw = glfw::init(glfw::fail_on_errors).unwrap();
    glfw.window_hint(glfw::WindowHint::ContextVersion(3, 3));
    glfw.window_hint(glfw::WindowHint::OpenGlProfile(
        glfw::OpenGlProfileHint::Core,
    ));
    glfw.window_hint(glfw::WindowHint::DoubleBuffer(true));
    glfw.window_hint(glfw::WindowHint::Resizable(false));
    glfw.window_hint(glfw::WindowHint::ScaleToMonitor(true));

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
        RawInput {
            screen_rect: Some(Rect::from_min_size(
                Pos2::new(0f32, 0f32),
                vec2(width as f32, height as f32) / native_pixels_per_point,
            )),
            ..Default::default()
        },
        native_pixels_per_point,
    );

    let start_time = Instant::now();

    let shader = Shader::new(VS_SRC, FS_SRC);
    let poly = Poly::new();
    let mut shape = PolyGraph::cube();
    //shape.recompute_qualities();
    let quit = false;
    let mut rotating = true;
    poly.prepare(&shape, &shader);

    while !window.should_close() {
        egui_input_state.input.time = Some(start_time.elapsed().as_secs_f64());
        egui_ctx.begin_frame(egui_input_state.input.take());
        egui_input_state.pixels_per_point = native_pixels_per_point;

        unsafe {
            verify!(gl::Enable(gl::DEPTH_TEST));
            gl::ClearColor(0.8, 0.8, 0.8, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }

        /*
        let c = Matrix4::look_at_lh(
            Point3::from_vec(vec3(0.0, 3.0, 0.0)),
            Point3::from_vec(Vector3::zero()),
            vec3(0.0, 0.0, 1.0),
        );
        */
        let c = Matrix4::new(
            2.4142134, 0.0, 0.0, 0.0, 0.0, 2.4142134, 0.0, 0.0, 0.0, 0.0, -1.020202, -1.0, 0.0,
            0.0, 3.878788, 4.0,
        );

        let time = start_time.elapsed().as_secs_f32();
        let model_rotation =
            Matrix4::from_angle_y(Rad(0.5 * time)) * Matrix4::from_angle_x(Rad(0.7 * time));

        shape.update();
        shader.activate();
        poly.draw(&shape);
        shader.set_mat4("model", &model_rotation);
        shader.set_mat4("projection", &c);

        unsafe {
            verify!(gl::Disable(gl::DEPTH_TEST));
        }
        TopBottomPanel::bottom("controls").show(&egui_ctx, |ui| {
            ui.heading(shape.name.clone());
            ui.add(Checkbox::new(&mut rotating, "rotating"));
            ui.horizontal(|ui| {
                ui.label("Seeds:");
                // Buttons to revert to platonic solids
                if ui.button("Tetrahedron").clicked() {
                    shape = PolyGraph::tetrahedron();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Cube").clicked() {
                    shape = PolyGraph::cube();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Octahedron").clicked() {
                    shape = PolyGraph::octahedron();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Dodecahedron").clicked() {
                    shape = PolyGraph::dodecahedron();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Icosahedron").clicked() {
                    shape = PolyGraph::icosahedron();
                    poly.prepare(&shape, &shader);
                }
            });

            ui.separator();

            ui.horizontal(|ui| {
                ui.label("Operations:");
                if ui.button("Truncate").clicked() {
                    shape.truncate();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Ambo").clicked() {
                    shape.ambo();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Bevel").clicked() {
                    shape.bevel();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Expand").clicked() {
                    shape.expand();
                    poly.prepare(&shape, &shader);
                }
                if ui.button("Snub").clicked() {
                    shape.snub();
                    poly.prepare(&shape, &shader);
                }
            });
        });

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
        painter.paint_and_update_textures(1.0, &clipped_shapes[..], &textures_delta);

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
