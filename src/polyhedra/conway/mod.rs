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
        let mut new_face = vec![vertex.clone()];
        // Skipping the first connection
        for edge in edges[1..].iter() {
            // Insert a new vertex
            let new_vertex = self.insert();
            new_face.push(new_vertex.clone());
            // Connect the new one
            self.connect((edge.other(vertex.clone()), new_vertex));
        }

        for edge in edges[1..].iter() {
            self.disconnect(edge);
        }

        for i in 0..new_face.len() {
            let edge: Edge<Self> = (
                new_face[i].clone(),
                new_face[(i + 1) % new_face.len()].clone(),
            )
                .into();
            self.connect(edge);
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

        assert_eq!(graph.vertices().len(), 6);
        assert_eq!(graph.all_edges().len(), 5);

        graph.contract_edge((1, 3));

        assert_eq!(graph.vertices().len(), 5);
        assert_eq!(graph.all_edges().len(), 4);

        assert_eq!(graph.connections(&0), vec![2]);
        assert_eq!(graph.connections(&1), vec![2]);

        assert_eq!(graph.connections(&2), vec![0, 1, 3, 4]);

        assert_eq!(graph.connections(&3), vec![2]);
        assert_eq!(graph.connections(&4), vec![2]);
    }

    #[test]
    fn split_vertex() {
        let mut graph = SimpleGraph::new(5);
        graph.connect((1, 0));
        graph.connect((1, 2));

        graph.connect((1, 3));
        graph.connect((1, 4));

        assert_eq!(graph.vertices().len(), 5);
        assert_eq!(graph.all_edges().len(), 4);

        graph.split_vertex(1);

        println!("all_edges: {:?}", graph.all_edges());

        assert_eq!(graph.vertices().len(), 8);
        assert_eq!(graph.all_edges().len(), 8);
    }
}
