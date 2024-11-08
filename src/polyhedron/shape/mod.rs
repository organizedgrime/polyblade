mod conway;
mod cycles;
mod distance;
mod platonic;
use std::ops::Range;

use cycles::*;
use distance::*;

#[cfg(test)]
mod test;
use ultraviolet::{Vec3, Vec4};

use crate::polyhedron::*;
use crate::render::{message::PresetMessage, pipeline::ShapeVertex};

/// Contains all properties that need to be computed iff the structure of the graph changes
#[derive(Default, Debug, Clone)]
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

impl Shape {
    pub fn len(&self) -> usize {
        self.distance.len()
    }

    pub fn edges(&self) -> impl Iterator<Item = [VertexId; 2]> + use<'_> {
        self.distance.edges()
    }

    pub fn vertices(&self) -> Range<VertexId> {
        self.distance.vertices()
    }

    pub fn recompute(&mut self) {
        // Update the distance matrix in place
        self.distance.pst();
        // Find and save cycles
        self.cycles = Cycles::from(&self.distance);
        // Find and save springs
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
        self.svg = self.distance.svg().unwrap_or(vec![]);
    }

    // pub fn preset(preset: &PresetMessage) -> Shape {
    //     let mut shape = Shape {
    //         distance: Distance::preset(preset),
    //         ..Default::default()
    //     };
    //     shape.recompute();
    //     shape
    // }

    pub fn release(&mut self, edges: &[[VertexId; 2]]) {
        for &edge in edges {
            self.distance.disconnect(edge);
        }
        self.recompute();
    }

    pub fn contraction(&mut self, edges: &[[VertexId; 2]]) {
        self.distance.contract_edges(edges.to_vec());
        self.recompute();
    }

    /// Given a vertex pairing, what is their distance in G divided by the diameter of G
    pub fn diameter_percent(&self, [v, u]: [VertexId; 2]) -> f32 {
        self.distance[[v, u]] as f32 / self.distance.diameter() as f32
    }
}
