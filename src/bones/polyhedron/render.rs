use rand::random;
use ultraviolet::Vec3;

use super::{VertexId, SPEED_DAMPENING, TICK_SPEED};

#[derive(Debug, Clone)]
pub struct Render {
    /// Positions in 3D space
    pub positions: Vec<Vec3>,
    /// Speeds in 3D space
    pub speeds: Vec<Vec3>,
    /// Edge length
    pub edge_length: f32,
}

//impl rand::
pub fn random_positions(n: usize) -> Vec<Vec3> {
    vec![Vec3::new(random(), random(), random()).normalized(); n]
}

impl Render {
    pub fn new(n: usize) -> Self {
        Self {
            positions: random_positions(n),
            speeds: vec![Vec3::zero(); n],
            edge_length: 1.0,
        }
    }

    pub fn update(&mut self, second: f32) {
        self.center();
        self.resize(second);
    }

    fn center(&mut self) {
        let shift =
            self.positions.iter().fold(Vec3::zero(), |a, &b| a + b) / self.positions.len() as f32;
        log::debug!("shifting all positions by {shift:?}");

        // for p in self.positions.iter_mut() {
        //     *p -= shift;
        // }
    }

    fn resize(&mut self, second: f32) {
        let mean_length = self.positions.iter().map(Vec3::mag).fold(0.0, f32::max);
        let matrixance = mean_length - 1.0;
        self.edge_length -= matrixance / TICK_SPEED * second;
    }

    pub fn new_capacity(&mut self, n: usize) {
        if n > self.positions.len() {
            let difference = n - self.positions.len();
            self.positions.extend(random_positions(difference));
            self.speeds.extend(vec![Vec3::zero(); difference]);
        }
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
        self.speeds[v] = (self.speeds[v] + f) * SPEED_DAMPENING;
        self.speeds[u] = (self.speeds[u] - f) * SPEED_DAMPENING;
        self.positions[v] += self.speeds[v];
        self.positions[u] += self.speeds[u];
    }
}
