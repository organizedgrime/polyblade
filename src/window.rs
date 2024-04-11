#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release
#![allow(unsafe_code)]
#![allow(clippy::arc_with_non_send_sync)] // glow::Context was accidentally non-Sync in glow 0.13, but that will be fixed in future releases of glow: https://github.com/grovesNL/glow/commit/c4a5f7151b9b4bbb380faa06ec27415235d1bf7e

use std::{num::NonZeroU32, time::Duration};

use egui_winit::winit;
use glutin::{
    config::{ConfigTemplateBuilder, GlConfig},
    context::{ContextApi, ContextAttributesBuilder, NotCurrentGlContext, Version},
    display::{GetGlDisplay, GlDisplay},
    prelude::GlSurface,
    surface::{SurfaceAttributesBuilder, WindowSurface},
};
use glutin_winit::DisplayBuilder;
use rwh_05::HasRawWindowHandle;
use winit::{dpi::LogicalSize, event_loop::EventLoopWindowTarget, window::WindowBuilder};

/// The majority of `GlutinWindowContext` is taken from `eframe`
pub struct GlutinWindowContext {
    window: winit::window::Window,
    gl_context: glutin::context::PossiblyCurrentContext,
    gl_display: glutin::display::Display,
    gl_surface: glutin::surface::Surface<glutin::surface::WindowSurface>,
}

impl GlutinWindowContext {
    // refactor this function to use `glutin-winit` crate eventually.
    // preferably add android support at the same time.
    #[allow(unsafe_code)]
    unsafe fn new(event_loop: &EventLoopWindowTarget<UserEvent>) -> Self {
        let winit_window_builder = WindowBuilder::new()
            .with_resizable(true)
            .with_inner_size(LogicalSize {
                width: 800.0,
                height: 600.0,
            })
            .with_title("egui_glow example"); // Keep hidden until we've painted something. See https://github.com/emilk/egui/pull/2279

        let template = ConfigTemplateBuilder::new().with_transparency(false);
        //.prefer_hardware_accelerated(None)
        //.with_depth_size(0)
        //.with_stencil_size(0)

        println!("trying to get gl_config");
        let (mut window, gl_config) =
            DisplayBuilder::new() // let glutin-winit helper crate handle the complex parts of opengl context creation
                .with_window_builder(Some(winit_window_builder.clone()))
                .build(event_loop, template, |configs| {
                    configs
                        .reduce(|accum, config| {
                            if config.num_samples() > accum.num_samples() {
                                config
                            } else {
                                accum
                            }
                        })
                        .unwrap()
                })
                .expect("failed to create gl_config");
        let gl_display = gl_config.display();
        println!("found gl_config: {:?}", &gl_config);

        let raw_window_handle = window.as_ref().map(|w| w.raw_window_handle());
        println!("raw window handle: {:?}", raw_window_handle);
        let context_attributes = ContextAttributesBuilder::new()
            //.with_context_api(ContextApi::Gles(None))
            .with_context_api(ContextApi::OpenGl(Some(Version { major: 3, minor: 3 })))
            .build(raw_window_handle);
        let not_current_gl_context = gl_display
            .create_context(&gl_config, &context_attributes)
            .unwrap();

        // this is where the window is created, if it has not been created while searching for suitable gl_config
        let window = window.take().unwrap_or_else(|| {
            log::debug!("window doesn't exist yet. creating one now with finalize_window");
            glutin_winit::finalize_window(event_loop, winit_window_builder.clone(), &gl_config)
                .expect("failed to finalize glutin window")
        });
        let (width, height): (u32, u32) = window.inner_size().into();
        let width = NonZeroU32::new(width).unwrap_or(NonZeroU32::MIN);
        let height = NonZeroU32::new(height).unwrap_or(NonZeroU32::MIN);
        let surface_attributes = SurfaceAttributesBuilder::<WindowSurface>::new().build(
            window.raw_window_handle(),
            width,
            height,
        );
        log::debug!(
            "creating surface with attributes: {:?}",
            &surface_attributes
        );
        let gl_surface = gl_display
            .create_window_surface(&gl_config, &surface_attributes)
            .unwrap();
        log::debug!("surface created successfully: {gl_surface:?}.making context current");
        let gl_context = not_current_gl_context.make_current(&gl_surface).unwrap();

        gl_surface
            .set_swap_interval(
                &gl_context,
                glutin::surface::SwapInterval::Wait(NonZeroU32::MIN),
            )
            .unwrap();

        Self {
            window,
            gl_context,
            gl_display,
            gl_surface,
        }
    }

    pub fn window(&self) -> &winit::window::Window {
        &self.window
    }

    pub fn resize(&self, physical_size: winit::dpi::PhysicalSize<u32>) {
        self.gl_surface.resize(
            &self.gl_context,
            physical_size.width.try_into().unwrap(),
            physical_size.height.try_into().unwrap(),
        );
    }

    pub fn swap_buffers(&self) -> glutin::error::Result<()> {
        self.gl_surface.swap_buffers(&self.gl_context)
    }

    pub fn get_proc_address(&self, addr: &std::ffi::CStr) -> *const std::ffi::c_void {
        self.gl_display.get_proc_address(addr)
    }
}

#[derive(Debug)]
pub enum UserEvent {
    Redraw(Duration),
}

pub unsafe fn create_display(
    event_loop: &winit::event_loop::EventLoopWindowTarget<UserEvent>,
) -> (GlutinWindowContext, glow::Context) {
    let glutin_window_context = GlutinWindowContext::new(event_loop);
    let gl = glow::Context::from_loader_function(|s| {
        let s = std::ffi::CString::new(s).expect("C string from string for gl proc address");
        glutin_window_context.get_proc_address(&s)
    });

    (glutin_window_context, gl)
}
