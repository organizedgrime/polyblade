use std::{
    iter::Chain,
    ops::{Add, Mul},
};

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
    fn triangle_buffers(&self, context: &Context) -> (VertexBuffer, VertexBuffer) {
        let mut polyhedron_vertices = Vec::new();
        let mut polyhedron_colors = Vec::new();
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
            }

            let mut color = HSL::new(
                (360.0 / (self.faces.len() as f64)) * face_index as f64,
                1.0,
                0.5,
            )
            .to_linear_srgb();
            if face_index == 0 {
                color = Srgba::WHITE.to_linear_srgb();
            }
            polyhedron_colors.extend(vec![color; face_vertices.len()]);
            polyhedron_vertices.extend(face_vertices);
        }

        let positions = VertexBuffer::new_with_data(context, &polyhedron_vertices);
        let colors = VertexBuffer::new_with_data(context, &polyhedron_colors);
        (positions, colors)
    }

    pub fn render_schlegel(&self) {
        // Create a window (a canvas on web)
        let window = Window::new(WindowSettings {
            title: "Core Triangle!".to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            max_size: Some((1280, 720)),
            ..Default::default()
        })
        .unwrap();
        // Get the graphics context from the window
        let context: Context = window.gl();

        /*
        let program = Program::from_source(
            &context,
            include_str!("../shaders/basic.vert"),
            include_str!("../shaders/basic.frag"),
        )
        .unwrap();
        */

        let mut camera = Camera::new_perspective(
            window.viewport(),
            self.face_normal(0) * 1.01,
            vec3(0.0, 0.0, 0.0), // target
            vec3(0.0, 1.0, 0.0), // up
            degrees(45.0),
            0.01,
            5.0,
        );

        //let (positions, colors) = self.triangle_buffers(&context);

        let projection = camera.view(); //* camera.view();
        let mut lines = Vec::new();
        // For each
        for face in self.faces.iter() {
            for i in 0..face.len() {
                let p1 = self.vertices[face[i]];
                let p2 = self.vertices[face[(i + 1) % face.len()]];
                //let norm = self.face_normal(0) * 1.001; //* 2.0;
                //let r1 = norm.distance(p1);
                //let r2 = norm.distance(p2);

                let p1 = ((projection * vec4(p1.x, p1.y, p1.z, 1.0)) / 3.0).xy();
                let p2 = ((projection * vec4(p2.x, p2.y, p2.z, 1.0)) / 3.0).xy();

                let line = Line::new(&context, p1, p2, 0.009);

                lines.push(Gm::new(
                    line,
                    ColorMaterial {
                        color: Srgba::RED,
                        ..Default::default()
                    },
                ))
            }
        }

        /*
        let mut camera = Camera::new_perspective(
            window.viewport(),
            self.face_normal(0) * 1.2,
            vec3(0.0, 0.0, 0.0), // target
            vec3(0.0, 5.0, 0.0), // up
            degrees(45.0),
            0.01,
            10.0,
        );
        */

        window.render_loop(move |frame_input| {
            camera.set_viewport(frame_input.viewport);
            frame_input
                .screen()
                // Clear the color and depth of the screen render target
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .render(&camera, &lines, &[]);
            /*
            .write(|| {
                let time = frame_input.accumulated_time as f32;
                program.use_uniform("projection", camera.projection() * camera.view());
                program.use_vertex_attribute("position", &positions);
                program.use_vertex_attribute(
                    "barycentric",
                    &VertexBuffer::new_with_data(
                        &context,
                        &vec![vec2(0.0, 0.0); positions.vertex_count()],
                    ),
                );
                program.draw_arrays(
                    RenderStates::default(),
                    frame_input.viewport,
                    positions.vertex_count(),
                );
            });
            */
            FrameOutput::default()
        });
    }

    pub fn render_form(&self) {
        // Create a window (a canvas on web)
        let window = Window::new(WindowSettings {
            title: "Core Triangle!".to_string(),
            #[cfg(not(target_arch = "wasm32"))]
            max_size: Some((1280, 720)),
            ..Default::default()
        })
        .unwrap();
        // Get the graphics context from the window
        let context: Context = window.gl();

        let program = Program::from_source(
            &context,
            include_str!("../shaders/basic.vert"),
            include_str!("../shaders/basic.frag"),
        )
        .unwrap();
        //let mut position = self.face_centroid(0);
        let position = self.face_normal(0).mul(4.0);

        let mut camera = Camera::new_perspective(
            window.viewport(),
            position,
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 5.0, 0.0),
            degrees(45.0),
            0.1,
            10.0,
        );

        let (positions, colors) = self.triangle_buffers(&context);

        window.render_loop(move |frame_input| {
            camera.set_viewport(frame_input.viewport);
            frame_input
                .screen()
                // Clear the color and depth of the screen render target
                .clear(ClearState::color_and_depth(0.8, 0.8, 0.8, 1.0, 1.0))
                .write(|| {
                    let time = frame_input.accumulated_time as f32;
                    program.use_uniform("model", Mat4::from_angle_x(radians(0.001 * time)));
                    program.use_uniform("viewProjection", camera.projection() * camera.view());
                    program.use_vertex_attribute("position", &positions);
                    program.use_vertex_attribute("color", &colors);
                    program.draw_arrays(
                        RenderStates::default(),
                        frame_input.viewport,
                        positions.vertex_count(),
                    );
                });
            FrameOutput::default()
        });
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
