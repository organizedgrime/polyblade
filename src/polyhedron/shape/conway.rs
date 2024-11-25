use std::collections::{HashMap, HashSet};

use super::{Cycles, Shape};
use crate::polyhedron::{shape::Cycle, VertexId};

//use crate::bones::{Cycle, Shape, VertexId};

impl Shape {
    pub fn split_vertex(&mut self, v: VertexId) -> Vec<[usize; 2]> {
        let sc = self.cycles.sorted_connections(v);
        // log::info!("sc_{v}: {sc:?}");
        let edges = self.distance.split_vertex(v, sc);
        self.cycles = Cycles::from(&self.distance);
        edges
    }

    pub fn ordered_face_indices(&self, v: VertexId) -> Vec<usize> {
        let relevant = (0..self.cycles.len())
            .filter(|&i| self.cycles[i].contains(&v))
            .collect::<Vec<usize>>();

        let mut edges: HashMap<[usize; 2], usize> = HashMap::default();

        for &i in relevant.iter() {
            let ui = self.cycles[i].iter().position(|&x| x == v).unwrap();
            // Find the values that came before and after in the face
            let a = self.cycles[i][ui + self.cycles[i].len() - 1];
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
    //
    pub fn expand(&mut self, snub: bool) -> Vec<[VertexId; 2]> {
        let mut new_edges = HashSet::<[VertexId; 2]>::default();
        let mut face_edges = HashSet::<[VertexId; 2]>::default();

        let ordered_face_indices: HashMap<usize, Vec<usize>> = self
            .vertices()
            .map(|v| (v, self.ordered_face_indices(v)))
            .collect();

        // For every vertex
        for v in self.vertices() {
            //let original_position = self.positions[&v];
            let mut new_face = Cycle::default();
            // For every face which contains the vertex
            for &i in ordered_face_indices.get(&v).unwrap() {
                // Create a new vertex
                let u = self.distance.insert();
                // Replace it in the face
                self.cycles[i].replace(v, u);
                // Now replace
                let ui = self.cycles[i].iter().position(|&x| x == u).unwrap();
                let flen = self.cycles[i].len();
                // Find the values that came before and after in the face
                let a = self.cycles[i][(ui + flen - 1) % flen];
                let b = self.cycles[i][(ui + 1) % flen];
                // Remove existing edges which may no longer be accurate
                new_edges.remove(&[a, v]);
                new_edges.remove(&[b, v]);
                // Add the new edges which are so yass
                new_edges.insert([a, u]);
                new_edges.insert([b, u]);
                // Add u to the new face being formed
                new_face.push(u);
                // pos
                //self.positions.insert(u, original_position);
            }
            for i in 0..new_face.len() {
                face_edges.insert((new_face[i], new_face[(i + 1) % new_face.len()]).into());
            }
            //self.recompute();
            self.distance.delete(v);
        }

        for &e in new_edges.iter() {
            self.distance.connect(e);
        }
        for f in face_edges {
            self.distance.connect(f);
        }
        new_edges.into_iter().collect()
    }

    /// `t` truncate
    // pub fn truncate(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
    //     // let mut vertices = self.distance.vertices().clone().collect::<Vec<_>>();
    //     // if let Some(degree) = degree {
    //     //     vertices.retain(|&v| self.distance.connections(v).len() == degree);
    //     // }
    //     // for v in vertices {
    //     //     new_edges.extend(self.split_vertex(v));
    //     // }
    //     Vec::default()
    // }

    pub fn contract_edges(&mut self, edges: Vec<[VertexId; 2]>) {
        self.distance.contract_edges(edges);
        self.recompute();
    }

    // pub fn truncate(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
    //     log::info!("sorted_connections: {:?}", self.sorted_connections(0));
    //     log::info!("connections: {:?}", self.distance.connections(0));
    //     let edges = self.distance.truncate(degree);
    //     self.recompute();
    //     edges
    // }

    // pub fn ambo(&mut self) -> Vec<[VertexId; 2]> {
    //     let edges = self.distance.ambo();
    //     self.recompute();
    //     edges
    // }

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

    pub fn chamfer(&mut self) {
        let originals = self.edges().collect::<Vec<_>>();
        for cycle in self.cycles.iter() {
            let mut new_face = vec![];
            for &v in cycle.iter() {
                let u = self.distance.insert();
                new_face.push(u);
                self.distance.connect([v, u]);
            }
            for i in 0..new_face.len() {
                self.distance
                    .connect([new_face[i], new_face[(i + 1) % new_face.len()]]);
            }
        }
        for edge in originals {
            self.distance.disconnect(edge);
        }
        self.recompute();
    }
}
