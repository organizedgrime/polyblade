use cgmath::{vec3, InnerSpace, MetricSpace, Vector3, VectorSpace, Zero};

use super::*;
use crate::prelude::{V3f, HSL};
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};

const TICK_SPEED: f32 = 200.0;

// Operations
impl PolyGraph {
    fn apply_forces(&mut self, edges: HashSet<Edge>, l: f32, k: f32) {
        for (v, u) in edges.into_iter().map(|e| e.id()) {
            if self
                .contracting_edges
                .iter()
                .map(|e| self.ghost_edges.get(e).unwrap_or(e).id().into())
                .collect::<Vec<Edge>>()
                .contains(&(v, u).into())
            {
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
        // Natural lengths
        let l_d = self.edge_length * 2.0;
        let l_a = l_d / 9.0;
        let l_n = l_a * 2.0;

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
            .0
            .iter()
            .map(|v| self.positions[v])
            .collect()
    }

    #[allow(dead_code)]
    pub fn face_normal(&self, face_index: usize) -> V3f {
        self.faces[face_index]
            .0
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
        (0..self.faces.len())
            .into_iter()
            .fold(Vec::new(), |acc, i| [acc, self.face_xyz_buffer(i)].concat())
    }

    pub fn static_buffer(&self) -> Vec<V3f> {
        let mut rgb = Vec::new();
        let mut bsc = Vec::new();
        let mut tri = Vec::new();

        for face_index in 0..self.faces.len() {
            //let (face_xyz, face_tri) = self.face_xyz_buffer(face_index);
            let face_tri = self.face_tri_buffer(face_index);
            let color = HSL::new(
                (360.0 / (self.faces.len() as f64)) * face_index as f64,
                360.0 / 1.0,
                0.5,
            )
            .to_rgb_float();
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

        let mut buffer = Vec::new();
        for i in 0..rgb.len() {
            buffer.extend(vec![rgb[i], bsc[i], tri[i]]);
        }

        buffer
    }

    pub fn animate_contraction(&mut self) {
        // If all edges are contracted visually
        if !self.contracting_edges.is_empty()
            && self.contracting_edges.iter().fold(true, |acc, e| {
                let (v, u) = self.ghost_edges.get(e).unwrap_or(e).id();
                if self.positions.contains_key(&v) && self.positions.contains_key(&u) {
                    acc && self.positions[&v].distance(self.positions[&u]) < 0.08
                } else {
                    acc
                }
            })
        {
            // Contract them in the graph
            for e in self.contracting_edges.clone().into_iter() {
                self.contract_edge(e);
            }
            self.ghost_edges = HashMap::new();
            self.contracting_edges = HashSet::new();
            self.recompute_qualities();
            self.name.truncate(self.name.len() - 1);
            self.name += "a";
        }
    }
}
