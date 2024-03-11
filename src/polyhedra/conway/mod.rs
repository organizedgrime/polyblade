mod graph;
pub use graph::*;

trait Conway: Graph + Sized {
    fn contract_edge(&mut self, edge: impl Into<Edge<Self>>) {
        let edge = edge.into();
        // Determine all of a's connections
        let connections = self.connections(&edge.a);
        // Give b all the same connections
        for connection in connections {
            self.connect((edge.b.clone(), connection))
        }
        // Delete a
        self.delete(&edge.a);
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

impl<G: Graph> Conway for G {}

#[cfg(test)]
mod test {
    use super::{Conway, Graph, SimpleGraph};

    #[test]
    fn contract_edge() {
        let mut graph = SimpleGraph::new(6);
        graph.connect((1, 0));
        graph.connect((1, 2));

        graph.connect((1, 3));

        graph.connect((3, 4));
        graph.connect((3, 5));

        graph.contract_edge((1, 3));

        assert_eq!(graph.connections(&0), vec![2]);
        assert_eq!(graph.connections(&1), vec![2]);

        assert_eq!(graph.connections(&2), vec![0, 1, 3, 4]);

        assert_eq!(graph.connections(&3), vec![2]);
        assert_eq!(graph.connections(&4), vec![2]);
    }
}
