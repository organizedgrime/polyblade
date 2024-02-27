use serde::{Deserialize, Serialize};
use three_d::{vec3, Context, Vector3, VectorSpace, VertexBuffer};

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
    pub fn render(&self, context: &Context) -> VertexBuffer {
        let mut polyhedron_vertices = Vec::new();
        for face in self.faces.iter() {
            // All vertices associated with this face
            let vertices: Vec<_> = face
                .iter()
                .map(|f| self.vertices[*f as usize].clone())
                .collect();

            let mut center = vertices[0];
            for v in vertices[1..].iter() {
                center = center.lerp(*v, 0.5);
            }

            let mut face_vertices = Vec::new();
            for i in 0..vertices.len() {
                face_vertices.extend(vec![
                    vertices[i],
                    center,
                    vertices[(i + 1) % vertices.len()],
                ]);
            }

            polyhedron_vertices.extend(face_vertices);
        }
        VertexBuffer::new_with_data(context, &polyhedron_vertices)
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
