pub use super::*;
use cgmath::{vec3, InnerSpace, Vector3, Zero};
use rand::random;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    u32,
};

#[derive(Debug, Serialize, Deserialize)]
pub struct PolyGraph {
    /// Conway Polyhedron Notation
    pub name: String,

    /// [Actual Graph]
    /// Adjacents
    pub adjacency_matrix: HashMap<VertexId, HashMap<VertexId, bool>>,
    /// Edges that have had Vertices split
    pub ghost_edges: HashMap<Edge, Edge>,

    /// [Derived Properties]
    /// Faces
    pub faces: Vec<Face>,
    /// Edge sets
    pub adjacents: HashSet<Edge>,
    pub neighbors: HashSet<Edge>,
    pub diameter: HashSet<Edge>,
    /// Distances between all points
    pub dist: HashMap<VertexId, HashMap<VertexId, u32>>,

    /// [Render Properties]
    /// Positions in 3D space
    pub positions: HashMap<VertexId, Vector3<f32>>,
    /// Speeds
    pub speeds: HashMap<VertexId, Vector3<f32>>,
    /// Edge length
    pub edge_length: f32,
}

impl PolyGraph {
    /// New with n vertices
    pub fn new_disconnected(vertex_count: usize) -> Self {
        let mut poly = Self {
            name: String::new(),
            adjacency_matrix: (0..vertex_count)
                .map(|x| {
                    (
                        x,
                        (0..vertex_count).map(|y| (y, false)).collect(), // vec![false; vertex_count]
                    )
                })
                .collect(),
            ghost_edges: HashMap::new(),
            faces: vec![],
            adjacents: HashSet::new(),
            neighbors: HashSet::new(),
            diameter: HashSet::new(),
            dist: HashMap::new(),
            positions: (0..vertex_count)
                .map(|x| (x, vec3(random(), random(), random()).normalize()))
                .collect(),
            speeds: (0..vertex_count).map(|x| (x, Vector3::zero())).collect(),
            edge_length: 1.0,
        };
        poly.recompute_qualities();
        poly
    }

    /// New known shape
    pub fn new_platonic(name: &str, points: Vec<Vec<usize>>) -> Self {
        let mut poly = Self::new_disconnected(points.len());
        poly.name = String::from(name);
        for (v, conns) in points.into_iter().enumerate() {
            for u in conns {
                poly.connect(Into::<Edge>::into((v, u)).id());
            }
        }

        poly.recompute_qualities();
        poly
    }

    /// Vertex
    pub fn vertex_exists(&self, v: &VertexId) -> bool {
        self.adjacency_matrix.contains_key(v)
    }

    pub fn vertex_count(&self) -> usize {
        self.adjacency_matrix.len()
    }

    pub fn vertices(&self) -> Vec<VertexId> {
        self.adjacency_matrix.clone().into_keys().collect()
    }

    pub fn connect(&mut self, e: impl Into<Edge>) {
        let (v, u) = e.into().id();
        self.adjacency_matrix.get_mut(&v).unwrap().insert(u, true);
        self.adjacency_matrix.get_mut(&u).unwrap().insert(v, true);
    }

    pub fn disconnect(&mut self, e: impl Into<Edge>) {
        let (v, u) = e.into().id();
        self.adjacency_matrix.get_mut(&v).unwrap().insert(u, false);
        self.adjacency_matrix.get_mut(&u).unwrap().insert(v, false);
    }

    pub fn insert(&mut self, pos: Option<Vector3<f32>>) -> VertexId {
        let existing_vertices = self.vertices();
        let new_id = self.adjacency_matrix.keys().max().unwrap() + 1;

        // Adjacency matric update
        for (_, l) in self.adjacency_matrix.iter_mut() {
            (*l).insert(new_id, false);
        }
        self.adjacency_matrix.insert(
            new_id,
            existing_vertices.into_iter().map(|v| (v, false)).collect(),
        );
        // Position and speed
        self.positions.insert(
            new_id,
            pos.unwrap_or(Vector3::new(random(), random(), random()).normalize()),
        );
        self.speeds.insert(new_id, Vector3::zero());

        new_id
    }

    pub fn delete(&mut self, id: usize) {
        for (_, l) in self.adjacency_matrix.iter_mut() {
            (*l).remove(&id);
        }
        self.adjacency_matrix.remove(&id);
        self.positions.remove(&id);
        self.speeds.remove(&id);
    }

    /// Edges of a vertex
    pub fn edges(&self, v: VertexId) -> Vec<Edge> {
        self.connections(v)
            .into_iter()
            .map(|other| (v, other).into())
            .collect()
    }

    /// Number of faces
    pub fn face_count(&mut self) -> i64 {
        2 + self.adjacents.len() as i64 - self.adjacency_matrix.len() as i64
    }

    // Vertices that are connected to a given vertex
    pub fn connections(&self, v: usize) -> HashSet<VertexId> {
        let mut connections = HashSet::<VertexId>::new();
        if let Some(list) = self.adjacency_matrix.get(&v) {
            for (other, connected) in list.iter() {
                if *connected && other != &v {
                    connections.insert(*other);
                }
            }
        }

        for (ge, le) in self.ghost_edges.iter() {
            if let Some(u) = ge.other(v) {
                if self.vertex_exists(&u) {
                    connections.insert(le.other(v).unwrap());
                }
            }
        }

        connections
    }

    /// All faces
    pub fn faces(&mut self) {
        let all_edges = self.adjacents.clone();
        let mut triplets = Vec::<Face>::new();
        let mut cycles = HashSet::<Face>::new();

        // find all the triplets
        for u in self.vertices() {
            let adj: HashSet<VertexId> = self.connections(u);
            for x in adj.clone().into_iter() {
                for y in adj.clone().into_iter() {
                    if x != y && u < x && x < y {
                        let new_face = Face(vec![x, u, y]);
                        if all_edges.contains(&(x, y).into()) {
                            cycles.insert(new_face);
                        } else {
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }
        // while there are unparsed triplets
        while !triplets.is_empty() && (cycles.len() as i64) < self.face_count() {
            let triplet = triplets.remove(0);
            let p = triplet.0;
            // for each v adjacent to u_t
            for v in self.connections(p[p.len() - 1]) {
                if v > p[1] {
                    let c = self.connections(v);
                    // if v is not a neighbor of u_2..u_t-1
                    if !p[1..p.len() - 1].iter().any(|vi| c.contains(vi)) {
                        let new_face = Face([p.clone(), vec![v]].concat());
                        if self.connections(p[0]).contains(&v) {
                            //cycles.remo
                            cycles.insert(new_face);
                        } else {
                            //println!("lengthened: {:?}", new_face);
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }

        self.faces = cycles.into_iter().collect();
    }

    /// All edges
    pub fn adjacents(&mut self) {
        self.adjacents = self
            .vertices()
            .into_iter()
            .flat_map(|v| self.edges(v))
            .fold(HashSet::<Edge>::new(), |mut acc, e| {
                acc.insert(e);
                acc
            })
    }
    /// Neighbors
    pub fn neighbors(&mut self) {
        let dist = self.dist.clone();

        let mut neighbors = HashSet::<Edge>::new();
        for u in self.vertices() {
            for v in self.vertices() {
                if dist[&u][&v] == 2 || dist[&v][&u] == 2 {
                    neighbors.insert((u, v).into());
                }
            }
        }
        self.neighbors = neighbors
    }

    pub fn distances(&mut self) {
        // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
        let mut dist: HashMap<VertexId, HashMap<VertexId, u32>> = self
            .vertices()
            .into_iter()
            .map(|v| {
                (
                    v,
                    self.vertices()
                        .into_iter()
                        .map(|u| {
                            (
                                u,
                                if u == v {
                                    0
                                } else if self.adjacents.contains(&(v, u).into()) {
                                    1
                                } else {
                                    u32::MAX
                                },
                            )
                        })
                        .collect(),
                )
            })
            .collect();

        for k in self.vertices() {
            for i in self.vertices() {
                for j in self.vertices() {
                    if dist[&i][&k] != u32::MAX && dist[&k][&j] != u32::MAX {
                        let nv = dist[&i][&k] + dist[&k][&j];
                        if dist[&i][&j] > nv || dist[&j][&i] > nv {
                            dist.get_mut(&i).unwrap().insert(j, nv);
                            dist.get_mut(&j).unwrap().insert(i, nv);
                        }
                    }
                }
            }
        }

        self.dist = dist;
    }

    /// Periphery / diameter
    pub fn diameter(&mut self) {
        let dist = self.dist.clone();
        if let Some(max) = dist
            .values()
            .flatten()
            .map(|(_, d)| d)
            .filter(|&d| d < &u32::MAX)
            .max()
        {
            let mut diameter = HashSet::<Edge>::new();
            for u in self.vertices() {
                for v in self.vertices() {
                    if &dist[&u][&v] == max || &dist[&v][&u] == max {
                        diameter.insert((u, v).into());
                    }
                }
            }
            self.diameter = diameter
        }
    }

    pub fn recompute_qualities(&mut self) {
        //
        self.adjacents();
        self.distances();
        // Neighbors and diameters rely on distances
        self.neighbors();
        self.diameter();
        self.faces();
    }
}

impl Display for PolyGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vertices = self.vertices();
        vertices.sort();
        let mut adjacents = self.adjacents.clone().into_iter().collect::<Vec<_>>();
        adjacents.sort();

        f.write_fmt(format_args!(
            "name:\t\t{}\nvertices:\t{:?}\nadjacents:\t{}\nfaces:\t\t{}\n",
            self.name,
            vertices,
            adjacents
                .iter()
                .fold(String::new(), |acc, e| format!("{e}, {acc}")),
            self.faces.iter().fold(String::new(), |acc, f| format!(
                "[{}], {acc}",
                f.0.iter()
                    .fold(String::new(), |acc, x| format!("{x}, {acc}"))
            ))
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn basics() {
        let mut graph = PolyGraph::new_disconnected(4);
        // Connect
        graph.connect((0, 1));
        graph.connect((0, 2));
        graph.connect((1, 2));
        assert_eq!(graph.connections(0), vec![1, 2].into_iter().collect());
        assert_eq!(graph.connections(1), vec![0, 2].into_iter().collect());
        assert_eq!(graph.connections(2), vec![0, 1].into_iter().collect());
        assert_eq!(graph.connections(3), vec![].into_iter().collect());

        // Disconnect
        graph.disconnect((0, 1));
        assert_eq!(graph.connections(0), vec![2].into_iter().collect());
        assert_eq!(graph.connections(1), vec![2].into_iter().collect());

        // Delete
        graph.delete(1);
        assert_eq!(graph.connections(0), vec![2].into_iter().collect());
        assert_eq!(graph.connections(1), vec![].into_iter().collect());
        assert_eq!(graph.connections(2), vec![0].into_iter().collect());
    }

    #[test]
    fn chordless_cycles() {
        let mut graph = PolyGraph::new_disconnected(4);
        // Connect
        graph.connect((0, 1));
        graph.connect((1, 2));
        graph.connect((2, 3));

        graph.recompute_qualities();
        assert_eq!(graph.faces.len(), 0);

        graph.connect((2, 0));
        graph.recompute_qualities();
        assert_eq!(graph.faces, vec![Face(vec![0, 1, 2])]);
    }
}
