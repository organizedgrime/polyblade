use std::{collections::HashSet, ops::Add};

use serde::{Deserialize, Serialize};
use three_d::*;

use crate::prelude::{WindowScene, HSL};

use super::Point;

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

// Platonic Solids
impl Polyhedron {
    pub fn cube() -> Polyhedron {
        Polyhedron {
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
        }
    }
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
        for (i1, i2) in edges.into_iter() {
            let v1 = &self.points[i1].pos();
            let v2 = &self.points[i2].pos();

            let d = v1.xyz() - v2.xyz();
            let dist = d.magnitude();
            let distention = l - dist;
            let restorative_force = k / 2.0 * distention;
            let f = d * restorative_force / 1000.0;

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

        // Natural lengths
        let l_a = 0.7;
        let l_n = l_a * 2.0;
        let l_d = l_a * 4.0;

        // Spring constants
        let k_a = 0.9;
        let k_n = 0.4;
        let k_d = 0.3;

        self.apply_forces(self.adjacents(), l_a, k_a);
        //self.apply_forces(edges.into_iter().filter(|b| b.0 == 0).collect(), l_a, k_a);
        self.apply_forces(self.neighbors(), l_n, k_n);
        self.apply_forces(self.foreigners(), l_d, k_d);
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
                    vec3(1.0, 0.0, 0.0),
                    vec3(0.0, 1.0, 0.0),
                    vec3(0.0, 0.0, 1.0),
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
}

impl Polyhedron {
    pub fn render_schlegel(&mut self, scene: &mut WindowScene, frame_input: &FrameInput) {
        self.apply_spring_forces();
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
        scene
            .program
            .use_vertex_attribute("barycentric", &barycentric);
        scene.program.draw_arrays(
            RenderStates::default(),
            frame_input.viewport,
            positions.vertex_count(),
        );
    }
    pub fn render_model(&mut self, scene: &mut WindowScene, frame_input: &FrameInput) {
        self.apply_spring_forces();
        let (positions, colors, barycentric) = self.triangle_buffers(&scene.context);
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
        scene.program.draw_arrays(
            RenderStates::default(),
            frame_input.viewport,
            positions.vertex_count(),
        );
    }
}
