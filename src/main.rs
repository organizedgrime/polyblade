mod bones;
mod render;
use render::Runner;

use iced_winit::winit;
use winit::event_loop::EventLoop;

#[cfg(target_arch = "wasm32")]
pub use iced::time::Instant;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

#[rustfmt::skip::macros(menu_bar)]
pub fn main() -> Result<(), winit::error::EventLoopError> {
    #[cfg(target_arch = "wasm32")]
    {
        console_log::init().expect("Initialize logger");
        std::panic::set_hook(Box::new(console_error_panic_hook::hook));
    }

    #[cfg(not(target_arch = "wasm32"))]
    tracing_subscriber::fmt::init();

    let event_loop = EventLoop::new()?;

    let mut runner = Runner::Loading;
    event_loop.run_app(&mut runner)
}
