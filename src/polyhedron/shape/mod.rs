mod conway;
mod cycles;
mod distance;
mod platonic;
use cycles::*;
use distance::*;

#[cfg(test)]
mod test;
use ultraviolet::{Vec3, Vec4};

use crate::polyhedron::*;
use crate::render::{message::PresetMessage, pipeline::ShapeVertex};

/// Contains all properties that need to be computed iff the structure of the graph changes
#[derive(Default, Debug, Clone)]
pub struct Shape {
    /// Graph, represented as Distance matrix
    pub distance: Distance,
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

    pub fn vertices(&self) -> Vec<ShapeVertex> {
        let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
        self.cycles
            .iter()
            .map(|cycle| {
                let sides: Vec4 = match cycle.len() {
                    3 => Vec3::new(1.0, 1.0, 1.0),
                    4 => Vec3::new(1.0, 0.0, 1.0),
                    _ => Vec3::new(0.0, 1.0, 0.0),
                }
                .into();

                let b_shapes: Vec<ShapeVertex> = barycentric
                    .iter()
                    .map(|&b| ShapeVertex {
                        barycentric: b.into(),
                        sides,
                    })
                    .collect();

                match cycle.len() {
                    3 => b_shapes.clone(),
                    4 => (0..6)
                        .map(|i| ShapeVertex {
                            barycentric: barycentric[i % 3].into(),
                            sides,
                        })
                        .collect(),
                    _ => vec![b_shapes; cycle.len()].concat(),
                }
            })
            .collect::<Vec<Vec<ShapeVertex>>>()
            .concat()
    }

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
}
