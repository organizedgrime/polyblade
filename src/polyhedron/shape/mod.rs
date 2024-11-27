mod conway;
mod cycles;
mod distance;
mod platonic;
use std::{fmt::Display, ops::Range};

use cycles::*;
use distance::*;

#[cfg(test)]
mod test;

use crate::polyhedron::*;

/// Contains all properties that need to be computed iff the structure of the graph changes
#[derive(Default, Clone)]
pub(super) struct Shape {
    /// Graph, represented as Distance matrix
    distance: Distance,
    /// Cycles in the graph
    pub cycles: Cycles,
    /// Faces / chordless cycles
    pub springs: Vec<[VertexId; 2]>,
    /// SVG string of graph representation
    pub svg: Vec<u8>,
}

impl PartialEq for Shape {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}

impl std::fmt::Debug for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.distance.to_string())
    }
}

impl Display for Shape {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.distance.to_string())
    }
}

impl Shape {
    pub fn len(&self) -> usize {
        self.distance.len()
    }

    pub fn degree(&self, v: usize) -> usize {
        self.distance.connections(v).len()
    }

    pub fn edges(&self) -> impl Iterator<Item = [VertexId; 2]> + use<'_> {
        self.distance.edges()
    }

    pub fn vertices(&self) -> Range<VertexId> {
        self.distance.vertices()
    }

    pub fn recompute(&mut self) {
        // log::info!("new distance:\n{}", self.distance);
        // Update the distance matrix in place
        self.distance.pst();
        // Find and save cycles
        self.cycles = Cycles::from(&self.distance);
        // log::info!("new cycles:\n{:?}", self.cycles);
        // Find and save springs
        self.springs = self.distance.springs();
    }

    pub fn compute_springs(&mut self) {
        self.springs = self.distance.springs();
    }

    pub fn from(distance: Distance) -> Shape {
        let mut shape = Shape {
            distance,
            ..Default::default()
        };
        shape.recompute();
        shape
    }

    pub fn compute_graph_svg(&mut self) {
        self.svg = self.distance.svg().unwrap_or_default();
    }

    pub fn release(&mut self, edges: &[[VertexId; 2]]) {
        for &edge in edges {
            self.distance.disconnect(edge);
        }
        self.recompute();
    }

    /// Given a vertex pairing, what is their distance in G divided by the diameter of G
    pub fn diameter_percent(&self, [v, u]: [VertexId; 2]) -> f32 {
        self.distance[[v, u]] as f32 / self.distance.diameter() as f32
    }

    pub fn png(&self) {
        use image::ImageReader;
        use std::io::Cursor;
        use viuer::{print, Config};
        if let Some(bytes) = self.distance.png() {
            let mut reader = ImageReader::new(Cursor::new(bytes));
            reader.set_format(image::ImageFormat::Png);
            let img = reader.decode().unwrap();
            let cfg = Config {
                width: Some(20),
                height: Some(20),
                use_kitty: true,
                ..Default::default()
            };
            print(&img, &cfg).unwrap();
            for i in 0..40 {
                println!("\n");
            }
        }
    }
}
