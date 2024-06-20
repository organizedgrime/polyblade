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
    // Join,
    Ambo,
    // 3
    // Kis,
    // Needle,
    // Zip,
    Truncate,
    // 4
    // Ortho,
    Expand,
    // 5
    // Gyro,
    // Snub,
    // // 6
    // Meta,
    Bevel,
    Contract,
}

/*
impl ConwayMessage {
    pub fn apply(&self, graph: &mut PolyGraph) -> Transaction {
        use ConwayMessage::*;
        match self {
            Dual => {
                graph.dual();
                None
            }
            Truncate => {
                graph.truncate();
                graph.pst();
                None
            }
            Ambo => {
                graph.ambo();
                //graph.pst();
                None
            }
            Bevel => {
                graph.bevel();
                graph.pst();
                None
            }
            Expand => {
                graph.expand();
                graph.pst();
                None
            }
            _ => None,
        }
    }
}
*/
