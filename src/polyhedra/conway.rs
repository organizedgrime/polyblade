use std::collections::{HashMap, VecDeque};

use ultraviolet::Vec3;

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

        self.edges = self
            .edges
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

    pub fn contract_edges(&mut self, edges: HashSet<Edge>) {
        let mut map = HashMap::<VertexId, VertexId>::new();
        for e in edges.into_iter() {
            let u = e.u();
            let v = e.v();
            let e = match (map.get(&u), map.get(&v)) {
                (None, None) => e,
                (None, Some(v)) => (&u, v).into(),
                (Some(u), None) => (u, &v).into(),
                (Some(u), Some(v)) => (u, v).into(),
            };
            if e.v() != e.u() {
                self.contract_edge(e);
                map.insert(e.v(), e.u());
            }
        }

        self.cycles = self
            .cycles
            .clone()
            .into_iter()
            .filter(|c| c.len() > 2)
            .collect();
    }

    pub fn split_vertex(&mut self, v: VertexId) -> HashSet<Edge> {
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

    /// `a` ambo
    /// Returns a set of edges to contract
    pub fn ambo(&mut self) -> HashSet<Edge> {
        // Truncate
        let new_edges = self.truncate(None);
        self.edges
            .clone()
            .difference(&new_edges)
            .map(Edge::clone)
            .collect()
    }

    /// `k` kis
    pub fn kis(&mut self, degree: Option<usize>) -> HashSet<Edge> {
        let edges = self.edges.clone();
        let mut cycles = self.cycles.clone();
        if let Some(degree) = degree {
            cycles.retain(|c| c.len() == degree);
        }
        for cycle in cycles {
            let v = self.insert();
            let mut vpos = Vec3::zero();

            for &u in cycle.iter() {
                self.connect((v, u));
                vpos += self.positions[&u];
            }

            self.positions.insert(v, vpos / cycle.len() as f32);
        }
        self.pst();
        self.find_cycles();
        //self.transactions.insert(1, Transaction::Name('k'));
        edges
    }

    /// `t` truncate
    pub fn truncate(&mut self, degree: Option<usize>) -> HashSet<Edge> {
        let mut new_edges = HashSet::new();
        let mut vertices = self.vertices.clone();
        if let Some(degree) = degree {
            vertices.retain(|&v| self.connections(v).len() == degree);
        }
        for v in vertices {
            new_edges.extend(self.split_vertex(v));
        }
        new_edges
    }

    /// `o` ortho
    #[allow(dead_code)]
    pub fn ortho(&mut self) {
        for _c in self.cycles.clone() {
            let _v = self.insert();
        }
    }
    /// `b` = `ta`
    pub fn bevel(&mut self) -> HashSet<Edge> {
        self.truncate(None);
        self.pst();
        self.ambo()
    }

    pub fn ordered_face_indices(&self, v: VertexId) -> Vec<usize> {
        let relevant = (0..self.cycles.len())
            .filter(|&i| self.cycles[i].containz(&v))
            .collect::<Vec<usize>>();

        let mut edges = HashMap::new();

        for &i in relevant.iter() {
            let ui = self.cycles[i].iter().position(|&x| x == v).unwrap();
            let flen = self.cycles[i].len();
            // Find the values that came before and after in the face
            let a = self.cycles[i][(ui + flen - 1) % flen];
            let b = self.cycles[i][(ui + 1) % flen];
            edges.insert((a, b).into(), i);
        }

        let f: Face = edges.keys().cloned().collect::<HashSet<_>>().into();

        let mut ordered_face_indices = vec![];
        for i in 0..f.len() {
            let e: Edge = (f[i], f[(i + 1) % f.len()]).into();
            let fi = edges.get(&e).unwrap();
            ordered_face_indices.push(*fi);
        }

        ordered_face_indices
    }

    /// `e` = `aa`
    pub fn expand(&mut self, snub: bool) -> HashSet<Edge> {
        let mut new_edges = HashSet::<Edge>::new();
        let mut face_edges = HashSet::<Edge>::new();

        let ordered_face_indices: HashMap<usize, Vec<usize>> = self
            .vertices
            .iter()
            .map(|&v| (v, self.ordered_face_indices(v)))
            .collect();

        // For every vertex
        for v in self.vertices.clone() {
            let original_position = self.positions[&v];
            let mut new_face = Face::default();
            // For every face which contains the vertex
            for &i in ordered_face_indices.get(&v).unwrap() {
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
                new_edges.remove(&(a, v).into());
                new_edges.remove(&(b, v).into());
                // Add the new edges which are so yass
                new_edges.insert((a, u).into());
                new_edges.insert((b, u).into());
                // Add u to the new face being formed
                new_face.push(u);
                // pos
                self.positions.insert(u, original_position);
            }
            for i in 0..new_face.len() {
                face_edges.insert((new_face[i], new_face[(i + 1) % new_face.len()]).into());
            }
            self.cycles.push(new_face);
            self.delete(v);
        }

        let mut solved_edges = HashSet::new();

        // For every triangle / nf edge
        for a in face_edges.iter() {
            // find the edge which is parallel to it
            for b in face_edges.iter() {
                if !solved_edges.contains(a) && !solved_edges.contains(b) {
                    if new_edges.contains(&(a.v(), b.v()).into())
                        && new_edges.contains(&(a.u(), b.u()).into())
                    {
                        if snub {
                            new_edges.insert((a.v(), b.u()).into());
                            let m = Face::new(vec![a.v(), b.u(), a.u()]);
                            let n = Face::new(vec![a.v(), b.u(), b.v()]);
                            self.cycles.push(m);
                            self.cycles.push(n);
                        } else {
                            let quad = Face::new(vec![b.u(), a.u(), a.v(), b.v()]);
                            self.cycles.push(quad);
                        }

                        solved_edges.insert(a);
                        solved_edges.insert(b);
                    }

                    if new_edges.contains(&(a.u(), b.v()).into())
                        && new_edges.contains(&(a.v(), b.u()).into())
                    {
                        if snub {
                            new_edges.insert((a.u(), b.u()).into());
                            let m = Face::new(vec![a.u(), b.u(), a.v()]);
                            let n = Face::new(vec![a.u(), b.u(), b.v()]);
                            self.cycles.push(m);
                            self.cycles.push(n);
                        } else {
                            let quad = Face::new(vec![a.u(), b.v(), b.u(), a.v()]);
                            self.cycles.push(quad);
                        }
                        solved_edges.insert(a);
                        solved_edges.insert(b);
                    }
                }
            }
        }
        self.edges = HashSet::new();
        self.edges.extend(new_edges.clone());
        self.edges.extend(face_edges);
        new_edges
    }

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
        shape.truncate(None);
    }

    #[test]
    fn contract_edge() {
        let mut graph = PolyGraph::prism(4);
        assert_eq!(graph.vertices.len(), 8);
        assert_eq!(graph.edges.len(), 12);

        graph.contract_edge((0, 1));
        graph.pst();

        assert_eq!(graph.vertices.len(), 7);
        assert_eq!(graph.edges.len(), 11);
    }

    #[test]
    fn split_vertex() {
        let mut graph = PolyGraph::prism(4);
        assert_eq!(graph.vertices.len(), 8);
        assert_eq!(graph.edges.len(), 12);

        graph.split_vertex(0);
        graph.pst();

        assert_eq!(graph.vertices.len(), 10);
        assert_eq!(graph.edges.len(), 15);
    }
}
