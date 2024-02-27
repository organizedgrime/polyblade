use std::iter::Chain;

use rand::random;
use serde::{Deserialize, Serialize};
use three_d::*;

use crate::prelude::HSL;

// Include the raw data in these platonic solid JSONs
const TETRAHEDRON_DATA: &[u8] = include_bytes!("../platonic_solids/tetrahedron.json");
const CUBE_DATA: &[u8] = include_bytes!("../platonic_solids/cube.json");
const OCTAHEDRON_DATA: &[u8] = include_bytes!("../platonic_solids/octahedron.json");
const DODECAHEDRON_DATA: &[u8] = include_bytes!("../platonic_solids/dodecahedron.json");
const ICOSAHEDRON_DATA: &[u8] = include_bytes!("../platonic_solids/icosahedron.json");

// Representation of an undirected graph
// Uses adjacency lists
#[derive(Debug, Serialize, Deserialize)]
pub struct Polyhedron {
    // Conway Polyhedron Notation
    pub name: String,

    // Faces
    pub faces: Vec<Vec<i32>>,

    // Vertices
    pub vertices: Vec<Vector3<f32>>,
}

// Platonic Solids
impl Polyhedron {
    pub fn tetrahedron() -> Polyhedron {
        serde_json::from_slice(TETRAHEDRON_DATA).unwrap()
    }

    pub fn cube() -> Polyhedron {
        serde_json::from_slice(CUBE_DATA).unwrap()
    }

    pub fn octahedron() -> Polyhedron {
        serde_json::from_slice(OCTAHEDRON_DATA).unwrap()
    }

    pub fn dodecahedron() -> Polyhedron {
        serde_json::from_slice(DODECAHEDRON_DATA).unwrap()
    }

    pub fn icosahedron() -> Polyhedron {
        serde_json::from_slice(ICOSAHEDRON_DATA).unwrap()
    }
}

impl Polyhedron {
    pub fn prism(n: i32) -> Self {
        // Starting vars
        let name = format!("P{}", n);
        let mut faces = Vec::new();
        let mut vertices = Vec::new();

        // Pie angle
        let theta = std::f32::consts::PI / (n as f32);
        // Half edge
        let h = (theta / 2.0).sin();

        for i in 0..n {
            let i = i as f32;
            vertices.push(vec3((i * theta).cos(), (i * theta).sin(), h));
        }
        for i in 0..n {
            let i = i as f32;
            vertices.push(vec3((i * theta).cos(), (i * theta).sin(), -h));
        }

        // Top face
        faces.push((0..=n - 1).rev().collect());
        // Bottom face
        faces.push((n..=2 * n - 1).collect());
        // n square faces
        for i in 0..n {
            faces.push(vec![i, (i + 1) % n, (i + 1) % n + n, i + n]);
        }

        // TODO adjust xyz

        Self {
            name,
            faces,
            vertices,
        }
    }
}

// Operations
impl Polyhedron {
    // k: kisN(self, n)
    // a: ambo(self)
    // g: gyro(self)
    // p: propellor(self)
    // d: dual(self)
    // r: reflect(self)
    // c: canonicalize xyz
}

impl Polyhedron {
    pub fn render_schlegel(&self, context: &Context) -> Vec<Gm<Line, ColorMaterial>> {
        let scale = 500.0;
        let mut lines = Vec::new();
        for face in self.faces.iter() {
            for i in 0..face.len() {
                let v1 = self.vertices[face[i] as usize];
                let v2 = self.vertices[face[(i + 1) % face.len()] as usize];

                let v1 = PhysicalPoint {
                    x: v1[0] * scale,
                    y: v1[1] * scale,
                };
                let v2 = PhysicalPoint {
                    x: v2[0] * scale,
                    y: v2[1] * scale,
                };
                let line = Line::new(&context, v1, v2, 50.0);
                lines.push(Gm::new(
                    line,
                    ColorMaterial {
                        color: Srgba::GREEN,
                        ..Default::default()
                    },
                ));
            }
        }

        lines
    }

    pub fn render_form(&self, program: &Program, context: &Context, viewport: Viewport) {
        let mut polyhedron_vertices = Vec::new();
        let mut polyhedron_colors = Vec::new();
        for (idx, face) in self.faces.iter().enumerate() {
            // All vertices associated with this face
            let vertices: Vec<_> = face
                .iter()
                .map(|f| self.vertices[*f as usize].clone())
                .collect();

            // Find the center of the polygon
            let mut center = vertices[0];
            for v in vertices[1..].iter() {
                center = center.lerp(*v, 0.5);
            }

            // Create triangles from the center to each corner
            let mut face_vertices = Vec::new();
            for i in 0..vertices.len() {
                face_vertices.extend(vec![
                    vertices[i],
                    center,
                    vertices[(i + 1) % vertices.len()],
                ]);
            }

            let color = HSL::new((360.0 / (self.faces.len() as f64)) * idx as f64, 1.0, 0.5)
                .to_linear_srgb();
            polyhedron_colors.extend(vec![color; face_vertices.len()]);
            polyhedron_vertices.extend(face_vertices);
        }

        let positions = VertexBuffer::new_with_data(context, &polyhedron_vertices);
        let colors = VertexBuffer::new_with_data(context, &polyhedron_colors);

        program.use_vertex_attribute("position", &positions);
        program.use_vertex_attribute("color", &colors);

        program.draw_arrays(RenderStates::default(), viewport, positions.vertex_count());
    }
}
/*
 *
 *
 * Euler's formula:
 * V - E + F = 2
 *
 *
 * How do we create pretty Schlegel diagrams from our
 * known edge sets like octahedrons, dodecahedrons, etc?
 * just make the vertices repel each other and let the physics solve it somehow.
 * can perform a greedy algorithm for initial layout:
 * start at a vertex, (or actually, a face, if we want it centered), and draw all the adjacent
 * vertices next, at an increased radius from the origin. do this again and again until all
 * vertices have been placed, then allow the physics simulation to act on them and bring them to a
 * state of rest. this should be a cute and simple way to solve for these diagrams no matter the
 * polyhedra we're solving for.
 */
