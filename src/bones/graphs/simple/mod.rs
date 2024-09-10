mod edge;
pub use edge::*;
use std::collections::hash_set::Iter;
use std::collections::HashSet;

pub type VertexId = usize;

pub struct Simple {
    /// Vertex Identifiers
    pub vertices: HashSet<VertexId>,
    /// Set of undirected edges
    pub edges: HashSet<Edge>,
}

impl Simple {
    pub fn connect(&mut self, e: impl Into<Edge>) {
        let e = e.into();
        if e.v() != e.u() {
            self.edges.insert(e);
        }
    }

    pub fn disconnect(&mut self, e: impl Into<Edge>) {
        self.edges.remove(&e.into());
    }

    pub fn insert(&mut self) -> VertexId {
        let new_id = self.vertices.iter().max().unwrap() + 1 % VertexId::MAX;
        self.vertices.insert(new_id);
        new_id
    }

    pub fn delete(&mut self, v: VertexId) {
        self.vertices.remove(&v);

        self.edges = self
            .edges
            .clone()
            .into_iter()
            .filter(|e| !e.contains(v))
            .collect();
    }

    pub fn vertex_count(&self) -> usize {
        self.vertices.len()
    }

    pub fn edge_count(&self) -> usize {
        self.edges.len()
    }

    pub fn vertices(&self) -> Iter<VertexId> {
        self.vertices.iter()
    }

    pub fn edges(&self) -> Iter<Edge> {
        self.edges.iter()
    }

    pub fn vertex_connections(&self, v: VertexId) -> HashSet<VertexId> {
        self.edges.iter().filter_map(|e| e.other(v)).collect()
    }

    pub fn edge_connections(&self, v: VertexId) -> Vec<Edge> {
        self.edges
            .iter()
            .filter_map(|e| if e.other(v).is_some() { Some(*e) } else { None })
            .collect()
    }

    pub fn face_count(&self) -> i64 {
        2 + self.edges.len() as i64 - self.vertices.len() as i64
    }
}
