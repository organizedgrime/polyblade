use crate::polyhedron::{Polyhedron, VertexId};

impl Polyhedron {
    pub fn split_vertex(&mut self, v: usize) -> Vec<[usize; 2]> {
        let Polyhedron { shape, render, .. } = self;
        let edges = shape.split_vertex(v);
        render.extend(edges.len() - 1, render.positions[v]);
        edges
    }

    pub fn truncate(&mut self) -> Vec<[VertexId; 2]> {
        // log::info!("before truncation:");
        // self.shape.png();
        // let Polyhedron { shape, render, .. } = self;
        let mut new_edges = Vec::default();
        log::info!("there are {:?} vertices", self.shape.vertices());
        for v in self.shape.vertices().rev() {
            new_edges.extend(self.split_vertex(v));
            self.shape.recompute();
            // log::info!("after splitting {v}:");
            // self.shape.png();
        }
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

    pub fn ambo_contract(&mut self) {
        let edges = self.ambo();
        self.shape.contract_edges(edges.clone());
        self.render.contract_edges(edges);
        log::info!(
            "p: {}, d: {}",
            self.render.positions.len(),
            self.shape.len()
        );
    }
}
