use super::*;
use crate::prelude::HSL;
use std::{
    collections::{HashMap, HashSet},
    ops::Add,
};
use three_d::*;

const TICK_SPEED: f32 = 100.0;

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
            / self.vertex_count() as f32;

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
    fn face_normal(&self, face_index: usize) -> Vector3<f32> {
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

    pub fn triangle_buffers(
        &self,
        context: &Context,
    ) -> (VertexBuffer, VertexBuffer, VertexBuffer) {
        let mut polyhedron_xyz = Vec::new();
        let mut polyhedron_colors = Vec::new();
        let mut polyhedron_barycentric = Vec::new();

        for face_index in 0..self.faces.len() {
            // Create triangles from the center to each corner
            let mut face_xyz = Vec::new();
            let vertices = self.face_xyz(face_index);
            let center = self.face_centroid(face_index);

            // Construct a triangle
            for i in 0..vertices.len() {
                face_xyz.extend(vec![
                    vertices[i],
                    center,
                    vertices[(i + 1) % vertices.len()],
                ]);
                polyhedron_barycentric.extend(vec![
                    Vector3::<f32>::unit_x(),
                    Vector3::<f32>::unit_y(),
                    Vector3::<f32>::unit_z(),
                ]);
            }

            let color = HSL::new(
                (360.0 / (self.faces.len() as f64)) * face_index as f64,
                1.0,
                0.5,
            )
            .to_linear_srgb();
            polyhedron_colors.extend(vec![color; face_xyz.len()]);
            polyhedron_xyz.extend(face_xyz);
        }

        let positions = VertexBuffer::new_with_data(context, &polyhedron_xyz);
        let colors = VertexBuffer::new_with_data(context, &polyhedron_colors);
        let barycentric = VertexBuffer::new_with_data(context, &polyhedron_barycentric);

        (positions, colors, barycentric)
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
            self.recompute_qualities();
            self.ghost_edges = HashMap::new();
            self.contracting_edges = HashSet::new();
            self.name.truncate(self.name.len() - 1);
            self.name += "a";
        }
    }
}
