use cgmath::{vec3, InnerSpace, MetricSpace, Vector3, VectorSpace, Zero};

use super::*;
use crate::prelude::{V3f, HSL};
use std::{collections::HashSet, ops::Add};

const TICK_SPEED: f32 = 100.0;

// Operations
impl PolyGraph {
    fn apply_forces(&mut self, edges: HashSet<Edge>, l: f32, k: f32) {
        for (v, u) in edges.into_iter().map(|e| e.id()) {
            if self.contracting_edges.contains(&(v, u).into()) {
                let vp = self.positions[&v];
                let up = self.positions[&u];
                let l = vp.distance(up);
                let f = (self.edge_length / TICK_SPEED * 3.0) / l;
                *self.positions.get_mut(&v).unwrap() = vp.lerp(up, f);
                *self.positions.get_mut(&u).unwrap() = up.lerp(vp, f);
            } else {
                let diff = self.positions[&v] - self.positions[&u];
                let dist = diff.magnitude();
                let distention = l - dist;
                let restorative_force = k / 2.0 * distention;
                let f = diff * restorative_force / TICK_SPEED;

                // Add forces
                *self.speeds.get_mut(&v).unwrap() += f;
                *self.speeds.get_mut(&u).unwrap() -= f;

                // Apply damping
                *self.speeds.get_mut(&v).unwrap() *= 0.92;
                *self.speeds.get_mut(&u).unwrap() *= 0.92;

                *self.positions.get_mut(&v).unwrap() += self.speeds[&v];
                *self.positions.get_mut(&u).unwrap() += self.speeds[&u];
            }
        }
    }

    fn apply_spring_forces(&mut self) {
        let diam = *self.dist.values().max().unwrap_or(&1) as f32;
        // Natural lengths
        let l_d = self.edge_length * 2.0;
        let l_a = l_d / (diam * 3.0);
        let l_n = l_a * 2.2;

        // Spring constants
        let k_a = 0.9;
        let k_n = 0.4;
        let k_d = 0.3;

        // Apply!
        self.apply_forces(self.adjacents.clone(), l_a, k_a);
        self.apply_forces(self.neighbors.clone(), l_n, k_n);
        self.apply_forces(self.diameter.clone(), l_d, k_d);
    }

    fn center(&mut self) {
        let shift = self.positions.values().fold(Vector3::zero(), Vector3::add)
            / self.vertices.len() as f32;

        for (_, v) in self.positions.iter_mut() {
            *v -= shift;
        }
    }

    fn resize(&mut self) {
        let mean_magnitude = self
            .positions
            .values()
            .map(|p| p.magnitude())
            .fold(0.0, f32::max);
        let distance = mean_magnitude - 1.0;

        self.edge_length -= distance / TICK_SPEED;
    }

    pub fn update(&mut self) {
        self.center();
        self.resize();
        self.animate_contraction();
        self.apply_spring_forces();
    }

    fn face_xyz(&self, face_index: usize) -> Vec<Vector3<f32>> {
        self.faces[face_index]
            .iter()
            .map(|v| self.positions[v])
            .collect()
    }

    pub fn face_normal(&self, face_index: usize) -> V3f {
        self.faces[face_index]
            .iter()
            .map(|v| self.positions[v])
            .fold(Vector3::zero(), |acc, v| acc.cross(v))
            .normalize()
    }

    fn face_centroid(&self, face_index: usize) -> Vector3<f32> {
        // All vertices associated with this face
        let vertices: Vec<_> = self.face_xyz(face_index);
        vertices.iter().fold(Vector3::zero(), Vector3::add) / vertices.len() as f32
    }

    fn face_xyz_buffer(&self, face_index: usize) -> Vec<V3f> {
        let positions = self.face_xyz(face_index);
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

    fn face_tri_buffer(&self, face_index: usize) -> Vec<V3f> {
        let positions = self.face_xyz(face_index);
        let n = positions.len();
        match n {
            3 => vec![vec3(1.0, 1.0, 1.0); 3],
            4 => vec![vec3(1.0, 0.0, 1.0); 6],
            _ => vec![vec3(0.0, 1.0, 0.0); n * 3],
        }
    }

    pub fn xyz_buffer(&self) -> Vec<Vector3<f32>> {
        (0..self.faces.len()).fold(Vec::new(), |acc, i| [acc, self.face_xyz_buffer(i)].concat())
    }

    pub fn poly_color(n: usize) -> V3f {
        let colors = vec![
            vec3(72.0, 132.0, 90.0),
            vec3(163.0, 186.0, 112.0),
            vec3(51.0, 81.0, 69.0),
            vec3(254.0, 240.0, 134.0),
            vec3(95.0, 155.0, 252.0),
            vec3(244.0, 164.0, 231.0),
            vec3(170.0, 137.0, 190.0),
        ];

        colors[n % colors.len()]
    }

    pub fn static_buffer(&self) -> (Vec<V3f>, Vec<V3f>, Vec<V3f>) {
        let mut rgb = Vec::new();
        let mut bsc = Vec::new();
        let mut tri = Vec::new();

        for face_index in 0..self.faces.len() {
            let face_tri = self.face_tri_buffer(face_index);
            let color = Self::poly_color(self.faces[face_index].len()) / 255.0;
            rgb.extend(vec![color; face_tri.len()]);
            tri.extend(face_tri);
        }

        for _ in 0..rgb.len() / 3 {
            bsc.extend(vec![
                Vector3::unit_x(),
                Vector3::unit_y(),
                Vector3::unit_z(),
            ]);
        }

        (rgb, bsc, tri)
    }

    pub fn animate_contraction(&mut self) {
        // If all edges are contracted visually
        if !self.contracting_edges.is_empty()
            && self.contracting_edges.iter().fold(true, |acc, e| {
                if self.positions.contains_key(&e.v()) && self.positions.contains_key(&e.u()) {
                    acc && self.positions[&e.v()].distance(self.positions[&e.u()]) < 0.08
                } else {
                    acc
                }
            })
        {
            // Contract them in the graph
            for e in self.contracting_edges.clone() {
                self.contract_edge(e);
            }
            self.contracting_edges = HashSet::new();
            self.recompute_qualities();
            self.name.truncate(self.name.len() - 1);
            self.name += "a";
        }
    }
}
