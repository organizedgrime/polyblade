use super::{Edge, EdgeId, Vertex, VertexId};

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
}
