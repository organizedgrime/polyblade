use super::Distance;
use crate::polyhedron::shape::Cycle;
use crate::polyhedron::VertexId;
use rustc_hash::FxHashSet as HashSet;

impl Distance {
    pub(super) fn contract_edge(&mut self, [v, u]: [VertexId; 2]) {
        // Give u all the same connections as v
        for w in self.connections(v).into_iter() {
            self.connect([w, u]);
            self.disconnect([w, v]);
        }

        // // Delete a
        // for cycle in self.cycles.iter_mut() {
        //     cycle.replace(v, u);
        // }

        // Delete v
        self.delete(v);
    }

    pub fn contract_edges(&mut self, mut edges: Vec<[VertexId; 2]>) {
        // let mut transformed = HashSet::default();
        while !edges.is_empty() {
            // Pop an edge
            let [w, x] = edges.remove(0);
            let v = w.max(x);
            let u = w.min(x);

            // // If this is not a redundant edge
            // if !(transformed.contains(&v) && transformed.contains(&u)) {

            // Contract [v, u], deleting v
            self.contract_edge([v, u]);

            // // Mark that this vertex has been transformed
            // transformed.insert(v);

            // Decrement the value of every vertex
            for [x, w] in &mut edges {
                if *x > v {
                    *x -= 1;
                }
                if *w > v {
                    *w -= 1;
                }
            }
        }
    }

    // pub fn contract_edges(&mut self, mut edges: Vec<[VertexId; 2]>) {
    //     let mut map = HashMap::<VertexId, VertexId>::default();
    //     for [v, u] in edges.into_iter() {
    //         let u = *map.get(&u).unwrap_or(&u);
    //         let v = *map.get(&v).unwrap_or(&v);
    //         if v != u {
    //             //self.contract_edge([v, u]);
    //             map.insert(v, u);
    //         }
    //     }
    //     println!("map: {map:?}");
    // }

    pub fn split_vertex(&mut self, v: VertexId, connections: Vec<VertexId>) -> Vec<[VertexId; 2]> {
        // Remove the vertex
        let new_cycle: Cycle = Cycle::from(
            vec![v]
                .into_iter()
                .chain((1..connections.len()).map(|_| self.insert()))
                .collect(),
        );

        for c in &connections {
            self.disconnect([v, *c]);
        }

        for i in 0..new_cycle.len() {
            self.connect([new_cycle[i], connections[i]]);
        }

        // track the edges that will compose the new face
        let mut new_edges = vec![];
        for i in 0..new_cycle.len() {
            let edge = [new_cycle[i], new_cycle[i + 1]];
            self.connect(edge);
            new_edges.push(edge);
        }

        new_edges
    }

    // //
    // pub fn ordered_face_indices(&self, v: VertexId) -> Vec<usize> {
    //     let relevant = (0..self.cycles.len())
    //         .filter(|&i| self.cycles[i].containz(&v))
    //         .collect::<Vec<usize>>();
    //
    //     let mut edges = HashMap::default();
    //
    //     for &i in relevant.iter() {
    //         let ui = self.cycles[i].iter().position(|&x| x == v).unwrap();
    //         let flen = self.cycles[i].len();
    //         // Find the values that came before and after in the face
    //         let a = self.cycles[i][(ui + flen - 1) % flen];
    //         let b = self.cycles[i][(ui + 1) % flen];
    //         edges.insert((a, b).into(), i);
    //     }
    //
    //     let f: Cycle = edges.keys().cloned().collect::<Vec<_>>().into();
    //
    //     let mut ordered_face_indices = vec![];
    //     for i in 0..f.len() {
    //         let ev = f[i];
    //         let eu = f[(i + 1) % f.len()];
    //         let fi = edges
    //             .get(&[ev, eu])
    //             .unwrap_or(edges.get(&[eu, ev]).unwrap());
    //         ordered_face_indices.push(*fi);
    //     }
    //
    //     ordered_face_indices
    // }
    // //

    // /// `e` = `aa`
    // pub fn expand(&mut self, snub: bool) -> Vec<[VertexId; 2]> {
    //     let mut new_edges = HashSet::<Edge>::default();
    //     let mut face_edges = HashSet::<Edge>::default();
    //
    //     let ordered_face_indices: HashMap<usize, Vec<usize>> = self
    //         .vertices()
    //         .map(|v| (v, self.ordered_face_indices(v)))
    //         .collect();
    //
    //     // For every vertex
    //     for v in self.vertices() {
    //         //let original_position = self.positions[&v];
    //         let mut new_face = Cycle::default();
    //         // For every face which contains the vertex
    //         for &i in ordered_face_indices.get(&v).unwrap() {
    //             // Create a new vertex
    //             let u = self.insert();
    //             // Replace it in the face
    //             self.cycles[i].replace(v, u);
    //             // Now replace
    //             let ui = self.cycles[i].iter().position(|&x| x == u).unwrap();
    //             let flen = self.cycles[i].len();
    //             // Find the values that came before and after in the face
    //             let a = self.cycles[i][(ui + flen - 1) % flen];
    //             let b = self.cycles[i][(ui + 1) % flen];
    //             // Remove existing edges which may no longer be accurate
    //             new_edges.remove(&(a, v).into());
    //             new_edges.remove(&(b, v).into());
    //             // Add the new edges which are so yass
    //             new_edges.insert((a, u).into());
    //             new_edges.insert((b, u).into());
    //             // Add u to the new face being formed
    //             new_face.push(u);
    //             // pos
    //             //self.positions.insert(u, original_position);
    //         }
    //         for i in 0..new_face.len() {
    //             face_edges.insert((new_face[i], new_face[(i + 1) % new_face.len()]).into());
    //         }
    //         self.cycles.push(new_face);
    //         self.delete(v);
    //     }
    //
    //     let mut solved_edges = HashSet::default();
    //
    //     // For every triangle / nf edge
    //     for a in face_edges.iter() {
    //         // find the edge which is parallel to it
    //         for b in face_edges.iter() {
    //             if !solved_edges.contains(a) && !solved_edges.contains(b) {
    //                 if new_edges.contains(&(a.v(), b.v()).into())
    //                     && new_edges.contains(&(a.u(), b.u()).into())
    //                 {
    //                     if snub {
    //                         new_edges.insert((a.v(), b.u()).into());
    //                         let m = Cycle::new(vec![a.v(), b.u(), a.u()]);
    //                         let n = Cycle::new(vec![a.v(), b.u(), b.v()]);
    //                         self.cycles.push(m);
    //                         self.cycles.push(n);
    //                     } else {
    //                         let quad = Cycle::new(vec![b.u(), a.u(), a.v(), b.v()]);
    //                         self.cycles.push(quad);
    //                     }
    //
    //                     solved_edges.insert(a);
    //                     solved_edges.insert(b);
    //                 }
    //
    //                 if new_edges.contains(&(a.u(), b.v()).into())
    //                     && new_edges.contains(&(a.v(), b.u()).into())
    //                 {
    //                     if snub {
    //                         new_edges.insert((a.u(), b.u()).into());
    //                         let m = Cycle::new(vec![a.u(), b.u(), a.v()]);
    //                         let n = Cycle::new(vec![a.u(), b.u(), b.v()]);
    //                         self.cycles.push(m);
    //                         self.cycles.push(n);
    //                     } else {
    //                         let quad = Cycle::new(vec![a.u(), b.v(), b.u(), a.v()]);
    //                         self.cycles.push(quad);
    //                     }
    //                     solved_edges.insert(a);
    //                     solved_edges.insert(b);
    //                 }
    //             }
    //         }
    //     }
    //
    //     // self.edges = HashSet::default();
    //     // self.edges.extend(new_edges.clone());
    //     // self.edges.extend(face_edges);
    //     // new_edges
    //     vec![]
    // }

    //
    // `j` join
    // `z` zip
    // `g` gyro
    // `m` meta = `kj`
    // `o` ortho = `jj`
    // `n` needle
    // `k` kis
}
