use std::collections::{HashMap, HashSet};

pub use super::*;

impl PolyGraph {
    pub fn contract_edge(&mut self, e: impl Into<Edge>) {
        let e: Edge = e.into();
        // If this is the ghost edge, find its extant counterpart
        let id = self.ghost_edges.get(&e).unwrap_or(&e).id();
        // Give b all the same connections as a
        for b in self.connections(&id.0).iter() {
            if b != &id.1 {
                self.connect((b, &id.1))
            }
        }
        // Delete a
        self.delete(&id.0);
        for (_, v) in self.ghost_edges.iter_mut() {
            if let Some(u) = v.other(&id.0) {
                *v = (id.1, u).into();
            }
        }
    }

    pub fn split_vertex(&mut self, v: &VertexId) {
        let mut faces = self.faces.clone();
        let original_position = self.positions[v];
        let mut connections: HashSet<usize> = self.connections(v);
        connections.extend(self.ghost_connections(v));
        connections.remove(v);
        let mut new_guys = Vec::new();

        // Remove the vertex
        self.delete(v);
        'connections: while !connections.is_empty() {
            // closest vertex to the previous which is not itself and is connected
            let u = connections.clone().into_iter().collect::<Vec<_>>()[0];

            // Insert a new node in the same location
            let new_vertex = self.insert();
            new_guys.push(new_vertex);
            self.positions.insert(new_vertex, original_position);
            // Reform old connection
            self.connect((u, new_vertex));

            // Track
            connections.remove(&u);

            // Track the ghost edge and new edge
            let ge: Edge = (*v, u).into();
            let ne: Edge = (new_vertex, u).into();

            // If the ghost of this transaction was the new edge of a previous transaction
            for (_, v) in self.ghost_edges.iter_mut() {
                if v.id() == ge.id() {
                    // Update its child
                    *v = ne;
                    continue 'connections;
                }
            }
            // Track ghost edge directly if one didnt already exist
            self.ghost_edges.insert(ge, ne);
        }

        let mut ccc = HashSet::<Edge>::new();
        for i in 0..self.faces.len() {
            let face = &mut self.faces[i];
            if let Some(pos) = face.0.iter().position(|x| x == v) {
                let flen = face.0.len();
                let before = face.0[(pos + flen - 1) % flen];
                let after = face.0[(pos + 1) % flen];

                let be: Edge = (*v, before).into();
                let ae: Edge = (*v, after).into();

                let gb = self.ghost_edges.get(&be).unwrap();
                let ga = self.ghost_edges.get(&ae).unwrap();

                println!("removing {v}; {be:?} became {gb:?}");
                println!("removing {v}; {ae:?} became {ga:?}");
                let gb = gb.other(&before).unwrap();
                let ga = ga.other(&after).unwrap();

                face.0.remove(pos);
                face.0.insert(pos, ga);
                face.0.insert(pos, gb);

                ccc.insert((gb, ga).into());
            }
        }

        for c in ccc.clone().into_iter() {
            self.connect(c);
        }

        let mut fff = Vec::new();
        loop {
            if ccc.is_empty() {
                break;
            }
            if fff.is_empty() {
                let random = ccc.iter().collect::<Vec<_>>()[0].id().0;
                fff.push(random);
            } else {
                let l = fff.last().unwrap();
                let e = ccc.iter().find(|e| e.other(l).is_some()).unwrap().clone();
                fff.push(e.other(l).unwrap());
                ccc.remove(&e);
            }
        }

        self.faces.push(Face(fff));
        println!("faces: {:?}", self.faces);
    }

    /// `t` truncate
    pub fn truncate(&mut self) {
        for v in self.vertices.clone().iter() {
            self.split_vertex(v);
            self.ghost_edges = HashMap::new();
        }
        self.recompute_qualities();
        self.name.insert(0, 't');
    }

    /// `a` ambo
    pub fn ambo(&mut self) {
        let original_edges = self.adjacents.clone();
        // Truncate
        self.truncate();

        //self.contract_edges_visually(original_edges);
        // Animate

        //self.contracting_edges.extend(original_edges);
        // Contract original edge set
        for edge in original_edges.iter() {
            self.contract_edge(*edge);
        }

        self.recompute_qualities();
        self.ghost_edges = HashMap::new();
        self.name.remove(0);
        self.name.insert(0, 'a');
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
        self.ambo();
        self.ambo();
        self.name.remove(0);
        self.name.remove(0);
        self.name.insert(0, 'e');
    }

    /// `s` snub is applying `e` followed by diagonal addition
    pub fn snub(&mut self) {
        self.expand();
        //self.diagonal_addition();
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
    use crate::prelude::*;

    #[test]
    fn truncate() {
        let mut shape = PolyGraph::icosahedron();
        shape.truncate();
    }

    #[test]
    fn contract_edge() {
        let mut graph = PolyGraph::new_disconnected(6);
        graph.connect((1, 0));
        graph.connect((1, 2));

        graph.connect((1, 3));

        graph.connect((3, 4));
        graph.connect((3, 5));
        graph.recompute_qualities();

        assert_eq!(graph.vertices.len(), 6);
        assert_eq!(graph.adjacents.len(), 5);

        graph.contract_edge((1, 3));
        graph.recompute_qualities();

        assert_eq!(graph.vertices.len(), 5);
        println!("adja: {:?}", graph.adjacents);
        assert_eq!(graph.adjacents.len(), 4);

        assert_eq!(graph.connections(&0), vec![3].into_iter().collect());
        assert_eq!(graph.connections(&2), vec![3].into_iter().collect());

        assert_eq!(
            graph.connections(&3),
            vec![0, 2, 4, 5].into_iter().collect()
        );

        assert_eq!(graph.connections(&4), vec![3].into_iter().collect());
        assert_eq!(graph.connections(&5), vec![3].into_iter().collect());
    }

    #[test]
    fn split_vertex() {
        let mut graph = PolyGraph::new_disconnected(5);
        graph.connect((1, 0));
        graph.connect((1, 2));

        graph.connect((1, 3));
        graph.connect((1, 4));
        graph.recompute_qualities();

        assert_eq!(graph.vertices.len(), 5);
        assert_eq!(graph.adjacents.len(), 4);

        graph.split_vertex(&1);
        graph.recompute_qualities();

        assert_eq!(graph.vertices.len(), 8);
        assert_eq!(graph.adjacents.len(), 8);
    }
}
