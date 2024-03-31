mod conway;
mod edge;
mod face;
mod vertex;

use cgmath::{vec3, InnerSpace, Vector3, Zero};
pub use edge::*;
pub use face::*;
use rand::random;
use serde::{Deserialize, Serialize};
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    u32,
};
pub use vertex::*;

#[derive(Debug, Serialize, Deserialize)]
pub struct Graph {
    /// Conway Polyhedron Notation
    pub name: String,

    /// [Actual Graph]
    /// Adjacents
    pub adjacency_matrix: HashMap<VertexId, HashMap<VertexId, bool>>,
    /// Nodes that have been split and their children
    pub ghost_matrix: HashMap<VertexId, HashSet<VertexId>>,
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

impl Graph {
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
            ghost_matrix: HashMap::new(),
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
                poly.connect(Edge { v, u }.id());
            }
        }

        poly.recompute_qualities();
        poly
    }

    /// Vertex
    pub fn vertex(&self, id: VertexId) -> Option<usize> {
        if self.adjacency_matrix.contains_key(&id) {
            Some(id)
        } else {
            None
        }
    }

    pub fn vertices(&self) -> Vec<VertexId> {
        self.adjacency_matrix.clone().into_keys().collect()
    }

    pub fn vertex_count(&self) -> usize {
        self.positions.len()
    }

    /// Edge
    pub fn edge(&self, id: EdgeId) -> Option<Edge> {
        if self.vertex(id.0).is_some() && self.vertex(id.1).is_some() {
            Some((id.0, id.1).into())
        } else {
            None
        }
    }

    pub fn connect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.adjacency_matrix
                .get_mut(&edge.v)
                .unwrap()
                .insert(edge.u, true);
            self.adjacency_matrix
                .get_mut(&edge.u)
                .unwrap()
                .insert(edge.v, true);
        }
    }

    pub fn disconnect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.adjacency_matrix
                .get_mut(&edge.v)
                .unwrap()
                .insert(edge.u, false);
            self.adjacency_matrix
                .get_mut(&edge.u)
                .unwrap()
                .insert(edge.v, false);
        }
    }

    pub fn insert(&mut self, pos: Option<Vector3<f32>>) -> VertexId {
        let existing_vertices = self.vertices();
        let new_id = self.adjacency_matrix.clone().into_keys().max().unwrap() + 1;

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
    pub fn edges(&self, id: VertexId) -> Vec<Edge> {
        if let Some(v) = self.vertex(id) {
            self.connections(id)
                .into_iter()
                .map(|other| (v, other).into())
                .collect()
        } else {
            vec![]
        }
    }
    /// Number of faces
    pub fn face_count(&mut self) -> i64 {
        self.adjacents();
        2 + self.adjacents.len() as i64 - self.vertices().len() as i64
    }
    // Faces
    // Vertices that are connected to a given vertex
    //fn connections(&self, id: VertexId) -> HashSet<VertexId>;
    pub fn connections(&self, vertex: usize) -> HashSet<VertexId> {
        let mut connections = HashSet::<VertexId>::new();
        if let Some(list) = self.adjacency_matrix.get(&vertex) {
            for (other, connected) in list.iter() {
                if *connected && other != &vertex {
                    connections.insert(*other);
                }
            }
        }

        for (k, v) in self.ghost_edges.iter() {
            if k.u == vertex || k.v == vertex {
                let xxx = k.other(vertex);
                if self.vertices().contains(&xxx) {
                    connections.insert(v.other(vertex));
                }
            }
        }
        connections
    }
    /*
    pub fn sorted_connections(&mut self, id: VertexId) -> Vec<VertexId> {
        let mut m = HashSet::<(VertexId, VertexId)>::new();
        for face in self.faces.clone().into_iter() {
            for i in 0..face.0.len() {
                if face.0[i] == id {
                    m.insert((
                        face.0[(i + face.0.len() - 1) % face.0.len()],
                        face.0[(i + 1) % face.0.len()],
                    ));
                }
            }
        }
        println!("id: {:?}", id);
        println!("m: {:?}", m);

        let mut root = m.clone().into_iter().collect::<Vec<_>>()[0].0;
        let mut conn = vec![root];

        while let Some(next) = m.clone().into_iter().find(|e| e.0 == root || e.1 == root) {
            root = if next.0 == root { next.1 } else { next.0 };
            if !conn.contains(&root) {
                conn.push(root);
            }
            m.remove(&next);
        }

        println!("conn: {:?}", conn);
        println!("connold: {:?}", self.connections(id));

        for x in self.connections(id) {
            if !conn.contains(&x) {
                conn.push(x);
            }
        }

        conn
    }
    */

    pub fn ghosts(&self, id: VertexId) -> HashSet<VertexId> {
        if !self.ghost_matrix.contains_key(&id) {
            vec![id].into_iter().collect()
        } else {
            let l = self.ghost_matrix.get(&id).unwrap();
            l.clone()
            /*
            l.into_iter()
                .map(|g| self.ghosts(*g))
                .fold(HashSet::new(), |mut acc, l| {
                    acc.extend(l);
                    acc
                })
                */
        }
    }

    /// All faces
    pub fn faces(&mut self) {
        let all_edges = self.adjacents.clone();
        let mut triplets = Vec::<Face>::new();
        let mut cycles = HashSet::<Face>::new();

        println!("finding triplets");
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
                if dist.contains_key(&u)
                    && dist.contains_key(&v)
                    && (dist[&u][&v] == 2 || dist[&v][&u] == 2)
                {
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
                    if dist.contains_key(&j) && dist.contains_key(&i) && dist.contains_key(&k) {
                        //let lj = dist.get_mut(&j);
                        if dist[&i][&k] != u32::MAX && dist[&k][&j] != u32::MAX {
                            let nv = dist[&i][&k] + dist[&k][&j];
                            if dist[&i][&j] > nv || dist[&j][&i] > nv {
                                {
                                    let li = dist.get_mut(&i).unwrap();
                                    li.insert(j, nv);
                                }
                                {
                                    let lj = dist.get_mut(&j).unwrap();
                                    lj.insert(i, nv);
                                }
                            }
                        }
                    }
                }
            }
        }

        self.dist = dist;
    }

    /*
    pub fn distances(&mut self) {
        let V = self.vertices().len();
        // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)

        let mut id_map: Vec<VertexId> = self
            .adjacency_matrix
            .clone()
            .into_iter()
            .map(|(k, _)| k)
            .collect();

        let mut lookup: Vec<usize> = self
            .vertices()
            .into_iter()
            .map(|v| id_map.iter().position(|u| u == &v).unwrap())
            .collect();

        let mut dist = vec![vec![u32::MAX; V]; V];

        for edge in self.adjacents.clone() {
            // The weight of the edge (u, v)
            dist[lookup[edge.id().0]][lookup[edge.id().1]] = 1;
            dist[lookup[edge.id().1]][lookup[edge.id().0]] = 1;

            for v in self.vertices() {
                dist[lookup[v.id()]][lookup[v.id()]] = 0;
            }

            for k in 0..V {
                for i in 0..V {
                    for j in 0..V {
                        if dist[lookup[i]][lookup[k]] < u32::MAX
                            && dist[lookup[k]][lookup[j]] < u32::MAX
                        {
                            let nv = dist[lookup[i]][lookup[k]] + dist[lookup[k]][lookup[j]];
                            dist[lookup[i]][lookup[j]] = nv;
                            dist[lookup[j]][lookup[i]] = nv;
                        }
                    }
                }
            }
        }

        for (i, li) in dist.iter().enumerate() {
            for (j, d) in li.iter().enumerate() {
                if let Some(li)

            }
        }
    }
    */

    /// Periphery / diameter
    pub fn diameter(&mut self) {
        let dist = self.dist.clone();
        if let Some(max) = dist
            .clone()
            .into_values()
            .flatten()
            .map(|(_, d)| d)
            .filter(|&d| d < u32::MAX)
            .max()
        {
            let mut diameter = HashSet::<Edge>::new();
            for u in self.vertices() {
                for v in self.vertices() {
                    if dist.contains_key(&u)
                        && dist.contains_key(&v)
                        && (dist[&u][&v] == max || dist[&v][&u] == max)
                    {
                        diameter.insert((u, v).into());
                    }
                }
            }
            /*
            if diameter.len() < (V / 4) {
                println!("including some more, N = {} was too high", max);
                max -= 1;
                for u in 0..V {
                    for v in 0..V {
                        if dist[u][v] == max.clone() || dist[v][u] == max.clone() {
                            diameter.insert((u, v).into());
                        }
                    }
                }
            }
            */
            //println!("diameter N = {}: {:?}", max, diameter);
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

impl Display for Graph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut vertices = self.vertices();
        vertices.sort();
        let mut adjacents = self.adjacents.clone().into_iter().collect::<Vec<_>>();
        adjacents.sort();

        f.write_fmt(format_args!(
            "name:\t{}\nvertices:\t{:?}\nadjacents:\t{}\nfaces:{:?}",
            self.name,
            vertices,
            adjacents
                .iter()
                .fold(String::new(), |acc, e| format!("{acc}, {e}")),
            self.faces
                .iter()
                .fold(String::new(), |acc, f| format!("{acc}, {:?}", f.0))
        ))
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn basics() {
        let mut graph = Graph::new_disconnected(4);
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
        let mut graph = Graph::new_disconnected(4);
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
