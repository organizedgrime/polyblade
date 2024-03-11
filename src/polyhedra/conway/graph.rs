#[derive(Clone, Copy)]
pub struct Edge<V: Vertex> {
    pub a: V,
    pub b: V,
}
impl<V: Vertex> Edge<V> {
    pub fn other(&self, v: V) -> V {
        if self.a == v {
            self.b.clone()
        } else {
            self.a.clone()
        }
    }
}
impl<V: Vertex> std::fmt::Debug for Edge<V> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Edge")
            .field("a", &self.a)
            .field("b", &self.b)
            .finish()
    }
}
impl<V: Vertex> From<&Edge<V>> for Edge<V> {
    fn from(value: &Edge<V>) -> Self {
        (value.a.clone(), value.b.clone()).into()
    }
}
impl<V: Vertex> From<(V, V)> for Edge<V> {
    fn from(value: (V, V)) -> Self {
        Self {
            a: value.0,
            b: value.1,
        }
    }
}

impl<V: Vertex> PartialEq for Edge<V> {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

//impl<V: Graph> Eq for Edge<G> {}

pub trait Vertex: Clone + Copy + PartialEq + std::fmt::Debug {}
impl Vertex for usize {}

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
        let mut edges = Vec::new();
        for vertex in self.vertices() {
            for edge in self.edges(vertex) {
                if !edges.contains(&edge) {
                    edges.push(edge);
                }
            }
        }
        edges
    }
}

pub struct SimpleGraph {
    pub adjacency_matrix: Vec<Vec<bool>>,
}

impl Graph<usize> for SimpleGraph {
    fn new(vertex_count: usize) -> Self {
        Self {
            adjacency_matrix: vec![vec![false; vertex_count]; vertex_count],
        }
    }

    fn vertices(&self) -> Vec<usize> {
        (0..self.adjacency_matrix.len()).collect()
    }

    fn connect(&mut self, edge: impl Into<Edge<usize>>) {
        let edge = edge.into();
        self.adjacency_matrix[edge.a][edge.b] = true;
        self.adjacency_matrix[edge.b][edge.a] = true;
    }

    fn disconnect(&mut self, edge: impl Into<Edge<usize>>) {
        let edge = edge.into();
        self.adjacency_matrix[edge.a][edge.b] = false;
        self.adjacency_matrix[edge.b][edge.a] = false;
    }

    fn insert(&mut self) -> usize {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i].push(false);
        }

        self.adjacency_matrix
            .push(vec![false; self.adjacency_matrix.len() + 1]);

        self.adjacency_matrix.len() - 1
    }

    fn delete(&mut self, vertex: usize) {
        println!("before:\n{:?}", self.adjacency_matrix);
        for i in 0..self.adjacency_matrix.len() {
            let mut x = self.adjacency_matrix[i][..vertex.clone()].to_vec();
            x.extend(&self.adjacency_matrix[i][vertex.clone() + 1..]);
            self.adjacency_matrix[i] = x.to_vec();
        }
        self.adjacency_matrix.remove(vertex);
        println!("after:\n{:?}", self.adjacency_matrix);
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

#[cfg(test)]
mod test {
    use super::{Graph, SimpleGraph};

    #[test]
    fn basics() {
        let mut graph = SimpleGraph::new(4);

        // Connect
        graph.connect((0, 1));
        graph.connect((0, 2));
        graph.connect((1, 2));
        assert_eq!(graph.connections(0), vec![1, 2]);
        assert_eq!(graph.connections(1), vec![0, 2]);
        assert_eq!(graph.connections(2), vec![0, 1]);
        assert_eq!(graph.connections(3), vec![]);

        // Disconnect
        graph.disconnect((0, 1));
        assert_eq!(graph.connections(0), vec![2]);
        assert_eq!(graph.connections(1), vec![2]);

        // Delete
        graph.delete(1);
        assert_eq!(graph.connections(0), vec![1]);
        assert_eq!(graph.connections(1), vec![0]);
        assert_eq!(graph.connections(2), vec![]);
    }
}
