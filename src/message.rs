use std::time::Instant;

use strum_macros::{Display, EnumIter};

#[derive(Debug, Clone)]
pub enum Message {
    Tick(Instant),
    Rotate(bool),
    CloseAlert,
    Preset(PresetMessage),
    Conway(ConwayMessage),
}

#[derive(Debug, Clone, EnumIter, Display)]
pub enum PresetMessage {
    Prism(usize),
    Pyramid(usize),
    Tetrahedron,
    Cube,
    Octahedron,
    Dodecahedron,
    Icosahedron,
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
    Needle,
    Zip,
    Truncate,
    // 4
    Ortho,
    Expand,
    // 5
    Gyro,
    Snub,
    // 6
    Meta,
    Bevel,
}
