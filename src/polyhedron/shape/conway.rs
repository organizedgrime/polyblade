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

    pub fn expand(&mut self, snub: bool) -> Vec<[VertexId; 2]> {
        let mut new_edges = HashSet::<[VertexId; 2]>::default();
        let mut face_edges = HashSet::<[VertexId; 2]>::default();

        let all_connections: HashMap<usize, Cycle> = self
            .vertices()
            .map(|v| (v, Cycle::from(self.cycles.sorted_connections(v))))
            .collect();

        let removers: Vec<_> = self.vertices().collect();
        // For every vertex
        for v in self.vertices() {
            //let original_position = self.positions[&v];
            let mut new_face = Cycle::default();

            // For each connection
            for i in all_connections[&v].iter() {
                let u = self.distance.insert();
                let ui = all_connections[i].iter().position(|&x| x == v).unwrap();
                let flen = all_connections[i].len();
                let a = all_connections[i][ui + flen - 1];
                let b = all_connections[i][ui + 1];
                self.distance.disconnect([a, v]);
                self.distance.disconnect([b, v]);
                self.distance.connect([a, u]);
                self.distance.connect([b, u]);
                new_face.push(u);
            }
            for i in 0..new_face.len() {
                self.distance.connect([new_face[i], new_face[i + 1]]);
                // face_edges.insert((new_face[i], new_face[(i + 1) % new_face.len()]).into());
            }
            //self.recompute();
            //self.cycles = Cycles::from(&self.distance);
        }

        // for &e in new_edges.iter() {
        //     self.distance.connect(e);
        // }
        // for f in face_edges {
        //     self.distance.connect(f);
        // }
        for v in removers {
            self.distance.delete(v);
        }
        self.recompute();
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
