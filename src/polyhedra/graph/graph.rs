use std::collections::{HashMap, HashSet};

use super::*;

pub trait Graph<V: Vertex>: Sized {
    fn vertex(&self, id: VertexId) -> Option<V>;
    fn edge(&self, id: EdgeId) -> Option<Edge<V>> {
        if let (Some(v1), Some(v2)) = (self.vertex(id.0), self.vertex(id.1)) {
            Some((v1, v2).into())
        } else {
            None
        }
    }
    // New with n vertices
    fn new_disconnected(vertex_count: usize) -> Self;
    // Connect two vertices
    fn connect(&mut self, id: EdgeId);
    // Disconnect two vertices
    fn disconnect(&mut self, id: EdgeId);
    // New vertex
    fn insert(&mut self) -> V;
    // Delete
    fn delete(&mut self, id: VertexId);
    // Edges of a vertex
    fn edges(&self, id: VertexId) -> Vec<Edge<V>> {
        if let Some(vertex) = self.vertex(id) {
            self.connections(id)
                .iter()
                .map(|other| (vertex.clone(), other.clone()).into())
                .collect()
        } else {
            vec![]
        }
    }
    // Faces
    fn faces(&self) -> Vec<Face>;
    // Vertices that are connected to a given vertex
    fn connections(&self, id: VertexId) -> Vec<V>;
    // All vertices
    fn vertices(&self) -> Vec<V>;
    // All edges
    fn all_edges(&self) -> Vec<Edge<V>> {
        self.vertices()
            .iter()
            .map(|v| self.edges(v.id()))
            .flatten()
            .fold(Vec::new(), |mut acc, e| {
                if !acc.contains(&e) {
                    acc.push(e);
                }
                acc
            })
    }
    // Depth-first search to detect cycles
    fn dfs(
        &self,
        start: VertexId,
        node: VertexId,
        path: &mut Vec<VertexId>,
        visited: &mut HashSet<VertexId>,
        cycles: &mut Vec<Face>,
    ) {
        visited.insert(node);
        path.push(node);

        let neighbors = self.connections(node);
        for neighbor in neighbors {
            if neighbor.id() == start && path.len() > 2 {
                let face = Face(path.clone());
                if !cycles.contains(&face) {
                    cycles.push(face);
                }
            } else if !visited.contains(&neighbor.id()) {
                self.dfs(start, neighbor.id(), path, visited, cycles);
            }
        }

        visited.remove(&node);
        path.pop();
    }

    // Depth-first search to detect cycles
    fn recompute_faces(&mut self) {
        let mut cycles = Vec::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        for v in self.vertices() {
            self.dfs(v.id(), v.id(), &mut path, &mut visited, &mut cycles);
            visited.clear();
            path.clear();
        }

        let max_faces = 2 + self.all_edges().len() - self.vertices().len();
        cycles.sort_by(|c1, c2| c1.len().cmp(&c2.len()));
        let cycles = cycles[0..max_faces].to_vec();
        println!("cycles: {:?}", cycles);
        self.set_faces(cycles)
    }

    fn set_faces(&mut self, faces: Vec<Face>);
}
