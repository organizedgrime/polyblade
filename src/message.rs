use std::time::Instant;

use iced::font;
use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    Rotate(bool),
    SizeChanged(f32),
    Preset(PresetMessage),
    Conway(ConwayMessage),
    FontLoaded(Result<(), font::Error>),
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

impl std::fmt::Display for PresetMessage {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PresetMessage::*;
        match self {
            Prism(n) => match n {
                3 => f.write_str("Triangular"),
                4 => f.write_str("Cube"),
                5 => f.write_str("Pentagonal"),
                6 => f.write_str("Hexagonal"),
                7 => f.write_str("Heptagonal"),
                8 => f.write_str("Octagonal"),
                _ => f.write_str("?"),
            },
            AntiPrism(n) => match n {
                3 => f.write_str("Triangular"),
                4 => f.write_str("Cube"),
                5 => f.write_str("Pentagonal"),
                6 => f.write_str("Hexagonal"),
                7 => f.write_str("Heptagonal"),
                8 => f.write_str("Octagonal"),
                _ => f.write_str("?"),
            },
            Pyramid(n) => match n {
                3 => f.write_str("Tetrahedron"),
                4 => f.write_str("Square"),
                5 => f.write_str("Pentagonal"),
                6 => f.write_str("Hexagonal"),
                7 => f.write_str("Heptagonal"),
                8 => f.write_str("Octagonal"),
                _ => f.write_str("?"),
            },
            _ => f.write_fmt(format_args!("{self:?}")),
        }
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
    Contract,
}
