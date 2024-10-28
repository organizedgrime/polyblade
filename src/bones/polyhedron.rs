use crate::{
    bones::{Edge, PolyGraph, Transaction},
    render::message::ConwayMessage,
};
use std::time::{Duration, Instant};
use ultraviolet::{Lerp, Vec3};

use super::VertexId;

const TICK_SPEED: f32 = 10.0;
const SPEED_DAMPENING: f32 = 0.92;

// Operations
impl PolyGraph {
    fn apply_spring_forces(&mut self, second: f32) {
        let diameter = self.graph.diameter();
        let diameter_spring_length = self.edge_length * 2.0;
        let (edges, contracting): (std::slice::Iter<[VertexId; 2]>, bool) =
            if let Some(Transaction::Contraction(edges)) = self.transactions.first() {
                (edges.iter(), true)
            } else {
                (self.springs.iter(), false)
            };

        for &[v, u] in edges {
            let diff = self.positions[v] - self.positions[u];
            let spring_length = diff.mag();
            if contracting {
                let f = ((self.edge_length / TICK_SPEED * second) * 10.0) / spring_length;
                self.positions[v] = self.positions[v].lerp(self.positions[u], f);
                self.positions[u] = self.positions[u].lerp(self.positions[v], f);
            } else {
                let target_length =
                    diameter_spring_length * (self.graph[[v, u]] as f32 / diameter as f32);
                let f = diff * (target_length - spring_length) / TICK_SPEED * second;
                self.speeds[v] = (self.speeds[v] + f) * SPEED_DAMPENING;
                self.speeds[u] = (self.speeds[u] - f) * SPEED_DAMPENING;
                self.positions[v] += self.speeds[v];
                self.positions[u] += self.speeds[u];
            }
        }
    }

    fn center(&mut self) {
        let shift =
            self.positions.iter().fold(Vec3::zero(), |a, &b| a + b) / self.graph.len() as f32;

        for p in self.positions.iter_mut() {
            *p -= shift;
        }
    }

    fn resize(&mut self, second: f32) {
        let mean_length = self.positions.iter().map(|p| p.mag()).fold(0.0, f32::max);
        let matrixance = mean_length - 1.0;
        self.edge_length -= matrixance / TICK_SPEED * second;
    }

    pub fn update(&mut self, second: f32) {
        self.center();
        self.resize(second);
        self.apply_spring_forces(second);
        self.process_transactions();
    }

    pub fn face_positions(&self, face_index: usize) -> Vec<Vec3> {
        self.graph.cycles[face_index]
            .iter()
            .map(|&v| self.positions[v])
            .collect()
    }

    pub fn face_centroid(&self, face_index: usize) -> Vec3 {
        // All vertices associated with this face
        let vertices: Vec<_> = self.face_positions(face_index);
        vertices.iter().fold(Vec3::zero(), |a, &b| a + b) / vertices.len() as f32
    }

    pub fn process_transactions(&mut self) {
        if let Some(transaction) = self.transactions.first().cloned() {
            use Transaction::*;
            match transaction {
                Contraction(edges) => {
                    if !edges
                        .iter()
                        .any(|&[v, u]| (self.positions[v] - self.positions[u]).mag() > 0.08)
                    {
                        // Contract them in the graph
                        self.graph.contract_edges(edges);
                        self.graph.pst();
                        self.springs();
                        self.transactions.remove(0);
                    }
                }
                Release(edges) => {
                    for e in edges.into_iter() {
                        self.disconnect(e);
                    }
                    self.graph.pst();
                    self.graph.find_cycles();
                    self.springs();
                    self.transactions.remove(0);
                }
                Conway(conway) => {
                    self.transactions.remove(0);
                    use ConwayMessage::*;
                    use Transaction::*;
                    let new_transactions = match conway {
                        Dual => {
                            let edges = self.graph.expand(false);
                            vec![
                                Wait(Instant::now() + Duration::from_millis(650)),
                                Contraction(edges),
                                Name('d'),
                            ]
                        }
                        Join => {
                            let edges = self.graph.kis(Option::None);
                            vec![
                                //Wait(Instant::now() + Duration::from_secs(1)),
                                Release(edges),
                                Name('j'),
                            ]
                        }
                        Ambo => {
                            let edges = self.graph.ambo();
                            vec![Contraction(edges), Name('a')]
                        }
                        Kis => {
                            self.graph.kis(Option::None);
                            vec![Name('k')]
                        }
                        Truncate => {
                            self.graph.truncate(Option::None);
                            vec![Name('t')]
                        }
                        Expand => {
                            self.graph.expand(false);
                            vec![Name('e')]
                        }
                        Snub => {
                            self.graph.expand(true);
                            vec![Name('s')]
                        }
                        Bevel => {
                            vec![
                                Conway(Truncate),
                                Wait(Instant::now() + Duration::from_millis(500)),
                                Conway(Ambo),
                                Name('b'),
                            ]
                        }
                    };
                    //self.graph.cycles.sort_by_key(|c| usize::MAX - c.len());
                    self.transactions = [new_transactions, self.transactions.clone()].concat();
                    self.graph.pst();
                    self.graph.find_cycles();
                    println!("cycles {:?}", self.graph.cycles);
                    println!("verts {:?}", self.graph.vertices());
                    self.springs();
                }
                Name(c) => {
                    if c == 'b' {
                        self.name = self.name[2..].to_string();
                    }
                    if c == 'd' && &self.name[0..1] == "d" {
                        self.name = self.name[1..].to_string();
                    } else {
                        self.name = format!("{c}{}", self.name);
                    }
                    self.transactions.remove(0);
                }
                ShortenName(n) => {
                    self.name = self.name[n..].to_string();
                    self.transactions.remove(0);
                }
                Wait(instant) => {
                    if Instant::now() > instant {
                        self.transactions.remove(0);
                    }
                }
                None => {}
            }
        }
    }
}
