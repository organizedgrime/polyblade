use cgmath::{vec3, InnerSpace, Vector3, Zero};
use rand::random;
use serde::{Deserialize, Serialize};

use super::{Vertex, VertexId};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Point {
    pub id: usize,
    // List of point adjacents by index
    pub adjacents: Vec<usize>,
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
            adjacents: vec![],
            xyz: Vector3::zero(),
            dxyz: Vector3::zero(),
        }
    }
    pub fn connect(&mut self, id: usize) {
        if !self.adjacents.contains(&id) && id != self.id() {
            self.adjacents.push(id)
        }
    }
    pub fn disconnect(&mut self, id: usize) {
        self.adjacents = self
            .adjacents
            .clone()
            .into_iter()
            .filter(|v| v != &id)
            .collect();
    }
    pub fn delete(&mut self, id: usize) {
        self.disconnect(id);
        self.adjacents = self
            .adjacents
            .clone()
            .into_iter()
            .map(|i| if i > id { i - 1 } else { i })
            //.filter(|i| i.id() != self.id())
            .collect()
    }

    pub fn new(id: usize, neighbors: Vec<usize>) -> Self {
        Self {
            id,
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
