use crate::bones::*;
use rand::random;
use rustc_hash::{FxHashMap as HashMap, FxHashSet as HashSet};
use ultraviolet::Vec3;
type VertMap<T> = HashMap<VertexId, T>;
pub type VertexId = usize;

#[derive(Debug, Clone, Default)]
pub struct Polyhedron {
    /// Conway Polyhedron Notation
    pub name: String,
    /// The shape we're rendering
    pub shape: Shape,
    /// The properties
    pub render: Render,
    /// Transaction queue
    pub transactions: Vec<Transaction>,
}

impl Polyhedron {
    // Use a Fibonacci Lattice to spread the points evenly around a sphere
    // pub fn connect(&mut self, [v, u]: [VertexId; 2]) {
    //     self.graph.connect([v, u]);
    // }
    //
    // pub fn disconnect(&mut self, [v, u]: [VertexId; 2]) {
    //     self.graph.disconnect([v, u]);
    // }
    //
    // pub fn insert(&mut self) -> VertexId {
    //     self.positions
    //         .push(Vec3::new(random(), random(), random()).normalized());
    //     self.speeds.push(Vec3::zero());
    //     self.graph.insert()
    // }

    // pub fn delete(&mut self, v: VertexId) {
    //     self.vertices.remove(&v);
    //
    //     self.edges = self
    //         .edges
    //         .clone()
    //         .into_iter()
    //         .filter(|e| !e.contains(v))
    //         .collect();
    //
    //     self.cycles = self
    //         .cycles
    //         .clone()
    //         .into_iter()
    //         .map(|face| face.into_iter().filter(|&u| u != v).collect())
    //         .collect();
    //
    //     self.positions.remove(&v);
    //     self.speeds.remove(&v);
    // }
    //
    // /// Edges of a vertex
    // pub fn edges(&self, v: VertexId) -> Vec<Edge> {
    //     let mut edges = vec![];
    //     for u in 0..self.dist.len() {
    //         if self.dist[v][u] == 1 {
    //             edges.push((v, u).into());
    //         }
    //     }
    //     edges
    // }

    // /// Number of faces
    // pub fn face_count(&self) -> i64 {
    //     2 + self.edges.len() as i64 - self.vertices.len() as i64
    // }

    //
    //
    //
}

// impl Display for PolyGraph {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         let mut vertices = self.vertices.iter().collect::<Vec<_>>();
//         vertices.sort();
//         let mut adjacents = self.edges.clone().into_iter().collect::<Vec<_>>();
//         adjacents.sort();
//
//         f.write_fmt(format_args!(
//             "name:\t\t{}\nvertices:\t{:?}\nadjacents:\t{}\nfaces:\t\t{}\n",
//             self.name,
//             vertices,
//             adjacents
//                 .iter()
//                 .fold(String::new(), |acc, e| format!("{e}, {acc}")),
//             self.cycles.iter().fold(String::new(), |acc, f| format!(
//                 "[{}], {acc}",
//                 f.iter().fold(String::new(), |acc, x| format!("{x}, {acc}"))
//             ))
//         ))
//     }
// }
