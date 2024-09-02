use crate::{
    bones::PolyGraph,
    render::{camera::Camera, palette::Palette, polydex::InfoBox},
    Instant,
};

use iced::{time::Duration, Color};
use std::f32::consts::PI;
use ultraviolet::Mat4;

use super::polydex::Polydex;

pub struct AppState {
    pub model: ModelState,
    pub render: RenderState,
    pub polydex: Polydex,
    pub info: InfoBox,
}

pub struct RenderState {
    pub camera: Camera,
    pub start: Instant,
    pub rotation_duration: Duration,
    pub rotating: bool,
    pub schlegel: bool,
    pub line_thickness: f32,
    pub picker: ColorPickerState,
}

pub struct ColorPickerState {
    pub palette: Palette,
    pub color_index: Option<usize>,
    pub picked_color: Color,
    pub colors: i16,
}

impl Default for RenderState {
    fn default() -> Self {
        Self {
            camera: Camera::default(),
            start: Instant::now(),
            rotation_duration: Duration::from_secs(0),
            rotating: true,
            schlegel: false,
            line_thickness: 2.0,
            picker: ColorPickerState::default(),
        }
    }
}

impl Default for ColorPickerState {
    fn default() -> Self {
        Self {
            palette: Palette::polyblade(),
            color_index: None,
            picked_color: Color::from_rgba8(0, 0, 0, 1.0),
            colors: 1,
        }
    }
}

pub struct ModelState {
    pub polyhedron: PolyGraph,
    pub scale: f32,
    pub transform: Mat4,
}

impl Default for ModelState {
    fn default() -> Self {
        Self {
            polyhedron: PolyGraph::dodecahedron(),
            scale: 1.0,
            transform: Mat4::identity(),
        }
    }
}

impl Default for AppState {
    fn default() -> Self {
        let polyhedron = PolyGraph::dodecahedron();
        let info = polyhedron.polydex_entry(&vec![]);
        Self {
            model: ModelState::default(),
            render: RenderState::default(),
            polydex: vec![],
            info,
        }
    }
}

impl AppState {
    pub fn update_state(&mut self, time: Instant) {
        let time = if self.render.rotating {
            time.duration_since(self.render.start)
        } else {
            self.render.rotation_duration
        };

        self.model.polyhedron.update();
        let time = time.as_secs_f32();
        self.model.transform = Mat4::default();
        if self.render.schlegel {
            self.model.transform = Mat4::identity();
        } else {
            self.model.transform = Mat4::from_scale(self.model.scale)
                * Mat4::from_rotation_x(time / PI)
                * Mat4::from_rotation_y(time / PI * 1.1);
        }
    }
}
