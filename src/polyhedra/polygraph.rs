pub use super::*;
use cgmath::{vec3, InnerSpace, Vector3, Zero};
use ndarray::Array;
use rand::random;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    u32,
};

#[derive(Debug, Default)]
pub struct PolyGraph {
    /// Conway Polyhedron Notation
    pub name: String,

    /// [Actual Graph]
    pub vertices: HashSet<VertexId>,
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
    /// Edges in the process of contracting visually
    pub contracting_edges: HashSet<Edge>,
    /// Edge length
    pub edge_length: f32,
}

impl PolyGraph {
    /// New with n vertices
    pub fn new_disconnected(vertex_count: usize) -> Self {
        let mut poly = Self {
            vertices: (0..vertex_count).collect(),
            adjacency_matrix: (0..vertex_count)
                .map(|x| (x, (0..vertex_count).map(|y| (y, false)).collect()))
                .collect(),
            positions: (0..vertex_count)
                .map(|x| (x, vec3(random(), random(), random()).normalize()))
                .collect(),
            speeds: (0..vertex_count).map(|x| (x, Vector3::zero())).collect(),
            edge_length: 1.0,
            ..Default::default()
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
        let new_id = self.vertices.iter().max().unwrap() + 1;
        // Adjacency matrix update
        for (_, l) in self.adjacency_matrix.iter_mut() {
            (*l).insert(new_id, false);
        }
        self.adjacency_matrix
            .insert(new_id, self.vertices.iter().map(|v| (*v, false)).collect());
        self.vertices.insert(new_id);
        // Position and speed
        self.positions.insert(
            new_id,
            pos.unwrap_or(Vector3::new(random(), random(), random()).normalize()),
        );
        self.speeds.insert(new_id, Vector3::zero());

        new_id
    }

    pub fn delete(&mut self, v: &VertexId) {
        for (_, l) in self.adjacency_matrix.iter_mut() {
            (*l).remove(v);
        }
        self.vertices.remove(v);
        self.adjacency_matrix.remove(v);
        self.positions.remove(v);
        self.speeds.remove(v);
    }

    /// Edges of a vertex
    pub fn edges(&self, v: &VertexId) -> Vec<Edge> {
        self.connections(v)
            .into_iter()
            .map(|other| (*v, other).into())
            .collect()
    }

    /// Number of faces
    pub fn face_count(&mut self) -> i64 {
        2 + self.adjacents.len() as i64 - self.adjacency_matrix.len() as i64
    }

    // Vertices that are connected to a given vertex
    pub fn connections(&self, v: &VertexId) -> HashSet<VertexId> {
        if let Some(l) = self.adjacency_matrix.get(v) {
            l.iter()
                .filter_map(|(k, v)| if *v { Some(*k) } else { None })
                .collect::<HashSet<usize>>()
        } else {
            HashSet::new()
        }
    }

    pub fn ghost_connections(&self, v: &VertexId) -> HashSet<VertexId> {
        let mut connections = HashSet::new();
        for (ge, le) in self.ghost_edges.iter() {
            if let Some(u) = ge.other(v) {
                if self.vertices.contains(&u) {
                    connections.insert(le.other(v).unwrap());
                }
            }
        }

        connections
    }

    /// All faces
    pub fn faces(&mut self) {
        let mut triplets = Vec::<Face>::new();
        let mut cycles = HashSet::<Face>::new();

        // find all the triplets
        for u in self.vertices.iter() {
            let adj: HashSet<VertexId> = self.connections(u);
            for x in adj.iter() {
                for y in adj.iter() {
                    if x != y && u < x && x < y {
                        let new_face = Face(vec![*x, *u, *y]);
                        if self.adjacents.contains(&(*x, *y).into()) {
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
            for v in self.connections(&p[p.len() - 1]) {
                if v > p[1] {
                    let c = self.connections(&v);
                    // if v is not a neighbor of u_2..u_t-1
                    if !p[1..p.len() - 1].iter().any(|vi| c.contains(vi)) {
                        let new_face = Face([p.clone(), vec![v]].concat());
                        if self.connections(&p[0]).contains(&v) {
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
        self.adjacents = self.vertices.iter().flat_map(|v| self.edges(v)).fold(
            HashSet::<Edge>::new(),
            |mut acc, e| {
                acc.insert(e);
                acc
            },
        )
    }
    /// Neighbors
    pub fn neighbors(&mut self) {
        let mut neighbors = HashSet::<Edge>::new();
        for u in self.vertices.iter() {
            for v in self.vertices.iter() {
                if self.dist[u][v] == 2 || self.dist[v][u] == 2 {
                    neighbors.insert((u, v).into());
                }
            }
        }
        self.neighbors = neighbors
    }

    fn APD(A: Vec<Vec<bool>>) {
        /*
        let n = self.vertices.len();
        let Z = A * A;

        let B = [n x n] (0-1 matrix)
            where
            Bij = 1 iff i != 1 && (Aij = 1 or Zij > 0)

        if Bij = 1 for all i!=j then return D = (2B-A)
        else
        let T = APD(B);
        let X = T * A;
        return D
            where
            Dij = {
                2Tij   if Xij >= Tij * degree(j)
                2Tij-1 if Xij < Tij * degree(j)
            }
            */
    }

    pub fn distances(&mut self) {
        /*
         * what if we actually kept around a HashMap<VertexId, usize> that turns vertex ids into
         * the correct index in the adjacency matrix and related structs?
         * this would prevent us from needing to use a hashmap / hashset for a lot of this stuff
         * and come with the added benefit that we can operate over the matrix data in a valid way.
         * might not be strictly necessary, but worth thinking about
         */

        let n = self.vertices.len();
        let mut ids = self.vertices.clone().into_iter().collect::<Vec<_>>();
        let mut keys = self
            .adjacency_matrix
            .clone()
            .into_keys()
            .collect::<Vec<_>>();
        ids.sort();
        keys.sort();
        println!("ids: {:?}\nkey: {:?}", ids, keys);
        // Adjacency matrix
        //let mut A = vec![vec![false; n]; n];
        //let mut A = Array::from_elem((n, n), 0);

        let data = (0..n).into_iter().fold(Vec::new(), |acc, i| {
            [
                acc,
                (0..n)
                    .into_iter()
                    .map(|j| {
                        if i != j && self.adjacency_matrix[&ids[i]][&ids[j]] {
                            1
                        } else {
                            0
                        }
                    })
                    .collect(),
            ]
            .concat()
        });

        let A = Array::from_shape_vec((n, n), data);
        //println!("A: {:#?}", A);
        //println!("AM: {:#?}", self.adjacency_matrix);

        // A is the 0-1 adjacency matrix
        // D is the distance matrix of G
        // Aij = 1 iff i and j are adjacent in G
        // O(M(n)log(n))
        /*
        APD(A)
        */

        // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
        let mut dist: HashMap<VertexId, HashMap<VertexId, u32>> = self
            .vertices
            .iter()
            .map(|v| {
                (
                    *v,
                    self.vertices
                        .iter()
                        .map(|u| {
                            (
                                *u,
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

        for k in self.vertices.iter() {
            for i in self.vertices.iter() {
                for j in self.vertices.iter() {
                    if dist[i][k] != u32::MAX && dist[k][j] != u32::MAX {
                        let nv = dist[i][k] + dist[k][j];
                        if dist[i][j] > nv || dist[j][i] > nv {
                            dist.get_mut(i).unwrap().insert(*j, nv);
                            dist.get_mut(j).unwrap().insert(*i, nv);
                        }
                    }
                }
            }
        }

        self.dist = dist;
    }

    /// Periphery / diameter
    pub fn diameter(&mut self) {
        if let Some(max) = self
            .dist
            .values()
            .flatten()
            .map(|(_, d)| d)
            .filter(|&d| d < &u32::MAX)
            .max()
        {
            let mut diameter = HashSet::<Edge>::new();
            for u in self.vertices.iter() {
                for v in self.vertices.iter() {
                    if &self.dist[u][v] == max || &self.dist[v][u] == max {
                        diameter.insert((u, v).into());
                    }
                }
            }
            self.diameter = diameter
        }
    }

    //pub fn nearest(&)

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
        let mut vertices = self.vertices.iter().collect::<Vec<_>>();
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
        assert_eq!(graph.connections(&0), vec![1, 2].into_iter().collect());
        assert_eq!(graph.connections(&1), vec![0, 2].into_iter().collect());
        assert_eq!(graph.connections(&2), vec![0, 1].into_iter().collect());
        assert_eq!(graph.connections(&3), vec![].into_iter().collect());

        // Disconnect
        graph.disconnect((0, 1));
        assert_eq!(graph.connections(&0), vec![2].into_iter().collect());
        assert_eq!(graph.connections(&1), vec![2].into_iter().collect());

        // Delete
        graph.delete(&1);
        assert_eq!(graph.connections(&0), vec![2].into_iter().collect());
        assert_eq!(graph.connections(&1), vec![].into_iter().collect());
        assert_eq!(graph.connections(&2), vec![0].into_iter().collect());
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
