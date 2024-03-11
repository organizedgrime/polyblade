mod conway;
mod edge;
mod graph;
mod vertex;

pub use conway::*;
pub use edge::*;
pub use graph::*;
pub use vertex::*;

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
