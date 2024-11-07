use crate::bones::{Polyhedron, VertexId};

use super::Shape;

impl Polyhedron {
    pub fn truncate(&mut self) -> Vec<[VertexId; 2]> {
        let Polyhedron { shape, render, .. } = self;
        // let Shape {
        //     distance, cycles, ..
        // } = shape;

        let mut new_edges = Vec::default();
        // for v in distance.vertices() {
        //     let connections = cycles.sorted_connections(v);
        //     render.extend(connections.len() - 1, render.positions[v]);
        //     new_edges.extend(distance.split_vertex(v, connections));
        //     *cycles = distance.simple_cycles();
        // }
        // self.shape.recompute();
        new_edges
    }
}
