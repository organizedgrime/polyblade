mod conway;
mod edge;
mod graph;
mod vertex;

use std::collections::HashSet;

pub use conway::*;
pub use edge::*;
pub use graph::*;
pub use vertex::*;

use super::{Point, Polyhedron};

pub struct SimpleGraph {
    pub adjacency_matrix: Vec<Vec<bool>>,
}

impl Graph<usize> for SimpleGraph {
    fn vertex(&self, id: VertexId) -> Option<usize> {
        if id < self.adjacency_matrix.len() {
            Some(id)
        } else {
            None
        }
    }
    fn new_disconnected(vertex_count: usize) -> Self {
        Self {
            adjacency_matrix: vec![vec![false; vertex_count]; vertex_count],
        }
    }

    fn vertices(&self) -> Vec<usize> {
        (0..self.adjacency_matrix.len()).collect()
    }

    fn connect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.adjacency_matrix[edge.a][edge.b] = true;
            self.adjacency_matrix[edge.b][edge.a] = true;
        }
    }

    fn disconnect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.adjacency_matrix[edge.a][edge.b] = false;
            self.adjacency_matrix[edge.b][edge.a] = false;
        }
    }

    fn insert(&mut self) -> usize {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i].push(false);
        }

        self.adjacency_matrix
            .push(vec![false; self.adjacency_matrix.len() + 1]);

        self.adjacency_matrix.len() - 1
    }

    fn delete(&mut self, id: usize) {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i].remove(id);
        }
        self.adjacency_matrix.remove(id);
    }

    fn connections(&self, vertex: usize) -> Vec<usize> {
        let mut connections: Vec<usize> = Vec::new();
        for (other, connected) in self.adjacency_matrix[vertex].iter().enumerate() {
            if *connected && other != vertex {
                connections.push(other)
            }
        }
        connections
    }
}

impl Graph<Point> for Polyhedron {
    fn vertex(&self, id: VertexId) -> Option<Point> {
        self.points.get(id).map(|v| v.clone())
    }

    fn new_disconnected(vertex_count: usize) -> Self {
        Polyhedron {
            name: "".to_string(),
            points: (0..vertex_count).map(Point::new_empty).collect(),
            faces: vec![],
            enemies: HashSet::new(),
            edge_length: 1.0,
        }
    }

    fn connect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.points[edge.a.id].connect(edge.b.id);
            self.points[edge.b.id].connect(edge.a.id);
        }
    }

    fn disconnect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.points[edge.a.id].disconnect(edge.b.id);
            self.points[edge.b.id].disconnect(edge.a.id);
        }
    }

    fn insert(&mut self) -> Point {
        let point = Point::new(self.points.len(), vec![]);
        self.points.push(point.clone());
        point
    }

    fn delete(&mut self, id: VertexId) {
        for i in 0..self.points.len() {
            self.points[i].delete(id);
        }
        self.points.remove(id);
        self.points = self
            .points
            .clone()
            .into_iter()
            .enumerate()
            .map(|(new_id, mut v)| {
                v.id = new_id;
                v
            })
            .collect();
    }

    fn connections(&self, id: VertexId) -> Vec<Point> {
        if let Some(vertex) = self.vertex(id) {
            vertex
                .adjacents
                .iter()
                .map(|i| self.points[*i].clone())
                .collect()
        } else {
            vec![]
        }
    }

    fn vertices(&self) -> Vec<Point> {
        self.points.clone()
    }
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
}
