use std::collections::{HashMap, HashSet};

use super::*;

pub trait Graph<V: Vertex>: Sized {
    fn vertex(&self, id: VertexId) -> Option<V>;
    fn edge(&self, id: EdgeId) -> Option<Edge> {
        if self.vertex(id.0).is_some() && self.vertex(id.1).is_some() {
            Some((id.0, id.1).into())
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
    fn edges(&self, id: VertexId) -> Vec<Edge> {
        if let Some(vertex) = self.vertex(id) {
            self.connections(id)
                .iter()
                .map(|other| (vertex.id(), other.id()).into())
                .collect()
        } else {
            vec![]
        }
    }
    fn face_count(&self) -> usize {
        2 + self.all_edges().len() - self.vertices().len()
    }
    // Faces
    fn faces(&self) -> Vec<Face>;
    // Vertices that are connected to a given vertex
    fn connections(&self, id: VertexId) -> Vec<V>;
    // All vertices
    fn vertices(&self) -> Vec<V>;
    // All edges
    fn all_edges(&self) -> HashSet<Edge> {
        self.vertices()
            .iter()
            .map(|v| self.edges(v.id()))
            .flatten()
            .fold(HashSet::<Edge>::new(), |mut acc, e| {
                acc.insert(e);
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
        cycles: &mut HashSet<Face>,
    ) {
        visited.insert(node);
        path.push(node);

        for neighbor in self.connections(node) {
            if neighbor.id() == start && path.len() > 2 {
                cycles.insert(Face(path.clone()));
            } else if !visited.contains(&neighbor.id()) {
                self.dfs(start, neighbor.id(), path, visited, cycles);
            }
        }

        visited.remove(&node);
        path.pop();
    }

    // Depth-first search to detect cycles
    fn recompute_faces(&mut self) {
        let mut cycles = HashSet::new();
        let mut visited = HashSet::new();
        let mut path = Vec::new();

        for v in self.vertices() {
            self.dfs(v.id(), v.id(), &mut path, &mut visited, &mut cycles);
            visited.clear();
            path.clear();
        }

        let max_faces = 2 + self.all_edges().len() - self.vertices().len();
        let mut cycles = cycles.into_iter().collect::<Vec<_>>();
        cycles.sort_by(|c1, c2| c1.len().cmp(&c2.len()));
        let cycles = cycles[0..max_faces].to_vec();
        println!("cycles: {:?}", cycles);
        self.set_faces(cycles)
    }

    fn set_faces(&mut self, faces: Vec<Face>);
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use test_case::test_case;

    #[test_case(SimpleGraph::new_disconnected(4) ; "SimpleGraph")]
    #[test_case(Polyhedron::new_disconnected(4) ; "Polyhedron")]
    fn basics<G: Graph<V>, V: Vertex>(mut graph: G) {
        // Connect
        graph.connect((0, 1));
        graph.connect((0, 2));
        graph.connect((1, 2));
        assert_eq!(ids(graph.connections(0)), vec![1, 2]);
        assert_eq!(ids(graph.connections(1)), vec![0, 2]);
        assert_eq!(ids(graph.connections(2)), vec![0, 1]);
        assert_eq!(ids(graph.connections(3)), vec![]);

        // Disconnect
        graph.disconnect((0, 1));
        assert_eq!(ids(graph.connections(0)), vec![2]);
        assert_eq!(ids(graph.connections(1)), vec![2]);

        // Delete
        graph.delete(1);
        assert_eq!(ids(graph.connections(0)), vec![1]);
        assert_eq!(ids(graph.connections(1)), vec![0]);
        assert_eq!(ids(graph.connections(2)), vec![]);
    }

    #[test_case(SimpleGraph::new_disconnected(4) ; "SimpleGraph")]
    #[test_case(Polyhedron::new_disconnected(4) ; "Polyhedron")]
    fn chorsless_cycles<G: Graph<V>, V: Vertex>(mut graph: G) {
        // Connect
        graph.connect((0, 1));
        graph.connect((1, 2));
        graph.connect((2, 3));

        assert_eq!(graph.faces().len(), 0);

        graph.connect((2, 0));
        assert_eq!(graph.faces(), vec![Face(vec![0, 1, 2])]);
    }
}
