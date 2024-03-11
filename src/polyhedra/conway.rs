use super::Polyhedron;

trait Conway: Graph + Sized {
    fn contract_edge(&mut self, edge: Edge<Self>) {
        // Determine all of a's connections
        let connections = self.connections(&edge.a);
        // Delete a
        self.delete(&edge.a);
        // Give b all the same connections
        for connection in connections {
            self.connect(Edge::<Self>::new(edge.b.clone(), connection))
        }
    }

    fn split_vertex(&mut self, vertex: Self::Vertex) {
        let edges = self.edges(&vertex);
        let mut new_vertex = edges[0].a.clone();
        // Skipping the first connection
        for edge in edges[1..].iter() {
            // Insert a new vertex
            let b = self.insert();
            // Connect the new one
            self.connect((edge.a.clone(), b));
            // Connect to the previous
            self.connect((edge.a.clone(), new_vertex.clone()));
            new_vertex = edge.a.clone();
            // Disconnect the old one
            self.disconnect(edge);
        }
    }

    //
    fn dual(&mut self) {}

    /*
     * `t` truncate is equivalent to vertex splitting
     * `a` ambo is equivalent to the composition of vertex splitting and edge contraction vefore
     * applying vertex splitting.
     * `b` bevel is equivalent to `ta`
     * `e` expand is equal to `aa`
     * `s` snub is applying `e` followed by diagonal addition
     * the rest are just duals, apparently
     *
     *
     */
}

#[derive(Clone, Copy)]
struct Edge<G: Graph> {
    pub a: G::Vertex,
    pub b: G::Vertex,
}
impl<G: Graph> Edge<G> {
    fn new(a: G::Vertex, b: G::Vertex) -> Self {
        Self { a, b }
    }
}

impl<G: Graph> From<&Edge<G>> for Edge<G> {
    fn from(value: &Edge<G>) -> Self {
        (value.a.clone(), value.b.clone()).into()
    }
}
impl<G: Graph> From<(G::Vertex, G::Vertex)> for Edge<G> {
    fn from(value: (G::Vertex, G::Vertex)) -> Self {
        Self {
            a: value.0,
            b: value.1,
        }
    }
}

impl<G: Graph> PartialEq for Edge<G> {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}

trait Graph: Sized {
    type Vertex: Clone + PartialEq;
    //type Edge = Edge<Self>;

    // New with n vertices
    fn new(vertex_count: usize) -> Self;
    // Connect two vertices
    fn connect(&mut self, edge: impl Into<Edge<Self>>);
    // Disconnect two vertices
    fn disconnect(&mut self, edge: impl Into<Edge<Self>>);
    // New vertex
    fn insert(&mut self) -> Self::Vertex;
    // Delete
    fn delete(&mut self, vertex: &Self::Vertex);
    // Edges of a vertex
    fn edges(&self, vertex: &Self::Vertex) -> Vec<Edge<Self>> {
        self.connections(&vertex)
            .iter()
            .map(|other| Edge::<Self>::new(vertex.clone(), other.clone()))
            .collect()
    }
    fn connections(&self, vertex: &Self::Vertex) -> Vec<Self::Vertex>;
}

struct SimpleGraph {
    pub adjacency_matrix: Vec<Vec<bool>>,
}

impl Graph for SimpleGraph {
    type Vertex = usize;

    fn new(vertex_count: usize) -> Self {
        Self {
            adjacency_matrix: vec![vec![false; vertex_count]; vertex_count],
        }
    }

    fn connect(&mut self, edge: impl Into<Edge<Self>>) {
        let edge = edge.into();
        self.adjacency_matrix[edge.a][edge.b] = true;
        self.adjacency_matrix[edge.b][edge.a] = true;
    }

    fn disconnect(&mut self, edge: impl Into<Edge<Self>>) {
        let edge = edge.into();
        self.adjacency_matrix[edge.a][edge.b] = false;
        self.adjacency_matrix[edge.b][edge.a] = false;
    }

    fn insert(&mut self) -> Self::Vertex {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i].push(false);
        }

        self.adjacency_matrix
            .push(vec![false; self.adjacency_matrix.len() + 1]);

        self.adjacency_matrix.len()
    }

    fn delete(&mut self, vertex: &Self::Vertex) {
        println!("before:\n{:?}", self.adjacency_matrix);
        for i in 0..self.adjacency_matrix.len() {
            let mut x = self.adjacency_matrix[i][..vertex.clone()].to_vec();
            x.extend(&self.adjacency_matrix[i][vertex.clone() + 1..]);
            self.adjacency_matrix[i] = x.to_vec();
        }
        self.adjacency_matrix.remove(*vertex);
        println!("after:\n{:?}", self.adjacency_matrix);
    }

    fn connections(&self, vertex: &Self::Vertex) -> Vec<Self::Vertex> {
        let mut connections: Vec<Self::Vertex> = Vec::new();
        for (other, connected) in self.adjacency_matrix[*vertex].iter().enumerate() {
            if *connected {
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
        assert_eq!(graph.connections(&0), vec![1, 2]);
        assert_eq!(graph.connections(&1), vec![0, 2]);
        assert_eq!(graph.connections(&2), vec![0, 1]);
        assert_eq!(graph.connections(&3), vec![]);

        // Disconnect
        graph.disconnect((0, 1));
        assert_eq!(graph.connections(&0), vec![2]);
        assert_eq!(graph.connections(&1), vec![2]);

        // Delete
        graph.delete(&1);
        assert_eq!(graph.connections(&0), vec![1]);
        assert_eq!(graph.connections(&1), vec![0]);
        assert_eq!(graph.connections(&2), vec![]);
    }
}
