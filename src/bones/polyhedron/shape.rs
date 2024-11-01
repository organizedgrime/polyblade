use ultraviolet::{Vec3, Vec4};

use crate::{
    bones::polyhedron::*,
    render::{message::PresetMessage, pipeline::ShapeVertex},
};

/// Contains all properties that need to be computed iff the structure of the graph changes
#[derive(Default, Debug, Clone)]
pub struct Shape {
    /// Graph, represented as Distance matrix
    pub distance: Distance,
    /// Cycles in the graph
    pub cycles: Cycles,
    /// Faces / chordless cycles
    pub springs: Vec<[VertexId; 2]>,
}

impl Shape {
    pub fn recompute(&mut self) {
        // Find and save cycles
        self.cycles = self.distance.simple_cycles();
        // Update the distance matrix in place
        self.distance.pst();
        // Find and save springs
        self.springs = self.distance.springs();
    }

    pub fn preset(preset: &PresetMessage) -> Shape {
        let mut shape = Shape {
            distance: Distance::preset(preset),
            ..Default::default()
        };

        shape.recompute();
        shape
    }

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

    pub fn kis(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
        let edges = self.distance.edges().collect();
        // let mut cycles = self.cycles.clone();
        if let Some(degree) = degree {
            self.cycles
                .iter()
                .collect::<Vec<_>>()
                .retain(|c| c.len() == degree);
        }
        for cycle in self.cycles.iter() {
            let v = self.distance.insert();
            let mut vpos = Vec3::zero();

            for &u in cycle.iter() {
                self.distance.connect([v, u]);
                //vpos += self.positions[&u];
            }

            //self.positions.insert(v, vpos / cycle.len() as f32);
        }

        // self.pst();
        // self.find_cycles();
        //self.transactions.insert(1, Transaction::Name('k'));
        self.recompute();
        edges
    }
}
