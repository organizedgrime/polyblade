use super::*;
use crate::prelude::{WindowScene, HSL};
use std::{collections::HashSet, ops::Add};
use three_d::*;

// Operations
impl Graph {
    fn apply_forces(&mut self, edges: HashSet<Edge>, l: f32, k: f32) {
        for (v, u) in edges.into_iter().map(|e| e.id()) {
            let diff = self.positions[&v] - self.positions[&u];
            let dist = diff.magnitude();
            let distention = l - dist;
            let restorative_force = k / 2.0 * distention;
            let time_factor = 1000.0;
            let f = diff * restorative_force / time_factor;

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

    fn apply_spring_forces(&mut self) {
        // Natural lengths
        let l_d = self.edge_length * 2.0;
        let l_a = l_d / 5.0;
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

        self.edge_length -= distance / 100.0;
    }

    pub fn update(&mut self) {
        self.center();
        self.resize();
        self.apply_spring_forces()
    }

    fn face_xyz(&self, face_index: usize) -> Vec<Vector3<f32>> {
        self.faces[face_index]
            .0
            .iter()
            .map(|v| self.positions[v])
            .collect()
    }

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
