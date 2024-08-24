mod camera;
mod pipeline;
mod polygon;
use crate::polyhedra::PolyGraph;
use crate::{wgpu, RGB};
use camera::Camera;
use iced::mouse;
use iced::time::Duration;
use iced::widget::shader;
use iced::{Color, Rectangle};
pub use pipeline::*;
use std::f32::consts::PI;
use std::time::Instant;
use ultraviolet::Mat4;

use polygon::Polygon;

#[derive(Clone)]
pub struct Scene {
    pub start: Instant,
    pub size: f32,
    pub clear_face: Option<usize>,
    pub rotation: Mat4,
    pub polyhedron: PolyGraph,
    pub camera: Camera,
    pub light_color: Color,
    pub palette: Vec<wgpu::Color>,
}

impl Scene {
    pub fn new() -> Self {
        Self {
            start: Instant::now(),
            size: 1.0,
            clear_face: None,
            rotation: Mat4::default(),
            polyhedron: PolyGraph::icosahedron(),
            camera: Camera::default(),
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
            light_color: Color::WHITE,
        }
    }

    pub fn update(&mut self, schlegel: bool, time: Duration) {
        self.polyhedron.update();
        let time = time.as_secs_f32();
        self.rotation = Mat4::default();
        if !schlegel {
            self.rotation = Mat4::from_scale(self.size)
                * Mat4::from_rotation_x(time / PI)
                * Mat4::from_rotation_y(time / PI * 1.1);
        }
    }
}

impl<Message> shader::Program<Message> for Scene {
    type State = ();
    type Primitive = Polygon;

    fn draw(
        &self,
        _state: &Self::State,
        _cursor: mouse::Cursor,
        _bounds: Rectangle,
    ) -> Self::Primitive {
        Polygon::new(
            &self.polyhedron,
            &self.palette,
            &self.rotation,
            &self.camera,
        )
    }
}
