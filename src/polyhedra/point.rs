use cgmath::{vec3, InnerSpace, Vector3};
use rand::random;
use serde::{Deserialize, Serialize};

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
    }

    pub fn update(&mut self) {
        // Damping
        self.dxyz *= 0.92;
        self.xyz += self.dxyz;
    }

    pub fn dxyz(&self) -> Vector3<f32> {
        self.dxyz
    }
}
