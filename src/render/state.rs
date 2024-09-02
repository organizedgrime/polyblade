use crate::{
    bones::PolyGraph,
    render::{camera::Camera, palette::Palette, polydex::InfoBox},
    Instant,
};

use iced::{time::Duration, Color};
use std::f32::consts::PI;
use ultraviolet::Mat4;

pub struct AppState {
    pub polyhedron: PolyGraph,
    pub info: InfoBox,
    pub palette: Palette,
    pub color_index: Option<usize>,
    pub picked_color: Color,
    pub transform: Mat4,
    pub scale: f32,
    pub colors: i16,
    pub camera: Camera,
    pub rotating: bool,
    pub schlegel: bool,
    pub start: Instant,
    pub rotation_duration: Duration,
}

impl Default for AppState {
    fn default() -> Self {
        let polyhedron = PolyGraph::dodecahedron();
        let info = polyhedron.polydex_entry(&vec![]);
        Self {
            polyhedron,
            info,
            //palette: Palette::desatur8(),
            palette: Palette::polyblade(),
            color_index: None,
            picked_color: Color::from_rgba8(0, 0, 0, 1.0),
            transform: Mat4::identity(),
            scale: 1.0,
            colors: 1,
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
        if self.schlegel {
            self.transform = Mat4::identity();
        } else {
            self.transform = Mat4::from_scale(self.scale)
                * Mat4::from_rotation_x(time / PI)
                * Mat4::from_rotation_y(time / PI * 1.1);
        }
    }
}
