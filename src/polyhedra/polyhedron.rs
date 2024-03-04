use std::{
    collections::HashSet, iter::Chain, ops::{Add, Mul}
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

    // List of faces
    pub faces: Vec<Vec<usize>>,

    // Vertices in adjacency list
    pub vertices: Vec<Vec<usize>>,

    // XYZ Coordinates of Vertices
    pub xyz: Vec<Vector3<f32>>,
}

// Platonic Solids
impl Polyhedron {
    /*
    pub fn tetrahedron() -> Polyhedron {
        serde_json::from_slice(TETRAHEDRON_DATA).unwrap()
    }
    */

    pub fn cube() -> Polyhedron {
        let mut cube: Polyhedron = serde_json::from_slice(CUBE_DATA).unwrap();
        cube.xyz = vec![vec3(0.0, 0.0, 0.0); cube.xyz.len()];
        cube
    }

    /*
    pub fn cube() -> Polyhedron {
        Polyhedron {
            name: String::from("C"),
            faces: vec![
                vec![0, 1, 6, 7],
                vec![1, 3, 4, 6],
                vec![3, 2, 5, 4],
                vec![2, 0, 7, 5],
                vec![7, 6, 4, 5],
                vec![2, 3, 1, 0],
            ],
            vertices: vec![
                vec![1, 2, 7],
                vec![0, 3, 6],
                vec![0, 3, 5],
                vec![1, 2, 4],
                vec![3, 5, 6],
                vec![2, 4, 7],
                vec![1, 4, 7],
                vec![0, 5, 6],
            ],
        }
    }
    */

    /*
    pub fn octahedron() -> Polyhedron {
        serde_json::from_slice(OCTAHEDRON_DATA).unwrap()
    }

    pub fn dodecahedron() -> Polyhedron {
        serde_json::from_slice(DODECAHEDRON_DATA).unwrap()
    }

    pub fn icosahedron() -> Polyhedron {
        serde_json::from_slice(ICOSAHEDRON_DATA).unwrap()
    }
    */
}

// Operations
impl Polyhedron {

    pub fn edges(&self) -> HashSet<(usize, usize)> {
        let mut edges = HashSet::new();
        // For every 
        for (v1, neighbors) in self.vertices.iter().enumerate() {
            for v2 in neighbors.into_iter() {
                edges.insert((v1, *v2));
            }
        }
        edges
    }


    pub fn apply_spring_forces(&self) {
        // a = adjacent (length of an edge in 3D space)
        // n = neighbor (length of a path between two vertices is 2)
        // d = diameter (circumsphere / face of projection)

        // Fix the vertices associates with each category
        //let vertices_a =
        let vertices_d = self.faces[0];
        let projection_face = self.faces[0];


        // Natural lengths
        let l_a = 1.0;
        let l_n = 1.5;
        let l_d = 2.0;

        // Spring constants
        let k_a = 0.2;
        let k_n = 0.5;
        let k_d = 0.7;

        
        // Compute elastic forces
        // Ea should act on every edge
        // En should act between every vertex and all vertices which exist two away from it
        // Ed should act only on diameter edges
        //
        //
        let edges = self.edges();
        // 
        for edge in edges.iter() {

        }

        let e_a = k_a / 2.0 * sum ( (l_a - abs(v_i - v_j)^2) );

        // For each vertex in the face of projection
        let Ed = 0.0;
        for i in 0..=projection_face.len() {
            let edge = (projection_face[i], projection_face[(i+1) % projection_face.len()]);
            Ed +=
        }
        let Ed = Kd / 2.0 * self.faces[projection_face].iter().fold(0, |acc, vertex| {
            acc + ()
        });




        // Natural lengths of virtual springs
        let La = 0.5;

        let Ea = Ka / 2.0 *
    }
}

impl Polyhedron {
    fn face_xyz(&self, face_index: usize) -> Vec<Vector3<f32>> {
        self.faces[face_index]
            .iter()
            .map(|f| self.xyz[*f].clone())
            .collect()
    }

    fn face_normal(&self, face_index: usize) -> Vector3<f32> {
        let face = &self.faces[face_index];
        let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..face.len() {
            let v1 = self.xyz[face[i]];
            let v2 = self.xyz[face[(i + 1) % face.len()]];
            normal = normal.add(v1.cross(v2));
        }
        normal.normalize()
    }

    fn face_centroid(&self, face_index: usize) -> Vector3<f32> {
        // All vertices associated with this face
        let vertices: Vec<_> = self.face_xyz(face_index);
        let n = vertices.len() as f32;

        // Find the center of the polygon

        let mut center = vec3(0.0, 0.0, 0.0);
        for v in vertices.into_iter() {
            center += v;
        }

        center / n
    }

    pub fn triangle_buffers(
        &self,
        context: &Context,
    ) -> (VertexBuffer, VertexBuffer, VertexBuffer, VertexBuffer) {
        let mut polyhedron_xyz = Vec::new();
        let mut polyhedron_colors = Vec::new();
        let mut polyhedron_barycentric = Vec::new();
        let mut polyhedron_edges = Vec::new();

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
                    vec3(1.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
                ]);
                polyhedron_edges.extend(vec![vec3(0.0, 1.0, 0.0); 3])
            }

            let mut color = HSL::new(
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
        let edges = VertexBuffer::new_with_data(context, &polyhedron_edges);

        (positions, colors, barycentric, edges)
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
        self.apply_spring_forces();
        scene.camera.set_view(
            self.face_normal(0) * 0.75,
            vec3(0.0, 0.0, 0.0),
            vec3(0.0, 1.0, 0.0),
        );

        let (positions, colors, barycentric, edges) = self.triangle_buffers(&scene.context);
        let model = Mat4::from_angle_x(radians(0.0));

        scene.program.use_uniform("model", model);
        scene.program.use_uniform(
            "projection",
            scene.camera.projection() * scene.camera.view(),
        );
        scene.program.use_vertex_attribute("position", &positions);
        scene.program.use_vertex_attribute("color", &colors);
        scene
            .program
            .use_vertex_attribute("barycentric", &barycentric);
        //scene.program.use_vertex_attribute("edge", &edges);
        scene.program.draw_arrays(
            RenderStates::default(),
            frame_input.viewport,
            positions.vertex_count(),
        );
    }
    pub fn render_model(&self, scene: &mut WindowScene, frame_input: &FrameInput) {
        self.apply_spring_forces();
        let (positions, colors, barycentric, edges) = self.triangle_buffers(&scene.context);
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
        scene
            .program
            .use_vertex_attribute("barycentric", &barycentric);
        //scene.program.use_vertex_attribute("edge", &edges);
        scene.program.draw_arrays(
            RenderStates::default(),
            frame_input.viewport,
            positions.vertex_count(),
        );
    }
}
