use crate::bones::*;
use rand::random;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use ultraviolet::Vec3;
type VertMap<T> = HashMap<VertexId, T>;
pub type VertexId = usize;

#[derive(Debug, Clone, Default)]
pub struct PolyGraph {
    /// Conway Polyhedron Notation
    pub name: String,
    /// Graph
    pub graph: JagGraph,
    /// Faces / chordless cycles
    pub springs: Vec<[VertexId; 2]>,
    /// Positions in 3D space
    pub positions: Vec<Vec3>,
    /// Speeds
    pub speeds: Vec<Vec3>,
    /// Transaction queue
    pub transactions: Vec<Transaction>,
    /// Edge length
    pub edge_length: f32,
}

impl PolyGraph {
    /// New with n vertices
    pub fn new_disconnected(n: usize) -> Self {
        Self {
            graph: JagGraph::new(n),
            speeds: vec![Vec3::zero(); n],
            edge_length: 1.0,
            positions: vec![Vec3::new(random(), random(), random()).normalized(); n],
            ..Default::default()
        }
    }

    // Use a Fibonacci Lattice to spread the points evenly around a sphere
    pub fn lattice(&mut self) {
        // Use a Fibonacci Lattice to evently distribute starting points on a sphere
        let phi = std::f32::consts::PI * (3.0 - 5.0f32.sqrt());
        for v in 0..self.graph.len() {
            let y = 1.0 - (v as f32 / (self.graph.len() - 1) as f32);
            let radius = (1.0 - y * y).sqrt();
            let theta = (phi * (v as f32)) % (std::f32::consts::PI * 2.0);
            let x = theta.cos() * radius;
            let z = theta.sin() * radius;
            self.positions.push(Vec3::new(x, y, z));
        }
    }

    pub fn connect(&mut self, [v, u]: [VertexId; 2]) {
        self.graph.connect([v, u]);
    }

    pub fn disconnect(&mut self, [v, u]: [VertexId; 2]) {
        self.graph.disconnect([v, u]);
    }

    pub fn insert(&mut self) -> VertexId {
        self.positions
            .push(Vec3::new(random(), random(), random()).normalized());
        self.speeds.push(Vec3::zero());
        self.graph.insert()
    }

    /*
    pub fn delete(&mut self, v: VertexId) {
        self.vertices.remove(&v);

        self.edges = self
            .edges
            .clone()
            .into_iter()
            .filter(|e| !e.contains(v))
            .collect();

        self.cycles = self
            .cycles
            .clone()
            .into_iter()
            .map(|face| face.into_iter().filter(|&u| u != v).collect())
            .collect();

        self.positions.remove(&v);
        self.speeds.remove(&v);
    }
    */

    /// Edges of a vertex
    // pub fn edges(&self, v: VertexId) -> Vec<Edge> {
    //     let mut edges = vec![];
    //     for u in 0..self.dist.len() {
    //         if self.dist[v][u] == 1 {
    //             edges.push((v, u).into());
    //         }
    //     }
    //     edges
    // }

    /// Number of faces
    // pub fn face_count(&self) -> i64 {
    //     2 + self.edges.len() as i64 - self.vertices.len() as i64
    // }

    //
    //
    //

    pub fn springs(&mut self) {
        let diameter = self.graph.diameter();
        self.springs = self
            .graph
            .vertex_pairs()
            .filter(|&[v, u]| {
                v != u && (self.graph[[v, u]] <= 2 || self.graph[[v, u]] >= diameter - 1)
            })
            .collect::<Vec<_>>();

        // log::debug!(
        //     "v_len: {} | v2: {} | springs: {}",
        //     self.vertices.len(),
        //     (self.vertices.len() as f32).powi(2),
        //     self.springs.len()
        // );
    }
}

/*
impl Display for PolyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vertices = self.vertices.iter().collect::<Vec<_>>();
        vertices.sort();
        let mut adjacents = self.edges.clone().into_iter().collect::<Vec<_>>();
        adjacents.sort();

        f.write_fmt(format_args!(
            "name:\t\t{}\nvertices:\t{:?}\nadjacents:\t{}\nfaces:\t\t{}\n",
            self.name,
            vertices,
            adjacents
                .iter()
                .fold(String::new(), |acc, e| format!("{e}, {acc}")),
            self.cycles.iter().fold(String::new(), |acc, f| format!(
                "[{}], {acc}",
                f.iter().fold(String::new(), |acc, x| format!("{x}, {acc}"))
            ))
        ))
    }
}
*/
