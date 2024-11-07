use std::time::{Duration, Instant};

use crate::{
    bones::*,
    render::message::{ConwayMessage, PresetMessage},
};
use rustc_hash::FxHashMap as HashMap;
use ultraviolet::{Lerp, Vec3};
type VertMap<T> = HashMap<VertexId, T>;
pub type VertexId = usize;
pub const TICK_SPEED: f32 = 10.0;
pub const SPEED_DAMPENING: f32 = 0.92;

#[derive(Debug, Clone)]
pub struct Polyhedron {
    /// Conway Polyhedron Notation
    pub name: String,
    /// The shape we're rendering
    pub shape: Shape,
    /// The properties
    pub render: Render,
    /// Transaction queue
    pub transactions: Vec<Transaction>,
}

impl Polyhedron {
    pub fn process_transactions(&mut self) {
        if let Some(transaction) = self.transactions.first().cloned() {
            use Transaction::*;
            let result = match transaction {
                Contraction(edges) => {
                    if !edges.iter().any(|&[v, u]| {
                        (self.render.positions[v] - self.render.positions[u]).mag() > 0.08
                    }) {
                        // Contract them in the graph
                        self.shape.contraction(&edges);
                        self.transactions.remove(0);
                    }
                }
                Release(edges) => {
                    self.shape.release(&edges);
                    self.transactions.remove(0);
                }
                Conway(conway) => {
                    self.transactions.remove(0);
                    use ConwayMessage::*;
                    use Transaction::*;
                    let new_transactions = match conway {
                        Dual => {
                            // let edges = self.shape.expand(false);
                            // vec![
                            //     Wait(Instant::now() + Duration::from_millis(650)),
                            //     Contraction(edges),
                            //     Name('d'),
                            // ]
                            todo!()
                        }
                        Join => {
                            // let edges = self.graph.kis(Option::None);
                            // vec![
                            //     //Wait(Instant::now() + Duration::from_secs(1)),
                            //     Release(edges),
                            //     Name('j'),
                            // ]
                            todo!()
                        }
                        Ambo => {
                            let edges = self.shape.ambo();
                            self.shape.recompute();
                            vec![Contraction(edges), Name('a')]
                        }
                        Kis => {
                            // self.graph.kis(Option::None);
                            // vec![Name('k')]
                            todo!()
                        }
                        Truncate => {
                            self.shape.truncate(Option::None);
                            self.shape.recompute();

                            vec![Name('t')]
                        }
                        Expand => {
                            // self.shape.expand(false);
                            // vec![Name('e')]
                            todo!()
                        }
                        Snub => {
                            // self.graph.expand(true);
                            // vec![Name('s')]
                            todo!()
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
                    self.render.new_capacity(self.shape.len());
                    self.transactions = [new_transactions, self.transactions.clone()].concat();
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
            };
            self.shape.compute_graph_svg();
            result
        }
    }

    pub fn update(&mut self, second: f32) {
        self.render.update(second);
        self.apply_spring_forces(second);
        self.process_transactions();
    }

    fn apply_spring_forces(&mut self, second: f32) {
        //println!("self: {:?}", self);
        let diameter = self.shape.distance.diameter();
        let diameter_spring_length = self.render.edge_length * 2.0;
        let (edges, contracting): (std::slice::Iter<[VertexId; 2]>, bool) =
            if let Some(Transaction::Contraction(edges)) = self.transactions.first() {
                (edges.iter(), true)
            } else {
                (self.shape.springs.iter(), false)
            };

        for &[w, x] in edges {
            let v = x.min(w);
            let u = x.max(w);

            let diff = self.render.positions[v] - self.render.positions[u];
            let spring_length = diff.mag();
            if contracting {
                log::warn!("CONTRACTING");
                let f = ((self.render.edge_length / TICK_SPEED * second) * 10.0) / spring_length;
                self.render.positions[v] =
                    self.render.positions[v].lerp(self.render.positions[u], f);
                self.render.positions[u] =
                    self.render.positions[u].lerp(self.render.positions[v], f);
            } else {
                let target_length =
                    diameter_spring_length * (self.shape.distance[[v, u]] as f32 / diameter as f32);
                let f = diff * (target_length - spring_length) / TICK_SPEED * second;
                self.render.apply_force([v, u], f);
            }
        }
    }

    pub fn preset(preset: &PresetMessage) -> Polyhedron {
        let shape = Shape::preset(preset);
        let render = Render::new(shape.distance.len());
        Polyhedron {
            name: preset.to_string(),
            shape,
            render,
            transactions: vec![],
        }
    }

    pub fn face_centroid(&self, face_index: usize) -> Vec3 {
        // All vertices associated with this face
        self.shape.cycles[face_index]
            .iter()
            .map(|&v| self.render.positions[v])
            .fold(Vec3::zero(), |a, b| a + b)
            / self.shape.cycles[face_index].len() as f32
    }

    // fn face_positions(&self, face_index: usize) -> Vec<Vec3> {
    //     self.shape.cycles[face_index]
    //         .iter()
    //         .map(|&v| self.render.vertices[v].position)
    //         .collect()
    // }
    // Use a Fibonacci Lattice to spread the points evenly around a sphere
    // pub fn connect(&mut self, [v, u]: [VertexId; 2]) {
    //     self.graph.connect([v, u]);
    // }
    //
    // pub fn disconnect(&mut self, [v, u]: [VertexId; 2]) {
    //     self.graph.disconnect([v, u]);
    // }
    //
    // pub fn insert(&mut self) -> VertexId {
    //     self.positions
    //         .push(Vec3::new(random(), random(), random()).normalized());
    //     self.speeds.push(Vec3::zero());
    //     self.graph.insert()
    // }

    // pub fn delete(&mut self, v: VertexId) {
    //     self.vertices.remove(&v);
    //
    //     self.edges = self
    //         .edges
    //         .clone()
    //         .into_iter()
    //         .filter(|e| !e.contains(v))
    //         .collect();
    //
    //     self.cycles = self
    //         .cycles
    //         .clone()
    //         .into_iter()
    //         .map(|face| face.into_iter().filter(|&u| u != v).collect())
    //         .collect();
    //
    //     self.positions.remove(&v);
    //     self.speeds.remove(&v);
    // }
    //
    // /// Edges of a vertex
    // pub fn edges(&self, v: VertexId) -> Vec<Edge> {
    //     let mut edges = vec![];
    //     for u in 0..self.dist.len() {
    //         if self.dist[v][u] == 1 {
    //             edges.push((v, u).into());
    //         }
    //     }
    //     edges
    // }

    // /// Number of faces
    // pub fn face_count(&self) -> i64 {
    //     2 + self.edges.len() as i64 - self.vertices.len() as i64
    // }

    //
    //
    //
}

// impl Display for PolyGraph {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut vertices = self.vertices.iter().collect::<Vec<_>>();
//         vertices.sort();
//         let mut adjacents = self.edges.clone().into_iter().collect::<Vec<_>>();
//         adjacents.sort();
//
//         f.write_fmt(format_args!(
//             "name:\t\t{}\nvertices:\t{:?}\nadjacents:\t{}\nfaces:\t\t{}\n",
//             self.name,
//             vertices,
//             adjacents
//                 .iter()
//                 .fold(String::new(), |acc, e| format!("{e}, {acc}")),
//             self.cycles.iter().fold(String::new(), |acc, f| format!(
//                 "[{}], {acc}",
//                 f.iter().fold(String::new(), |acc, x| format!("{x}, {acc}"))
//             ))
//         ))
//     }
// }
