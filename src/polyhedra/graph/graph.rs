use super::{Edge, Vertex};

pub trait Graph<V: Vertex>: Sized {
    // New with n vertices
    fn new(vertex_count: usize) -> Self;
    // Connect two vertices
    fn connect(&mut self, edge: impl Into<Edge<V>>);
    // Disconnect two vertices
    fn disconnect(&mut self, edge: impl Into<Edge<V>>);
    // New vertex
    fn insert(&mut self) -> V;
    // Delete
    fn delete(&mut self, vertex: V);
    // Edges of a vertex
    fn edges(&self, vertex: V) -> Vec<Edge<V>> {
        self.connections(vertex)
            .iter()
            .map(|other| (vertex.clone(), other.clone()).into())
            .collect()
    }
    fn connections(&self, vertex: V) -> Vec<V>;

    fn vertices(&self) -> Vec<V>;

    fn all_edges(&self) -> Vec<Edge<V>> {
        self.vertices()
            .iter()
            .map(|v| self.edges(*v))
            .flatten()
            .fold(Vec::new(), |mut acc, e| {
                if !acc.contains(&e) {
                    acc.push(e);
                }
                acc
            })
    }
}
