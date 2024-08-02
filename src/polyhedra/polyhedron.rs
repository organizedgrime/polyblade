use super::*;
use crate::Vertex;
use ultraviolet::{Slerp, Vec3, Vec4};

const TICK_SPEED: f32 = 1000.0;

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
                            *self.positions.get_mut(v).unwrap() = v_position.slerp(u_position, f);
                            *self.positions.get_mut(u).unwrap() = u_position.slerp(v_position, f);
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
        // self.process_transactions();
        self.apply_spring_forces();
    }

    fn face_positions(&self, face_index: usize) -> Vec<Vec3> {
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

    fn face_triangle_positions(&self, face_index: usize) -> Vec<Vec3> {
        let positions = self.face_positions(face_index);
        let n = positions.len();
        match n {
            3 => positions,
            4 => vec![
                positions[0],
                positions[1],
                positions[2],
                positions[2],
                positions[3],
                positions[0],
            ],
            _ => {
                let centroid = self.face_centroid(face_index);
                let n = positions.len();
                (0..n).fold(vec![], |acc, i| {
                    [acc, vec![positions[i], centroid, positions[(i + 1) % n]]].concat()
                })
            }
        }
    }

    fn face_sides_buffer(&self, face_index: usize) -> Vec<Vec3> {
        let positions = self.face_positions(face_index);
        let n = positions.len();
        match n {
            3 => vec![Vec3::new(1.0, 1.0, 1.0); 3],
            4 => vec![Vec3::new(1.0, 0.0, 1.0); 6],
            _ => vec![Vec3::new(0.0, 1.0, 0.0); n * 3],
        }
    }

    pub fn positions(&self) -> Vec<Vec3> {
        (1..self.cycles.len()).fold(Vec::new(), |acc, i| {
            [acc, self.face_triangle_positions(i)].concat()
        })
    }

    pub fn vertices(&self, clear_face: Option<usize>, palette: &[wgpu::Color]) -> Vec<Vertex> {
        let mut vertices = Vec::new();
        let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];

        let mut polygon_sizes: Vec<usize> = self.cycles.iter().fold(Vec::new(), |mut acc, f| {
            if !acc.contains(&f.len()) {
                acc.push(f.len());
            }
            acc
        });

        polygon_sizes.sort();

        for i in 1..self.cycles.len() {
            let color_index = polygon_sizes
                .iter()
                .position(|&x| x == self.cycles[i].len())
                .unwrap();

            let n = polygon_sizes.get(color_index).unwrap();
            let color = palette[n % palette.len()];
            let color = Vec4::new(
                color.r as f32,
                color.g as f32,
                color.b as f32,
                if Some(i) == clear_face { 0.0 } else { 1.0 },
            );
            let sides = self.face_sides_buffer(i);
            let positions = self.face_triangle_positions(i);

            for j in 0..positions.len() {
                let b = barycentric[j % barycentric.len()];
                vertices.push(Vertex {
                    sides: Vec4::new(sides[j].x, sides[j].y, sides[j].z, 0.0),
                    barycentric: Vec4::new(b.x, b.y, b.z, 0.0),
                    color,
                });
            }
        }

        vertices
    }

    /* pub fn process_transactions(&mut self) {
        if let Some(transaction) = self.transactions.first().cloned() {
            use Transaction::*;
            match transaction {
                Contraction(edges) => {
                    if edges.iter().fold(true, |acc, e| {
                        if self.positions.contains_key(&e.v())
                            && self.positions.contains_key(&e.u())
                        {
                            acc && self.positions[&e.v()].distance(self.positions[&e.u()]) < 0.08
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
                                Wait(Instant::now() + Duration::from_secs(1)),
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
    } */
}
