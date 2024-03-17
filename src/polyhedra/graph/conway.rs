pub use super::*;

pub trait Conway<V: Vertex>: Graph<V> + Sized {
    fn contract_edge(&mut self, id: EdgeId) {
        // Give b all the same connections as a
        for b in self.connections(id.0) {
            self.connect((b.id(), id.1))
        }
        // Delete a
        self.delete(id.0);
    }

    fn split_vertex(&mut self, id: VertexId) {
        let mut previous = id;
        for edge in &self.edges(id)[1..] {
            // Remove existing connection
            self.disconnect(edge.id());
            // Insert a new vertex
            let new_vertex = self.insert();

            // Build new face
            self.connect((previous, new_vertex.id()));
            previous = new_vertex.id();

            // Reform old connection
            self.connect((edge.other(id).id(), new_vertex.id()));
        }
        // Close the new face
        self.connect((previous, id));
    }

    /// `t` truncate is equivalent to vertex splitting
    fn truncate(&mut self) {
        for vertex in self.vertices() {
            self.split_vertex(vertex.id());
        }
        self.recompute_faces();
    }

    /// `a` ambo is equivalent to the composition of vertex splitting and edge contraction vefore
    /// applying vertex splitting.
    fn ambo(&mut self) {
        for edge in self.all_edges() {
            self.contract_edge(edge.id());
            self.split_vertex(edge.id().1);
        }
        self.truncate()
    }

    //
    //fn dual(&mut self) {}
    /// `b` bevel is equivalent to `ta`
    fn bevel(&mut self) {
        self.truncate();
        self.ambo();
    }

    /// `e` expand is equal to `aa`
    fn expand(&mut self) {
        self.ambo();
        self.ambo();
    }

    /// `s` snub is applying `e` followed by diagonal addition
    fn snub(&mut self) {
        self.expand();
        //self.diagonal_addition();
    }
}

impl<T, V> Conway<V> for T
where
    T: Graph<V>,
    V: Vertex,
{
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use test_case::test_case;

    #[test]
    fn poly() {
        let mut dodeca = Polyhedron::icosahedron();
        dodeca.contract_edge((0, 1));
    }

    #[test_case(SimpleGraph::new_disconnected(6) ; "SimpleGraph")]
    #[test_case(Polyhedron::new_disconnected(6) ; "Polyhedron")]
    fn contract_edge<C: Conway<V>, V: Vertex>(mut graph: C) {
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

        assert_eq!(ids(graph.connections(0)), vec![2]);
        assert_eq!(ids(graph.connections(1)), vec![2]);

        assert_eq!(ids(graph.connections(2)), vec![0, 1, 3, 4]);

        assert_eq!(ids(graph.connections(3)), vec![2]);
        assert_eq!(ids(graph.connections(4)), vec![2]);
    }

    #[test_case(SimpleGraph::new_disconnected(5) ; "SimpleGraph")]
    #[test_case(Polyhedron::new_disconnected(5) ; "Polyhedron")]
    fn split_vertex<C: Conway<V>, V: Vertex>(mut graph: C) {
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
