use rand::random;
use ultraviolet::Vec3;

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

#[derive(Debug, Clone, Default)]
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
}
