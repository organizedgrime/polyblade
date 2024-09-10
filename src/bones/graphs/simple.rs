use super::{Edge, VertexId};
use std::collections::HashSet;

pub struct Simple {
    /// Vertex Identifiers
    pub vertices: HashSet<VertexId>,
    /// Set of undirected edges
    pub edges: HashSet<Edge>,
}

pub trait SimpleGraph {
    fn connect(&mut self, e: impl Into<Edge>);
    fn disconnect(&mut self, e: impl Into<Edge>);
    fn insert(&mut self) -> VertexId;
    fn delete(&mut self, v: VertexId);
    fn vertex_count(&self) -> usize;
    fn edge_count(&self) -> usize;
    fn vertices(&self) -> Iter<VertexId>;
    fn edges(&self) -> Iter<Edge>;
}

impl SimpleGraph for Simple {
    fn connect(&mut self, e: impl Into<Edge>) {
        let e = e.into();
        if e.v() != e.u() {
            self.edges.insert(e);
        }
    }

    fn disconnect(&mut self, e: impl Into<Edge>) {
        self.edges.remove(&e.into());
    }

    fn insert(&mut self) -> VertexId {
        let new_id = self.vertices.iter().max().unwrap() + 1 % VertexId::MAX;
        self.vertices.insert(new_id);
        new_id
    }

    fn delete(&mut self, v: VertexId) {
        self.vertices.remove(&v);

        self.edges = self
            .edges
            .clone()
            .into_iter()
            .filter(|e| !e.contains(v))
            .collect();
    }

    fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    fn edge_count(&self) -> usize {
        self.edges.len()
    }

    fn vertices(&self) -> Iter<VertexId> {
        self.vertices.iter()
    }

    fn edges(&self) -> Iter<Edge> {
        self.edges.iter()
    }
}
