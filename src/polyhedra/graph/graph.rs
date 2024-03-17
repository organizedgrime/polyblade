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

    fn recompute_faces(&mut self) {
        let all_edges = self.all_edges();
        let mut triplets = Vec::<Face>::new();
        let mut cycles = HashSet::<Face>::new();

        // find all the triplets
        for u in self.vertices() {
            let adj = self.connections(u.id());
            for x in adj.iter() {
                for y in adj.iter() {
                    if x != y && u.id() < x.id() && x.id() < y.id() {
                        let new_face = Face(vec![x.id(), u.id(), y.id()]);
                        if all_edges.contains(&(x.id(), y.id()).into()) {
                            cycles.insert(new_face);
                        } else {
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }

        // while there are unparsed triplets
        while let Some(triplet) = triplets.pop() {
            let p = triplet.0;
            // for each v adjacent to u_t
            for v in self.connections(*p.last().unwrap()) {
                if v.id() > p[1] {
                    // if v is not a neighbor of u_2..u_t-1
                    if !p[1..p.len() - 1]
                        .iter()
                        .fold(false, |acc, vi| acc || self.connections(*vi).contains(&v))
                    {
                        let new_face = Face([p.clone(), vec![v.id()]].concat());
                        println!("pushing new face: {:?}", new_face);
                        if self.connections(p[0]).contains(&v) {
                            //cycles.remo
                            cycles.insert(new_face);
                        } else {
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }

        let mut cycles = cycles.into_iter().collect::<Vec<_>>();
        cycles.sort_by(|c1, c2| c1.len().cmp(&c2.len()));
        let cycles = cycles[0..self.face_count()].to_vec();
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
