use std::{fs::File, io::Read, sync::Arc};

use three_d::{renderer::*, FrameInput, FrameInputGenerator, WindowedContext};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowId},
};

use crate::prelude::Polyhedron;

pub struct WindowScene {
    // Window stuff
    //event_loop: Arc<EventLoop<()>>,
    pub window: Window,
    pub context: WindowedContext,
    pub frame_input_generator: FrameInputGenerator,

    // GL stuff
    // Need a camera
    pub camera: Camera,
    pub background: Srgba,
    // Optionally, vertex and fragment shaders
    pub program_name: String,
    pub program: Program,
}

impl WindowScene {
    pub fn new(
        event_loop: &EventLoop<()>,
        camera: Camera,
        background: Srgba,
        program_name: &str,
    ) -> Self {
        #[cfg(not(target_arch = "wasm32"))]
        let window_builder = winit::window::WindowBuilder::new()
            .with_title("winit window")
            .with_min_inner_size(winit::dpi::LogicalSize::new(720, 720))
            .with_inner_size(winit::dpi::LogicalSize::new(720, 720))
            .with_position(winit::dpi::LogicalPosition::new(300, 100));
        #[cfg(target_arch = "wasm32")]
        let window_builder = {
            use wasm_bindgen::JsCast;
            use winit::platform::web::WindowBuilderExtWebSys;
            winit::window::WindowBuilder::new()
                .with_canvas(Some(
                    web_sys::window()
                        .unwrap()
                        .document()
                        .unwrap()
                        .get_elements_by_tag_name("canvas")
                        .item(i)
                        .unwrap()
                        .dyn_into::<web_sys::HtmlCanvasElement>()
                        .unwrap(),
                ))
                .with_inner_size(winit::dpi::LogicalSize::new(720, 720))
                .with_prevent_default(true)
        };

        // Construct the new window from the builder
        let window = window_builder.build(&event_loop).unwrap();
        // Create a context for this window
        let context = WindowedContext::from_winit_window(
            &window,
            three_d::SurfaceSettings {
                vsync: false, // Wayland hangs in swap_buffers when one window is minimized or occluded
                ..three_d::SurfaceSettings::default()
            },
        )
        .unwrap();

        let frame_input_generator = three_d::FrameInputGenerator::from_winit_window(&window);

        let program: Program = {
            let mut vertex_shader = String::new();
            let mut fragment_shader = String::new();
            File::open(&format!("src/shaders/{}.vert", program_name))
                .unwrap()
                .read_to_string(&mut vertex_shader)
                .unwrap();
            File::open(&format!("src/shaders/{}.frag", program_name))
                .unwrap()
                .read_to_string(&mut fragment_shader)
                .unwrap();

            Program::from_source(&context, &vertex_shader, &fragment_shader).unwrap()
        };

        Self {
            //            event_loop,
            window,
            context,
            frame_input_generator,
            camera,
            background,
            program_name: String::from(program_name),
            program,
        }
    }
}
