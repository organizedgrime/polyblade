use super::Distance;
use crate::polyhedron::shape::Cycle;
use crate::polyhedron::VertexId;

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

    pub fn cycle_is_face(&self, mut cycle: Vec<VertexId>) -> bool {
        let mut dupe = self.clone();
        while !cycle.is_empty() {
            let v = cycle.remove(0);
            dupe.delete(v);
            for u in &mut cycle {
                if *u > v {
                    *u -= 1;
                }
            }
        }
        dupe.is_connected()
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

    //
    // `j` join
    // `z` zip
    // `g` gyro
    // `m` meta = `kj`
    // `o` ortho = `jj`
    // `n` needle
    // `k` kis
}
