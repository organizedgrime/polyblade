use super::Shape;
use crate::polyhedron::{Polyhedron, VertexId};

impl Polyhedron {
    pub fn split_vertex(&mut self, v: usize) -> Vec<[usize; 2]> {
        let Polyhedron { shape, render, .. } = self;
        let edges = shape.split_vertex(v);
        render.extend(edges.len() - 1, render.positions[v]);
        edges
    }

    pub fn truncate(&mut self) -> Vec<[VertexId; 2]> {
        // let Polyhedron { shape, render, .. } = self;
        let mut new_edges = Vec::default();
        for v in self.shape.distance.vertices() {
            new_edges.extend(self.split_vertex(v));
        }
        self.shape.recompute();
        new_edges
    }
}
