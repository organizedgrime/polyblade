use std::collections::HashSet;

use cgmath::{vec3, InnerSpace, Vector3, Zero};
use rand::random;
use serde::{Deserialize, Serialize};

use super::{Vertex, VertexId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub id: VertexId,
    // Position
    pub xyz: Vector3<f32>,
    // Speed
    dxyz: Vector3<f32>,
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl PartialOrd for Point {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.id.partial_cmp(&other.id)
    }
}

impl Vertex for Point {
    fn id(&self) -> VertexId {
        self.id
    }
}

impl Point {
    pub fn new_empty(id: usize) -> Self {
        Self {
            id,
            xyz: Vector3::zero(),
            dxyz: Vector3::zero(),
        }
    }

    pub fn new(id: usize) -> Self {
        Self {
            id,
            xyz: vec3(random(), random(), random()).normalize(),
            dxyz: Vector3::zero(),
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
