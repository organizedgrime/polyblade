use super::*;
use crate::prelude::{WindowScene, HSL};
use serde::{Deserialize, Serialize};
use std::{collections::HashSet, ops::Add};
use three_d::*;

// Representation of an undirected graph
// Uses adjacency lists
#[derive(Debug, Serialize, Deserialize)]
pub struct Polyhedron {
    // Conway Polyhedron Notation
    pub name: String,

    // Points
    pub points: Vec<Point>,

    // List of faces
    pub faces: Vec<Face>,

    // Secret list of vertices that need to avoid each other
    pub(crate) enemies: HashSet<Edge>,
    pub(crate) edge_length: f32,
}

// Operations
impl Polyhedron {
    pub fn new(name: &str, points: Vec<Vec<usize>>, _faces: Vec<Vec<usize>>) -> Polyhedron {
        let mut poly = Polyhedron {
            name: String::from(name),
            points: points
                .into_iter()
                .enumerate()
                .map(|(id, neighbors)| Point::new(id, neighbors.into_iter().collect()))
                .collect(),
            faces: vec![],
            enemies: HashSet::new(),
            edge_length: 1.0,
        };
        poly.recompute_faces();
        poly
    }

    fn adjacents(&self) -> HashSet<Edge> {
        self.all_edges()
    }

    fn neighbors(&self) -> HashSet<Edge> {
        let adjacents = self.adjacents();

        // Track all neighbors
        let mut neighbors = HashSet::new();

        // For each point
        for v1 in 0..self.points.len() {
            // Grab its adjacents
            for v2 in self.points[v1].adjacents.clone() {
                for v3 in self.points[v1].adjacents.clone() {
                    let e = (v2, v3).into();
                    if v2 != v3 && !adjacents.contains(&e) {
                        neighbors.insert(e);
                    }
                }
            }
        }

        neighbors
    }

    fn diameter(&self) -> HashSet<Edge> {
        let mut non_diameter = self.all_edges(); //HashSet::new();
        let mut diameter = HashSet::new();

        let mut modi = true;
        while modi {
            modi = false;
            for edge in non_diameter.clone().iter() {
                let id = edge.id();
                for j in self.points[id.1].adjacents.clone() {
                    if id.0 != j {
                        let l = non_diameter.len();
                        non_diameter.insert((id.0, j).into());
                        diameter.insert((id.0, j).into());

                        if non_diameter.len() > l {
                            modi = true;
                        }
                    }
                }
            }

            if modi {
                diameter = HashSet::new();
            }
        }

        &non_diameter - &diameter
    }

    fn apply_forces(&mut self, edges: HashSet<Edge>, l: f32, k: f32) {
        for (i1, i2) in edges.into_iter().map(|e| e.id()) {
            let v1 = &self.points[i1].xyz;
            let v2 = &self.points[i2].xyz;

            let d = v1 - v2;
            let dist = d.magnitude();
            let distention = l - dist;
            let restorative_force = k / 2.0 * distention;
            let time_factor = 1000.0;
            let f = d * restorative_force / time_factor;
            self.points[i1].add_force(f);
            self.points[i2].add_force(-f);
            self.points[i1].update();
            self.points[i2].update();
        }
    }

    fn apply_spring_forces(&mut self) {
        // a = adjacent (length of an edge in 3D space)
        // n = neighbor (length of a path between two vertices is 2)
        // d = diameter (circumsphere / face of projection)

        // Natural lengths
        let l_d = self.edge_length * 2.0;
        let l_a = l_d / 5.0;

        //let l_a = 0.7 / 1.5; //self.edge_length;
        let l_n = l_a * 2.0;
        //let l_d = l_a * 5.0;

        // Spring constants
        let k_a = 0.9;
        let k_n = 0.4;
        let k_d = 0.3;

        self.apply_forces(self.adjacents(), l_a, k_a);
        self.apply_forces(self.neighbors(), l_n, k_n);
        let d = self.diameter();
        println!("d: {:?}", d);
        self.apply_forces(d, l_d, k_d);
        //self.apply_forces(self.enemies.clone(), l_d * 1.5, k_d / 2.0);
    }

    fn center(&mut self) {
        let shift = self
            .points
            .iter()
            .map(|p| p.xyz)
            .fold(Vector3::zero(), Vector3::add)
            / self.points.len() as f32;
        for i in 0..self.points.len() {
            self.points[i].xyz -= shift;
        }
    }

    fn resize(&mut self) {
        let mean_magnitude = self
            .points
            .iter()
            .map(|p| p.xyz.magnitude())
            .fold(0.0, f32::max);
        let distance = mean_magnitude - 1.0;
        //println!("distance");

        self.edge_length -= distance / 500.0;
    }

    fn quarrel(&mut self) {
        let threshold = 0.001;
        for v1 in 0..self.points.len() {
            for v2 in 0..self.points.len() {
                if self.points[v1].xyz.distance(self.points[v2].xyz).abs() < threshold {
                    self.enemies.insert((v1, v2).into());
                }
            }
        }
    }

    pub fn update(&mut self) {
        self.center();
        self.resize();
        self.quarrel();
        self.apply_spring_forces()
    }
}

impl Polyhedron {
    fn face_xyz(&self, face_index: usize) -> Vec<Vector3<f32>> {
        self.faces()[face_index]
            .0
            .iter()
            .map(|f| self.points[*f].xyz)
            .collect()
    }

    fn face_normal(&self, face_index: usize) -> Vector3<f32> {
        let face = &self.faces()[face_index].0;
        let mut normal = Vector3::<f32>::new(0.0, 0.0, 0.0);
        for i in 0..face.len() {
            let v1 = self.points[face[i]].xyz;
            let v2 = self.points[face[(i + 1) % face.len()]].xyz;
            normal += v1.cross(v2);
        }
        normal.normalize()
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

        let faces = self.faces();
        for face_index in 0..faces.len() {
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

            let color = HSL::new((360.0 / (faces.len() as f64)) * face_index as f64, 1.0, 0.5)
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
        /*
        println!(
            "distances: {:?}",
            self.points
                .iter()
                .map(|p| p.xyz.magnitude())
                .collect::<Vec<_>>()
        );
        */
        let r = self.face_normal(0) * 1.05;
        let theta = 2.0 * (1.0 / (2.0_f32.sqrt() * r.magnitude() - 1.0)).atan();
        scene
            .camera
            .set_view(r, vec3(0.0, 0.0, 0.0), vec3(0.0, 1.0, 0.0));

        scene
            .camera
            .set_perspective_projection(radians(theta), 0.01, 10.0);

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
        let (positions, colors, barycentric) = self.triangle_buffers(&scene.context);
        //let program = scene.program.unwrap();

        let time = frame_input.accumulated_time as f32;
        let model =
            Mat4::from_angle_y(radians(0.001 * time)) * Mat4::from_angle_x(radians(0.0004 * time));

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
