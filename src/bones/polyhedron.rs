use super::*;
use crate::render::message::ConwayMessage;
use std::time::{Duration, Instant};
use ultraviolet::{Lerp, Vec3};

const TICK_SPEED: f32 = 800.0;

// Operations
impl PolyGraph {
    fn apply_spring_forces(&mut self) {
        let diam = *self.dist.values().max().unwrap_or(&1) as f32;
        let l_diam = self.edge_length * 2.0;
        for v in self.vertices.iter() {
            for u in self.vertices.iter() {
                if u != v {
                    let e: Edge = (v, u).into();
                    if let Some(Transaction::Contraction(contracting_edges)) =
                        self.transactions.first()
                    {
                        if contracting_edges.contains(&e) {
                            let v_position = self.positions[v];
                            let u_position = self.positions[u];
                            let l = (v_position - u_position).mag();
                            let f = (self.edge_length / TICK_SPEED * 4.5) / l;
                            *self.positions.get_mut(v).unwrap() = v_position.lerp(u_position, f);
                            *self.positions.get_mut(u).unwrap() = u_position.lerp(v_position, f);
                            continue;
                        }
                    } else if self.dist.contains_key(&e) {
                        let d = self.dist[&e] as f32;
                        let l = l_diam * (d / diam);
                        let k = 1.0 / d;
                        let diff = self.positions[v] - self.positions[u];
                        let dist = diff.mag();
                        let distention = l - dist;
                        let restorative_force = k / 2.0 * distention;
                        let f = diff * restorative_force / TICK_SPEED;

                        let v_speed = self.speeds.get_mut(v).unwrap();
                        *v_speed += f;
                        *v_speed *= 0.92;
                        *self.positions.get_mut(v).unwrap() += *v_speed;

                        let u_speed = self.speeds.get_mut(u).unwrap();
                        *u_speed -= f;
                        *u_speed *= 0.92;
                        *self.positions.get_mut(u).unwrap() += *u_speed;
                    }
                }
            }
        }
    }

    fn center(&mut self) {
        let shift =
            self.positions.values().fold(Vec3::zero(), |a, &b| a + b) / self.vertices.len() as f32;

        for (_, v) in self.positions.iter_mut() {
            *v -= shift;
        }
    }

    fn resize(&mut self) {
        let mean_length = self.positions.values().map(|p| p.mag()).fold(0.0, f32::max);
        let distance = mean_length - 1.0;
        self.edge_length -= distance / TICK_SPEED;
    }

    pub fn update(&mut self) {
        self.center();
        self.resize();
        self.process_transactions();
        self.apply_spring_forces();
    }

    pub fn face_positions(&self, face_index: usize) -> Vec<Vec3> {
        self.cycles[face_index]
            .iter()
            .map(|v| self.positions[v])
            .collect()
    }

    pub fn face_centroid(&self, face_index: usize) -> Vec3 {
        // All vertices associated with this face
        let vertices: Vec<_> = self.face_positions(face_index);
        vertices.iter().fold(Vec3::zero(), |a, &b| a + b) / vertices.len() as f32
    }

    pub fn vertex_count(&self) -> u64 {
        let mut vertex_triangle_count = 0;
        for face in self.cycles.iter() {
            match face.len() {
                3 => {
                    vertex_triangle_count += 3;
                }
                4 => {
                    vertex_triangle_count += 6;
                }
                _ => {
                    vertex_triangle_count += 3 * face.len() as u64;
                }
            }
        }
        vertex_triangle_count
    }

    pub fn process_transactions(&mut self) {
        if let Some(transaction) = self.transactions.first().cloned() {
            use Transaction::*;
            match transaction {
                Contraction(edges) => {
                    if edges.iter().fold(true, |acc, e| {
                        if self.positions.contains_key(&e.v())
                            && self.positions.contains_key(&e.u())
                        {
                            acc && (self.positions[&e.v()] - self.positions[&e.u()]).mag() < 0.08
                        } else {
                            acc
                        }
                    }) {
                        // Contract them in the graph
                        self.contract_edges(edges);
                        self.pst();
                        //self.find_cycles();
                        self.transactions.remove(0);
                    }
                }
                Release(edges) => {
                    for e in edges.into_iter() {
                        self.disconnect(e);
                    }
                    self.pst();
                    self.find_cycles();
                    self.transactions.remove(0);
                }
                Conway(conway) => {
                    self.transactions.remove(0);
                    use ConwayMessage::*;
                    use Transaction::*;
                    let new_transactions = match conway {
                        Dual => {
                            let edges = self.expand(false);
                            vec![
                                Wait(Instant::now() + Duration::from_millis(500)),
                                Contraction(edges),
                                Name('d'),
                            ]
                        }
                        Join => {
                            let edges = self.kis(Option::None);
                            vec![
                                //Wait(Instant::now() + Duration::from_secs(1)),
                                Release(edges),
                                Name('j'),
                            ]
                        }
                        Ambo => {
                            let edges = self.ambo();
                            vec![Contraction(edges), Name('a')]
                        }
                        Kis => {
                            self.kis(Option::None);
                            vec![Name('k')]
                        }
                        Truncate => {
                            self.truncate(Option::None);
                            vec![Name('t')]
                        }
                        Expand => {
                            self.expand(false);
                            vec![Name('e')]
                        }
                        Snub => {
                            self.expand(true);
                            vec![Name('s')]
                        }
                        Bevel => {
                            let edges = self.bevel();
                            vec![Contraction(edges), Name('b')]
                        }
                    };
                    self.cycles.sort_by_key(|c| c.len());
                    println!("cycles {:?}", self.cycles);
                    self.transactions = [new_transactions, self.transactions.clone()].concat();
                    self.pst();
                }
                Name(c) => {
                    self.name = format!("{c}{}", self.name);
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
