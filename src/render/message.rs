use crate::Instant;
use iced::{font, Color};
use std::fmt::Display;
use strum_macros::{Display, EnumIter};

use super::polydex::Polydex;

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    // UI controls
    // Rotate(bool),
    // Schlegel(bool),
    SizeChanged(f32),
    ColorsChanged(i16),
    FovChanged(f32),
    // Shape modifications
    Preset(PresetMessage),
    Conway(ConwayMessage),
    Render(RenderMessage),
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

trait HotKey: Display {
    fn hotkey(&self) -> char {
        self.to_string().to_lowercase().chars().nth(0).unwrap()
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

impl HotKey for PresetMessage {}
impl HotKey for ConwayMessage {}
