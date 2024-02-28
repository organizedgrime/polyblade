use std::sync::Arc;

use three_d::{renderer::*, FrameInputGenerator, WindowedContext};
use winit::{
    event_loop::EventLoop,
    window::{Window, WindowId},
};

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
    pub program: Option<Program>,
}

impl WindowScene {
    pub fn new(
        event_loop: &EventLoop<()>,
        camera: Camera,
        background: Srgba,
        program: Option<Program>,
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

        Self {
            //            event_loop,
            window,
            context,
            frame_input_generator,
            camera,
            background,
            program,
        }
    }

    pub fn render(&self) {}
}

/*
impl Renderable for Scene {
    type SceneData = ();

    // Render things
    fn render(&self, data: Self::SceneData) {
        self.context.make_current().unwrap();
        let frame_input = self.frame_input_generator.generate(&self.context);
        self.camera.set_viewport(frame_input.viewport);

        //        self.model.animate(frame_input.accumulated_time as f32);
        frame_input
            .screen()
            .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
            .render(&self.camera, &self.model, &[]);

        self.context.swap_buffers().unwrap();
        control_flow.set_poll();
        window.request_redraw();
    }
}
*/
