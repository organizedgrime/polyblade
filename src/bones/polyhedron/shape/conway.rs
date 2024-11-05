use std::collections::{HashMap, HashSet};

use crate::bones::{Cycle, Shape, VertexId};

impl Shape {
    pub fn ordered_face_indices(&self, v: VertexId) -> Vec<usize> {
        let relevant = (0..self.cycles.len())
            .filter(|&i| self.cycles[i].contains(&v))
            .collect::<Vec<usize>>();

        let mut edges: HashMap<[usize; 2], usize> = HashMap::default();

        for &i in relevant.iter() {
            let ui = self.cycles[i].iter().position(|&x| x == v).unwrap();
            // Find the values that came before and after in the face
            let a = self.cycles[i][ui - 1];
            let b = self.cycles[i][ui + 1];
            edges.insert([a, b], i);
        }

        log::info!("edges::: {edges:?}");
        let f: Cycle = edges.keys().cloned().collect::<Vec<_>>().into();

        let mut ordered_face_indices = vec![];
        for i in 0..f.len() {
            let v = f[i];
            let u = f[i + 1];
            log::info!("e::: [{v}, {u}]");
            let fi = edges.get(&[v, u]).or_else(|| edges.get(&[u, v])).unwrap();
            ordered_face_indices.push(*fi);
        }

        ordered_face_indices
    }

    pub fn truncate(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
        log::info!("ofi: {:?}", self.ordered_face_indices(0));
        let edges = self.distance.truncate(degree);
        self.recompute();
        edges
    }

    pub fn ambo(&mut self) -> Vec<[VertexId; 2]> {
        let edges = self.distance.ambo();
        self.recompute();
        edges
    }

    pub fn kis(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
        let edges = self.distance.edges().collect();
        // let mut cycles = self.cycles.clone();
        if let Some(degree) = degree {
            self.cycles
                .iter()
                .collect::<Vec<_>>()
                .retain(|c| c.len() == degree);
        }
        for cycle in self.cycles.iter() {
            let v = self.distance.insert();
            // let mut vpos = Vec3::zero();

            for &u in cycle.iter() {
                self.distance.connect([v, u]);
                //vpos += self.positions[&u];
            }

            //self.positions.insert(v, vpos / cycle.len() as f32);
        }

        self.recompute();
        edges
    }
}
