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
