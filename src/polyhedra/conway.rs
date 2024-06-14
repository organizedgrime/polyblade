use std::collections::{HashMap, VecDeque};

pub use super::*;
use std::collections::HashSet;

impl PolyGraph {
    pub fn contract_edge(&mut self, e: impl Into<Edge>) {
        let e: Edge = e.into();
        // Give u all the same connections as v
        for w in self.connections(e.v()).into_iter() {
            self.connect((w, e.u()));
        }
        // Delete a
        for f in self.cycles.iter_mut() {
            f.replace(e.v(), e.u());
        }

        self.adj_v = self
            .adj_v
            .clone()
            .into_iter()
            .map(|f| {
                if let Some(w) = f.other(e.v()) {
                    (e.u(), w).into()
                } else {
                    f
                }
            })
            .filter(|e| e.v() != e.u())
            .collect();

        self.delete(e.v());
    }

    #[allow(dead_code)]
    pub fn contract_edges(&mut self, edges: HashSet<Edge>) {
        for e in edges.into_iter() {
            self.contract_edge(e);
        }
    }

    pub fn split_vertex_face(&mut self, v: VertexId) -> HashSet<Edge> {
        let original_position = self.positions[&v];
        let mut connections: VecDeque<VertexId> = self.connections(v).into_iter().collect();
        let mut transformations: HashMap<VertexId, VertexId> = Default::default();
        let mut new_face = Vec::new();

        // Remove the vertex

        // connect a new node to every existing connection
        while let Some(u) = connections.pop_front() {
            // Insert a new node in the same location
            let new_vertex = self.insert();
            // Track it in the new face
            new_face.push(new_vertex);
            // Update pos
            self.positions.insert(new_vertex, original_position);
            // Reform old connection
            self.connect((u, new_vertex));
            // track transformation
            transformations.insert(u, new_vertex);
        }

        // track the edges that will compose the new face
        let mut new_edges = HashSet::new();

        // upate every face
        for i in 0..self.cycles.len() {
            // if this face had v in it
            if let Some(vi) = self.cycles[i].iter().position(|&x| x == v) {
                // indices before and after v in face
                let vh = (vi + self.cycles[i].len() - 1) % self.cycles[i].len();
                let vj = (vi + 1) % self.cycles[i].len();

                let b = transformations[&self.cycles[i][vh]];
                let a = transformations[&self.cycles[i][vj]];

                self.cycles[i].insert(vi, a);
                self.cycles[i].insert(vi, b);

                let e: Edge = (a, b).into();
                new_edges.insert(e);
                self.connect(e);
            }
        }

        self.cycles.push(new_edges.clone().into());

        self.delete(v);
        new_edges
    }

    /*
    pub fn sorted_cycles(&self, v: VertexId) -> Vec<usize> {
        let edge = self.adjacents.iter().find(|e| e.contains(v));
    }
    */

    /// `t` truncate
    pub fn truncate(&mut self) -> HashSet<Edge> {
        let mut new_edges = HashSet::new();
        for v in self.vertices.clone() {
            new_edges.extend(self.split_vertex_face(v));
        }
        self.name.insert(0, 't');
        new_edges
    }

    /// `a` ambo
    pub fn ambo(&mut self) {
        // Truncate
        let new_edges = self.truncate();
        let original_edges: HashSet<Edge> = self
            .adj_v
            .clone()
            .difference(&new_edges)
            .map(Edge::clone)
            .collect();

        self.contracting_edges.extend(original_edges);
        /*
        // Contract original edge set
        self.contract_edges(original_edges);
        self.name.remove(0);
        self.name.insert(0, 'a');
        */
    }

    /// `b` = `ta`
    pub fn bevel(&mut self) {
        self.truncate();
        self.ambo();
        self.name.remove(0);
        self.name.remove(0);
        self.name.insert(0, 'b');
    }

    /// `e` = `aa`
    pub fn expand(&mut self) {
        let mut new_edges = HashSet::<Edge>::new();
        // For every vertex
        for v in self.vertices.clone() {
            let original_position = self.positions[&v];
            let mut new_face = Face::default();
            // For every face which contains the vertex
            for i in (0..self.cycles.len())
                .into_iter()
                .filter(|&i| self.cycles[i].containz(&v))
                .collect::<Vec<usize>>()
            {
                // Create a new vertex
                let u = self.insert();
                // Replace it in the face
                self.cycles[i].replace(v, u);
                // Now replace
                let ui = self.cycles[i].iter().position(|&x| x == u).unwrap();
                let flen = self.cycles[i].len();
                // Find the values that came before and after in the face
                let a = self.cycles[i][(ui + flen - 1) % flen];
                let b = self.cycles[i][(ui + 1) % flen];
                // Remove existing edges which may no longer be accurate
                if new_edges.remove(&(a, v).into()) {
                    println!("meoww: a {a}");
                }
                if new_edges.remove(&(b, v).into()) {
                    println!("meoww: b {b}");
                }
                println!("nya");
                // Add the new edges which are so yass
                new_edges.insert((a, u).into());
                new_edges.insert((b, u).into());
                // Add u to the new face being formed
                new_face.push(u);
                // pos
                self.positions.insert(u, original_position);
            }

            for i in 0..new_face.len() {
                new_edges.insert((new_face[i], new_face[(i + 1) % new_face.len()]).into());
            }
            println!("new_face: {new_face:?}");
            self.cycles.push(new_face);
            self.delete(v);
        }

        self.adj_v = new_edges;

        /*
        self.name.remove(0);
        self.name.insert(0, 'e');
        */
    }

    /*
    /// `s` snub is applying `e` followed by diagonal addition
    pub fn snub(&mut self) {
        self.expand();
        //self.diagonal_addition();
    }
    */

    // `j` join
    // `z` zip
    // `g` gyro
    // `m` meta = `kj`
    // `o` ortho = `jj`
    // `n` needle
    // `k` kis
}

#[cfg(test)]
mod test {
    use crate::polyhedra::PolyGraph;

    #[test]
    fn truncate() {
        let mut shape = PolyGraph::icosahedron();
        shape.truncate();
    }

    #[test]
    fn contract_edge() {
        let mut graph = PolyGraph::cube();
        assert_eq!(graph.vertices.len(), 8);
        assert_eq!(graph.adj_v.len(), 12);

        graph.contract_edge((0, 1));
        graph.pst();

        assert_eq!(graph.vertices.len(), 7);
        assert_eq!(graph.adj_v.len(), 11);
    }

    #[test]
    fn split_vertex() {
        let mut graph = PolyGraph::cube();
        assert_eq!(graph.vertices.len(), 8);
        assert_eq!(graph.adj_v.len(), 12);

        graph.split_vertex_face(0);
        graph.pst();

        assert_eq!(graph.vertices.len(), 10);
        assert_eq!(graph.adj_v.len(), 15);
    }
}
