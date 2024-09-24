use iced::{Application as _, Settings};

use iced_wgpu::graphics::Viewport;
use iced_wgpu::{wgpu, Engine, Renderer};
use iced_winit::conversion;
use iced_winit::core::mouse;
use iced_winit::core::renderer;
use iced_winit::core::{Color, Font, Pixels, Size, Theme};
use iced_winit::futures;
use iced_winit::runtime::program;
use iced_winit::runtime::Debug;
use iced_winit::winit;
use iced_winit::Clipboard;

use winit::{
    event::WindowEvent,
    event_loop::{ControlFlow, EventLoop},
    keyboard::ModifiersState,
};

use crate::render::controls::Controls;

#[cfg(target_arch = "wasm32")]
pub use iced::time::Instant;
use std::sync::Arc;
#[cfg(not(target_arch = "wasm32"))]
pub use std::time::Instant;

pub enum Runner {
    Loading,
    Ready {
        window: Arc<winit::window::Window>,
        device: wgpu::Device,
        queue: wgpu::Queue,
        surface: wgpu::Surface<'static>,
        format: wgpu::TextureFormat,
        engine: Engine,
        renderer: Renderer,
        //scene: Scene,
        state: program::State<Controls>,
        cursor_position: Option<winit::dpi::PhysicalPosition<f64>>,
        clipboard: Clipboard,
        viewport: Viewport,
        modifiers: ModifiersState,
        resized: bool,
        debug: Debug,
    },
}

impl winit::application::ApplicationHandler for Runner {
    fn resumed(&mut self, event_loop: &winit::event_loop::ActiveEventLoop) {
        todo!()
    }

    fn window_event(
        &mut self,
        event_loop: &winit::event_loop::ActiveEventLoop,
        window_id: winit::window::WindowId,
        event: WindowEvent,
    ) {
        todo!()
    }
}
