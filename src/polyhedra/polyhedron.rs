use std::{
    collections::{HashMap, HashSet},
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

const DAMPING: f32 = 0.96;

#[derive(Debug, Serialize, Deserialize)]
pub struct Point {
    // List of point adjacents by index
    pub adjacents: Vec<usize>,
    // Position
    pub xyz: Vector3<f32>,
    // Speed
    dxyz: Vector3<f32>,
}

impl Point {
    pub fn new(neighbors: Vec<usize>) -> Self {
        Self {
            adjacents: neighbors,
            xyz: vec3(random(), random(), random()).normalize(),
            dxyz: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn add_force(&mut self, force: Vector3<f32>) {
        self.dxyz += force;
        //self.dxyz = self.dxyz.normalize();
    }

    pub fn update(&mut self) {
        self.dxyz *= DAMPING;
        self.xyz += self.dxyz;
        //self.dxyz = vec3(0.0, 0.0, 0.0);
    }

    pub fn pos(&self) -> Vector3<f32> {
        self.xyz
    }

    pub fn dxyz(&self) -> Vector3<f32> {
        self.dxyz
    }
}

// Representation of an undirected graph
// Uses adjacency lists
#[derive(Debug, Serialize, Deserialize)]
pub struct Polyhedron {
    // Conway Polyhedron Notation
    pub name: String,

    // Points
    pub points: Vec<Point>,

    // List of faces
    pub faces: Vec<Vec<usize>>,
}

impl Polyhedron {
    /*
    pub fn adjacents(&self, v: usize) -> Vec<usize> {
        self.points[v].adjacents.clone()
    }
    */
}

// Platonic Solids
impl Polyhedron {
    /*
    pub fn tetrahedron() -> Polyhedron {
        serde_json::from_slice(TETRAHEDRON_DATA).unwrap()
    }
    */

    /*
    pub fn cube() -> Polyhedron {
        let mut cube: Polyhedron = serde_json::from_slice(CUBE_DATA).unwrap();
        cube.xyz = vec![vec3(0.0, 0.0, 0.0); cube.xyz.len()];
        cube
    }
    */

    pub fn cube() -> Polyhedron {
        let mut c = Polyhedron {
            name: String::from("C"),
            points: vec![
                Point::new(vec![1, 2, 7]),
                Point::new(vec![0, 3, 6]),
                Point::new(vec![0, 3, 5]),
                Point::new(vec![1, 2, 4]),
                Point::new(vec![3, 5, 6]),
                Point::new(vec![2, 4, 7]),
                Point::new(vec![1, 4, 7]),
                Point::new(vec![0, 5, 6]),
            ],
            faces: vec![
                vec![3, 0, 1, 2],
                vec![3, 4, 5, 0],
                vec![0, 5, 6, 1],
                vec![1, 6, 7, 2],
                vec![2, 7, 4, 3],
                vec![5, 4, 7, 6],
            ],
        };

        let xyz = vec![
            vec3(0.407, 0.407, 0.407),
            vec3(-0.707, 0.707, 0.707),
            vec3(-0.707, -0.707, 0.707),
            vec3(0.707, -0.707, 0.707),
            vec3(0.707, -0.707, -0.707),
            vec3(0.707, 0.707, -0.707),
            vec3(-0.707, 0.707, -0.707),
            vec3(-0.707, -0.707, -0.707),
        ];

        for i in 0..c.points.len() {
            c.points[i].xyz = xyz[i]; //* 4.0;
        }

        c
    }

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
    pub fn adjacents(&self) -> HashSet<(usize, usize)> {
        let mut edges = HashSet::new();
        // For every
        for (v1, point) in self.points.iter().enumerate() {
            for v2 in point.adjacents.clone().into_iter() {
                if v1 <= v2 {
                    edges.insert((v1, v2));
                } else {
                    edges.insert((v2, v1));
                }
            }
        }
        edges
    }

    pub fn neighbors(&self) -> HashSet<(usize, usize)> {
        // Track all neighbors
        let mut neighbors = HashSet::new();
        // For each point
        for v1 in 0..self.points.len() {
            // Grab its adjacents
            let my_adjacents = self.points[v1].adjacents.clone();
            for v2 in my_adjacents.into_iter() {
                for v3 in self.points[v2].adjacents.clone() {
                    if v1 != v3 {
                        if v1 < v3 {
                            neighbors.insert((v1, v3));
                        } else {
                            neighbors.insert((v3, v1));
                        }
                    }
                }
            }
        }
        neighbors
    }

    pub fn foreigners(&self) -> HashSet<(usize, usize)> {
        // Track all neighbors
        let mut foreigners = HashSet::new();
        let mut known = self.adjacents();
        known.extend(self.neighbors());
        // For each point
        for v1 in 0..self.points.len() {
            for v2 in 0..self.points.len() {
                let pair = if v1 < v2 { (v1, v2) } else { (v2, v1) };
                if v1 != v2 && known.get(&pair).is_none() {
                    foreigners.insert(pair);
                }
            }
        }
        foreigners
    }

    pub fn apply_forces(&mut self, edges: HashSet<(usize, usize)>, l: f32, k: f32) {
        println!("applying forces to : {:?}", edges);
        for (i1, i2) in edges.into_iter() {
            let v1 = &self.points[i1].pos();
            let v2 = &self.points[i2].pos();

            let d = v1.xyz() - v2.xyz();
            let dist = d.magnitude();
            let distention = l - dist;
            let restorative_force = k / 2.0 * distention;
            let f = d * restorative_force / 1000.0;

            // If positive, we want to get further from the origin
            self.points[i1].add_force(f);
            self.points[i2].add_force(-f);

            self.points[i1].update();
            self.points[i2].update();
        }
    }

    pub fn apply_spring_forces(&mut self) {
        // a = adjacent (length of an edge in 3D space)
        // n = neighbor (length of a path between two vertices is 2)
        // d = diameter (circumsphere / face of projection)

        // Fix the vertices associates with each category
        //let vertices_a =
        let vertices_d = &self.faces[0];
        let projection_face = &self.faces[0];

        // Natural lengths
        let l_a = 0.7;
        let l_n = 2.0_f32.sqrt() * l_a;
        let l_d = 3.0_f32.sqrt() * l_a;

        // Spring constants
        let k_a = 0.7;
        let k_n = 0.7;
        let k_d = 0.8;

        // Compute elastic forces
        // Ea should act on every edge
        // En should act between every vertex and all vertices which exist two away from it
        // Ed should act only on diameter edges
        //
        //
        //

        let edges = self.adjacents();
        //println!("adjacents: {:?}", edges);
        self.apply_forces(edges, l_a, k_a);
        //self.apply_forces(edges.into_iter().filter(|b| b.0 == 0).collect(), l_a, k_a);

        let edges = self.neighbors();
        //println!("neighbors: {:?}", edges);
        self.apply_forces(edges, l_n, k_n);

        // diagonalz
        let edges = self.foreigners();
        //println!("foreigners: {:?}", edges);
        self.apply_forces(edges, l_d, k_d);

        /*
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
        */
    }
}

impl Polyhedron {
    fn face_xyz(&self, face_index: usize) -> Vec<Vector3<f32>> {
        self.faces[face_index]
            .iter()
            .map(|f| self.points[*f].pos())
            .collect()
    }

    fn face_normal(&self, face_index: usize) -> Vector3<f32> {
        let face = &self.faces[face_index];
        let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..face.len() {
            let v1 = self.points[face[i]].pos();
            let v2 = self.points[face[(i + 1) % face.len()]].pos();
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
    pub fn render_schlegel(&mut self, scene: &mut WindowScene, frame_input: &FrameInput) {
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
    pub fn render_model(&mut self, scene: &mut WindowScene, frame_input: &FrameInput) {
        self.apply_spring_forces();
        let (positions, colors, barycentric, edges) = self.triangle_buffers(&scene.context);
        //let program = scene.program.unwrap();

        let time = frame_input.accumulated_time as f32;
        let model =
            Mat4::from_angle_y(radians(0.001 * time)) * Mat4::from_angle_x(radians(0.000 * time));

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
