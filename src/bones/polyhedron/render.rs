use rand::random;
use ultraviolet::Vec3;

use super::{VertexId, SPEED_DAMPENING, TICK_SPEED};

#[derive(Debug, Clone)]
pub struct Vertex {
    pub position: Vec3,
    pub speed: Vec3,
}

impl Default for Vertex {
    fn default() -> Self {
        Self {
            position: Vec3::new(random(), random(), random()).normalized(),
            speed: Default::default(),
        }
    }
}

#[derive(Debug, Clone)]
pub struct Render {
    /// Positions in 3D space
    pub vertices: Vec<Vertex>,
    /// Edge length
    pub edge_length: f32,
}

impl Render {
    pub fn new(n: usize) -> Self {
        Self {
            vertices: vec![Vertex::default(); n],
            edge_length: 1.0,
        }
    }

    pub fn update(&mut self, second: f32) {
        self.center();
        self.resize(second);
    }

    fn center(&mut self) {
        let shift = self
            .vertices
            .iter()
            .fold(Vec3::zero(), |a, b| a + b.position)
            / self.vertices.len() as f32;

        for p in self.vertices.iter_mut() {
            (*p).position -= shift;
        }
    }

    fn resize(&mut self, second: f32) {
        let mean_length = self
            .vertices
            .iter()
            .map(|v| v.position.mag())
            .fold(0.0, f32::max);
        let matrixance = mean_length - 1.0;
        self.edge_length -= matrixance / TICK_SPEED * second;
    }

    // pub fn lattice(&mut self) {
    //     self.vertices = vec![];
    //     // Use a Fibonacci Lattice to evently distribute starting points on a sphere
    //     let phi = std::f32::consts::PI * (3.0 - 5.0f32.sqrt());
    //     for v in 0..self.graph.len() {
    //         let y = 1.0 - (v as f32 / (self.graph.len() - 1) as f32);
    //         let radius = (1.0 - y * y).sqrt();
    //         let theta = (phi * (v as f32)) % (std::f32::consts::PI * 2.0);
    //         let x = theta.cos() * radius;
    //         let z = theta.sin() * radius;
    //         self.positions.push(Vec3::new(x, y, z));
    //     }
    // }
    //
    pub fn apply_force(&mut self, [v, u]: [VertexId; 2], f: Vec3) {
        self.vertices[v].speed += f * SPEED_DAMPENING;
        self.vertices[u].speed -= f * SPEED_DAMPENING;
        let sv = self.vertices[v].speed;
        let su = self.vertices[u].speed;
        self.vertices[v].position += sv;
        self.vertices[u].position += su;
    }
}
