use crate::{
    bones::{PolyGraph, Transaction},
    Instant,
};
use iced::{font, Color, Command};
use std::fmt::Display;
use strum_macros::{Display, EnumIter};
use ultraviolet::Vec3;

use super::{
    polydex::Polydex,
    state::{AppState, ColorPickerState, ModelState, RenderState},
};

#[derive(Debug, Clone, Display)]
pub enum PolybladeMessage {
    Tick(Instant),
    Preset(PresetMessage),
    Conway(ConwayMessage),
    Render(RenderMessage),
    Model(ModelMessage),
    // Font
    FontLoaded(Result<(), font::Error>),
    // Polydex
    PolydexLoaded(Result<Polydex, String>),
    OpenWiki(String),
}

#[derive(Debug, Clone, EnumIter)]
pub enum PresetMessage {
    Prism(usize),
    AntiPrism(usize),
    Pyramid(usize),
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl Display for PresetMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PresetMessage::*;
        let value = match self {
            Prism(n) => match n {
                3 => "Triangular",
                4 => "Cube",
                5 => "Pentagonal",
                6 => "Hexagonal",
                7 => "Heptagonal",
                8 => "Octagonal",
                _ => "?",
            },
            AntiPrism(n) => match n {
                2 => "Digonal",
                3 => "Triangular",
                4 => "Square",
                5 => "Pentagonal",
                6 => "Hexagonal",
                7 => "Heptagonal",
                8 => "Octagonal",
                _ => "?",
            },
            Pyramid(n) => match n {
                3 => "Tetrahedron",
                4 => "Square",
                5 => "Pentagonal",
                6 => "Hexagonal",
                7 => "Heptagonal",
                8 => "Octagonal",
                _ => "?",
            },
            _ => &format!("{self:?}"),
        };
        f.write_str(value)
    }
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ConwayMessage {
    // 1
    Dual,
    // 2
    Join,
    Ambo,
    // 3
    Kis,
    // Needle,
    // Zip,
    Truncate,
    // 4
    //Ortho,
    Expand,
    // 5
    // Gyro,
    Snub,
    // // 6
    // Meta,
    Bevel,
}

#[derive(Debug, Clone)]
pub enum RenderMessage {
    Schlegel(bool),
    Rotating(bool),
    FovChanged(f32),
    LineThickness(f32),
    ColorMethod(ColorMethodMessage),
    ColorPicker(ColorPickerMessage),
}

impl Display for RenderMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use RenderMessage::*;
        let value = match &self {
            ColorMethod(method) => method.to_string(),
            _ => {
                format!("{self:?}")
            }
        };
        f.write_str(&value)
    }
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ColorMethodMessage {
    Vertex,
    Edge,
    Polygon,
    Face,
}

impl From<ColorMethodMessage> for f32 {
    fn from(val: ColorMethodMessage) -> Self {
        match val {
            ColorMethodMessage::Vertex => 0.0,
            ColorMethodMessage::Edge => 1.0,
            ColorMethodMessage::Polygon => 2.0,
            ColorMethodMessage::Face => 2.0,
        }
    }
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ColorPickerMessage {
    ChangeNumber(i16),
    ChooseColor(usize),
    SubmitColor(Color),
    CancelColor,
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ModelMessage {
    ScaleChanged(f32),
}

pub trait ProcessMessage<T> {
    fn process(&self, state: &mut T) -> Command<PolybladeMessage>;
}

impl ProcessMessage<ModelState> for PresetMessage {
    fn process(&self, state: &mut ModelState) -> Command<PolybladeMessage> {
        use PresetMessage::*;
        match &self {
            Prism(n) => {
                state.polyhedron = PolyGraph::prism(*n);
                if n == &4 {
                    state.polyhedron.name = "C".into();
                }
            }
            AntiPrism(n) => state.polyhedron = PolyGraph::anti_prism(*n),
            Pyramid(n) => {
                state.polyhedron = PolyGraph::pyramid(*n);
                if n == &3 {
                    state.polyhedron.name = "T".into();
                }
            }
            Octahedron => state.polyhedron = PolyGraph::octahedron(),
            Dodecahedron => state.polyhedron = PolyGraph::dodecahedron(),
            Icosahedron => state.polyhedron = PolyGraph::icosahedron(),
        }

        Command::none()
    }
}

impl ProcessMessage<ModelState> for ConwayMessage {
    fn process(&self, state: &mut ModelState) -> Command<PolybladeMessage> {
        state
            .polyhedron
            .transactions
            .push(Transaction::Conway(self.clone()));
        Command::none()
    }
}

impl ProcessMessage<RenderState> for RenderMessage {
    fn process(&self, state: &mut RenderState) -> Command<PolybladeMessage> {
        use RenderMessage::*;
        match &self {
            Schlegel(schlegel) => {
                state.schlegel = *schlegel;
                if *schlegel {
                    state.camera.fov_y = 2.9;
                } else {
                    state.camera.fov_y = 1.0;
                    state.camera.eye = Vec3::new(0.0, 2.0, 3.0);
                }
                Command::none()
            }
            Rotating(rotating) => {
                state.rotating = *rotating;
                if !rotating {
                    state.rotation_duration = Instant::now().duration_since(state.start);
                } else {
                    state.start = Instant::now().checked_sub(state.rotation_duration).unwrap();
                }
                Command::none()
            }
            FovChanged(fov) => {
                state.camera.fov_y = *fov;
                Command::none()
            }
            LineThickness(thickness) => {
                state.line_thickness = *thickness;
                Command::none()
            }
            ColorMethod(method) => {
                state.method = method.clone();
                Command::none()
            }
            ColorPicker(picker) => picker.process(&mut state.picker),
        }
    }
}

impl ProcessMessage<ColorPickerState> for ColorPickerMessage {
    fn process(&self, state: &mut ColorPickerState) -> Command<PolybladeMessage> {
        use ColorPickerMessage::*;
        match self {
            ChangeNumber(colors) => {
                state.colors = *colors;
            }
            ChooseColor(i) => {
                state.color_index = Some(*i);
                state.picked_color = state.palette.colors[*i].into();
            }
            SubmitColor(color) => {
                state.picked_color = *color;
                if let Some(i) = state.color_index {
                    state.palette.colors[i] = (*color).into();
                }
                state.color_index = None;
            }
            CancelColor => {
                state.color_index = None;
            }
        }
        Command::none()
    }
}

impl ProcessMessage<ModelState> for ModelMessage {
    fn process(&self, state: &mut ModelState) -> Command<PolybladeMessage> {
        match self {
            ModelMessage::ScaleChanged(scale) => state.scale = *scale,
        }
        Command::none()
    }
}

impl ProcessMessage<AppState> for PolybladeMessage {
    fn process(&self, state: &mut AppState) -> Command<PolybladeMessage> {
        use PolybladeMessage::*;
        match self {
            Tick(time) => {
                if state.render.schlegel {
                    state.render.camera.eye =
                        state.model.polyhedron.face_centroid(0) * state.model.scale;
                }

                // If the polyhedron has changed
                if state.info.conway != state.model.polyhedron.name {
                    // Recompute its Polydex entry
                    state.info = state.model.polyhedron.polydex_entry(&state.polydex);
                }
                state.update_state(*time);
                Command::none()
            }
            Preset(preset) => preset.process(&mut state.model),
            Conway(conway) => conway.process(&mut state.model),
            Render(render) => render.process(&mut state.render),
            Model(model) => model.process(&mut state.model),
            PolydexLoaded(polydex) => {
                if let Ok(polydex) = polydex {
                    state.polydex = polydex.to_vec();
                    state.info = state.model.polyhedron.polydex_entry(&state.polydex);
                } else {
                    //tracing_subscriber::warn
                }
                Command::none()
            }
            OpenWiki(wiki) => {
                let _ = open::that(wiki).ok();
                Command::none()
            }
            _ => Command::none(),
        }
    }
}
