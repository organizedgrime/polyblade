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
        for v in self.shape.vertices() {
            new_edges.extend(self.split_vertex(v));
        }
        self.shape.recompute();
        new_edges
    }

    /// `a` ambo
    /// Returns a set of edges to contract
    pub fn ambo(&mut self) -> Vec<[VertexId; 2]> {
        // Truncate
        let new_edges = self.truncate();
        // Edges that were already there get contracted
        self.shape
            .edges()
            .filter(|&[v, u]| !new_edges.contains(&[v, u]) && !new_edges.contains(&[u, v]))
            .collect()
    }

    pub fn ambod(&self) -> Self {
        let mut g = self.clone();
        let edges = g.ambo();
        //g.shape.contract_edges(edges);
        g.shape.recompute();
        g
    }
}
