mod bones;
mod render;
use iced::futures::executor::block_on;
use render::*;

use iced_winit::winit;
use winit::event_loop::EventLoop;

#[cfg(target_arch = "wasm32")]
pub use iced::time::Instant;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

pub async fn run() -> Result<(), winit::error::EventLoopError> {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }
    #[cfg(not(target_arch = "wasm32"))]
    {
        tracing_subscriber::fmt::init();
    }

    let event_loop = EventLoop::new()?;
    let window = Arc::new(
        // winit has diverged from WebGPU standards on window creation
        #[allow(deprecated)]
        event_loop
            .create_window(winit::window::WindowAttributes::default())
            .expect("Create window"),
    );

    #[cfg(target_arch = "wasm32")]
    {
        // Winit prevents sizing with CSS, so we have to set
        // the size manually when on web.
        use winit::dpi::PhysicalSize;

        use winit::platform::web::WindowExtWebSys;
        web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| {
                let dst = doc.get_element_by_id("polyblade")?;
                let canvas = web_sys::Element::from(window.canvas()?);
                dst.append_child(&canvas).ok()?;
                Some(())
            })
            .expect("Couldn't append canvas to document body.");

        let _ = window.request_inner_size(PhysicalSize::new(1280, 720));
    }

    let mut runner = App {
        graphics: Graphics::new(&window).await,
        data: None,
        surface_configured: false,
    };
    event_loop.run_app(&mut runner)
}

pub fn main() -> Result<(), winit::error::EventLoopError> {
    block_on(run())
}
