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
    state::{AppState, RenderState},
    Polyblade,
};

#[derive(Debug, Clone)]
pub enum PolybladeMessage {
    Tick(Instant),
    // UI controls
    // Rotate(bool),
    // Schlegel(bool),
    ColorsChanged(i16),
    FovChanged(f32),
    // Shape modifications
    Preset(PresetMessage),
    Conway(ConwayMessage),
    Render(RenderMessage),
    Model(ModelMessage),
    // Font
    FontLoaded(Result<(), font::Error>),
    // Polydex
    PolydexLoaded(Result<Polydex, String>),
    OpenWiki(String),
    // Color picking
    ChooseColor(usize),
    SubmitColor(Color),
    CancelColor,
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

#[derive(Debug, Clone, EnumIter, Display)]
pub enum RenderMessage {
    Schlegel(bool),
    Rotating(bool),
    ColorMethod(ColoringStrategyMessage),
}

#[derive(Debug, Default, Clone, EnumIter, Display)]
pub enum ColoringStrategyMessage {
    #[default]
    Vertex,
    Edge,
    Polygon,
    Face,
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum ModelMessage {
    ScaleChanged(f32),
}

pub trait ProcessMessage<T> {
    fn process(&self, state: &mut T) -> Command<PolybladeMessage>;
}

impl ProcessMessage<AppState> for PresetMessage {
    fn process(&self, state: &mut AppState) -> Command<PolybladeMessage> {
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

impl ProcessMessage<AppState> for ConwayMessage {
    fn process(&self, state: &mut AppState) -> Command<PolybladeMessage> {
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
            }
            Rotating(rotating) => {
                state.rotating = *rotating;
                if !rotating {
                    state.rotation_duration = Instant::now().duration_since(state.start);
                } else {
                    state.start = Instant::now().checked_sub(state.rotation_duration).unwrap();
                }
            }
            // LineThickness(thickness) => {
            //     state.render.line_thickness = *thickness;
            // }
            ColorMethod(_) => todo!(),
        }
        Command::none()
    }
}

impl ProcessMessage<AppState> for ModelMessage {
    fn process(&self, state: &mut AppState) -> Command<PolybladeMessage> {
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
                    state.render.camera.eye = state.polyhedron.face_centroid(0) * 1.1;
                }

                // If the polyhedron has changed
                if state.info.conway != state.polyhedron.name {
                    // Recompute its Polydex entry
                    //state.info = state.polyhedron.polydex_entry(&state.polydex);
                }
                state.update_state(*time);
                Command::none()
            }
            ColorsChanged(colors) => {
                state.colors = *colors;
                Command::none()
            }
            FovChanged(fov) => {
                state.render.camera.fov_y = *fov;
                Command::none()
            }
            Preset(preset) => preset.process(state),
            Conway(conway) => conway.process(state),
            Render(render) => render.process(&mut (*state).render),
            Model(model) => model.process(state),
            PolydexLoaded(polydex) => {
                if let Ok(polydex) = polydex {
                    state.polydex = polydex.to_vec();
                    state.info = state.polyhedron.polydex_entry(&state.polydex);
                } else {
                    //tracing_subscriber::warn
                }
                Command::none()
            }
            OpenWiki(wiki) => {
                let _ = open::that(wiki).ok();
                Command::none()
            }
            ChooseColor(i) => {
                state.color_index = Some(*i);
                state.picked_color = state.palette.colors[*i].into();
                Command::none()
            }
            SubmitColor(color) => {
                state.picked_color = *color;
                if let Some(i) = state.color_index {
                    state.palette.colors[i] = (*color).into();
                }
                state.color_index = None;
                Command::none()
            }
            CancelColor => {
                state.color_index = None;
                Command::none()
            }
            _ => Command::none(),
        }
    }
}
