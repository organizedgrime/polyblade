use std::{fmt::Display, time::Instant};
use strum_macros::{Display, EnumIter};

use crate::PolyGraph;

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    Rotate(bool),
    Schlegel(bool),
    SizeChanged(f32),
    FovChanged(f32),
    Preset(PresetMenu),
    Conway(ConwayMessage),
}

#[derive(Debug, Clone, EnumIter)]
pub enum PresetMenu {
    Prism(usize),
    AntiPrism(usize),
    Pyramid(usize),
    Octahedron,
    Dodecahedron,
    Icosahedron,
}

impl Display for PresetMenu {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use PresetMenu::*;
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
                2 => f.write_str("Digonal"),
                3 => f.write_str("Triangular"),
                4 => f.write_str("Square"),
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

impl PresetMenu {
    pub fn polyhedron(&self) -> PolyGraph {
        use PresetMenu::*;
        match *self {
            Prism(n) => PolyGraph::prism(n),
            AntiPrism(n) => PolyGraph::anti_prism(n),
            Pyramid(n) => PolyGraph::pyramid(n),
            Octahedron => PolyGraph::octahedron(),
            Dodecahedron => PolyGraph::dodecahedron(),
            Icosahedron => PolyGraph::icosahedron(),
        }
    }
}

#[allow(dead_code)]
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

impl HotKey for PresetMenu {}
impl HotKey for ConwayMessage {}
