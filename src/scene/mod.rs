mod camera;
mod pipeline;
mod polygon;
use crate::polyhedra::PolyGraph;
use crate::{wgpu, RGB};
use camera::Camera;
use iced::time::Duration;
pub use pipeline::*;
use std::f32::consts::PI;
use std::time::Instant;
use ultraviolet::Mat4;

pub use polygon::Polygon;

pub struct AppState {
    pub polyhedron: PolyGraph,
    pub palette: Vec<wgpu::Color>,
    pub transform: Mat4,
    pub scale: f32,
    pub camera: Camera,
    pub rotating: bool,
    pub schlegel: bool,
    pub start: Instant,
    pub rotation_duration: Duration,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            polyhedron: PolyGraph::dodecahedron(),
            palette: vec![
                RGB::new(72, 132, 90),
                RGB::new(163, 186, 112),
                RGB::new(51, 81, 69),
                RGB::new(254, 240, 134),
                RGB::new(95, 155, 252),
                RGB::new(244, 164, 231),
                RGB::new(170, 137, 190),
            ]
            .into_iter()
            .map(Into::<wgpu::Color>::into)
            .collect(),
            transform: Mat4::identity(),
            scale: 1.0,
            camera: Camera::default(),
            rotating: true,
            schlegel: false,
            start: Instant::now(),
            rotation_duration: Duration::from_secs(0),
        }
    }
}

impl AppState {
    pub fn update(&mut self, time: Instant) {
        let time = if self.rotating {
            time.duration_since(self.start)
        } else {
            self.rotation_duration
        };

        self.polyhedron.update();
        let time = time.as_secs_f32();
        self.transform = Mat4::default();
        if !self.schlegel {
            self.transform = Mat4::from_scale(self.scale)
                * Mat4::from_rotation_x(time / PI)
                * Mat4::from_rotation_y(time / PI * 1.1);
        }
    }
}
