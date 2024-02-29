use std::{
    iter::Chain,
    ops::{Add, Mul},
};

use rand::random;
use serde::{Deserialize, Serialize};
use three_d::*;

use crate::prelude::{Renderable, WindowScene, HSL};

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
    pub faces: Vec<Vec<usize>>,

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
        let mut faces = Vec::<Vec<usize>>::new();
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
        faces.push((0..=(n - 1)).map(|v| v as usize).rev().collect());
        // Bottom face
        faces.push((n..=2 * n - 1).map(|v| v as usize).collect());
        // n square faces
        for i in 0..n {
            faces.push(
                vec![i, (i + 1) % n, (i + 1) % n + n, i + n]
                    .into_iter()
                    .map(|v| v as usize)
                    .collect(),
            );
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
    fn face_vertices(&self, face_index: usize) -> Vec<Vector3<f32>> {
        self.faces[face_index]
            .iter()
            .map(|f| self.vertices[*f].clone())
            .collect()
    }

    fn face_normal(&self, face_index: usize) -> Vector3<f32> {
        let face = &self.faces[face_index];
        let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..face.len() {
            let v1 = self.vertices[face[i]];
            let v2 = self.vertices[face[(i + 1) % face.len()]];
            normal = normal.add(v1.cross(v2));
        }
        normal.normalize()
    }

    fn face_centroid(&self, face_index: usize) -> Vector3<f32> {
        // All vertices associated with this face
        let vertices: Vec<_> = self.face_vertices(face_index);

        // Find the center of the polygon
        let mut center = vertices[0];
        for v in vertices[1..].iter() {
            center = center.lerp(*v, 0.5);
        }

        center
    }

    pub fn triangle_buffers(&self, context: &Context) -> (VertexBuffer, VertexBuffer, VertexBuffer) {
        let mut polyhedron_vertices = Vec::new();
        let mut polyhedron_colors = Vec::new();
        let mut polyhedron_barycentric = Vec::new();
        for face_index in 0..self.faces.len() {
            // Create triangles from the center to each corner
            let mut face_vertices = Vec::new();
            let vertices = self.face_vertices(face_index);
            let center = self.face_centroid(face_index);

            // Construct a triangle
            for i in 0..vertices.len() {
                face_vertices.extend(vec![
                    vertices[i],
                    center,
                    vertices[(i + 1) % vertices.len()],
                ]);
                polyhedron_barycentric.extend(
                    vec![
                        vec3(1.0, 0.0, 0.0), 
                        vec3(0.0, 1.0, 0.0), 
                        vec3(0.0, 0.0, 1.0)
                    ])
            }

            let mut color = HSL::new(
                (360.0 / (self.faces.len() as f64)) * face_index as f64,
                1.0,
                0.5,
            )
            .to_linear_srgb();
            polyhedron_colors.extend(vec![color; face_vertices.len()]);
            polyhedron_vertices.extend(face_vertices);
        }
        
        let positions = VertexBuffer::new_with_data(context, &polyhedron_vertices);
        let colors = VertexBuffer::new_with_data(context, &polyhedron_colors);
        let barycentric = VertexBuffer::new_with_data(context, &polyhedron_barycentric);
        (positions, colors, barycentric)
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

impl Polyhedron {
    pub fn render_schlegel(&self, scene: &mut WindowScene, frame_input: &FrameInput) {
        scene.camera.set_view(
            self.face_normal(0) * 0.75,
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        let (positions, colors, barycentric) = self.triangle_buffers(&scene.context);
        let model = Mat4::from_angle_x(radians(0.0));

        scene.program.use_uniform("model", model);
        scene.program.use_uniform(
            "projection",
            scene.camera.projection() * scene.camera.view(),
        );
        scene.program.use_vertex_attribute("position", &positions);
        scene.program.use_vertex_attribute("color", &colors);
        scene.program.use_vertex_attribute("barycentric", &barycentric);
        scene.program.draw_arrays(
            RenderStates::default(),
            frame_input.viewport,
            positions.vertex_count(),
        );
    }
    pub fn render_model(&self, scene: &mut WindowScene, frame_input: &FrameInput) {
        let (positions, colors, barycentric) = self.triangle_buffers(&scene.context);
        //let program = scene.program.unwrap();

        let time = frame_input.accumulated_time as f32;
        let model =
            Mat4::from_angle_y(radians(0.001 * time)) * Mat4::from_angle_x(radians(0.001 * time));

        scene.program.use_uniform("model", model);
        scene.program.use_uniform(
            "projection",
            scene.camera.projection() * scene.camera.view(),
        );
        scene.program.use_vertex_attribute("position", &positions);
        scene.program.use_vertex_attribute("color", &colors);
        scene.program.use_vertex_attribute("barycentric", &barycentric);
        scene.program.draw_arrays(
            RenderStates::default(),
            frame_input.viewport,
            positions.vertex_count(),
        );
    }
}
