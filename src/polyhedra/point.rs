use cgmath::{vec3, InnerSpace, Vector3};
use rand::random;
use serde::{Deserialize, Serialize};

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
        let sign = if rand::random::<u8>() > 128 {
            1.0
        } else {
            -1.0
        };
        Self {
            adjacents: neighbors,
            xyz: vec3(random(), random(), random()).normalize() * sign * 3.0,
            dxyz: vec3(0.0, 0.0, 0.0),
        }
    }

    pub fn add_force(&mut self, force: Vector3<f32>) {
        self.dxyz += force;
    }

    pub fn update(&mut self) {
        self.dxyz *= DAMPING;
        self.xyz += self.dxyz;
    }

    pub fn pos(&self) -> Vector3<f32> {
        self.xyz
    }

    pub fn dxyz(&self) -> Vector3<f32> {
        self.dxyz
    }
}
