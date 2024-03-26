mod conway;
mod edge;
mod face;
mod vertex;

use std::{
    collections::{HashMap, HashSet},
    u32,
};

pub use conway::*;
pub use edge::*;
pub use face::*;
pub use vertex::*;

use super::{Point, Polyhedron};

pub trait Graph<V: Vertex>: Sized {
    /// New with n vertices
    fn new_disconnected(vertex_count: usize) -> Self;
    /// Vertex
    fn vertex(&self, id: VertexId) -> Option<V>;
    /// Edge
    fn edge(&self, id: EdgeId) -> Option<Edge> {
        if self.vertex(id.0).is_some() && self.vertex(id.1).is_some() {
            Some((id.0, id.1).into())
        } else {
            None
        }
    }
    /// All vertices
    fn vertices(&self) -> Vec<V>;
    /// Connect two vertices
    fn connect(&mut self, id: EdgeId);
    /// Disconnect two vertices
    fn disconnect(&mut self, id: EdgeId);
    /// New vertex
    fn insert(&mut self, neighbor: Option<VertexId>) -> V;
    /// Delete
    fn delete(&mut self, id: VertexId);
    /// Edges of a vertex
    fn edges(&self, id: VertexId) -> Vec<Edge> {
        if let Some(vertex) = self.vertex(id) {
            self.connections(id)
                .into_iter()
                .map(|other| (vertex.id(), other).into())
                .collect()
        } else {
            vec![]
        }
    }
    /// Number of faces
    fn face_count(&self) -> usize {
        2 + self.adjacents().len() - self.vertices().len()
    }
    // Faces
    // Vertices that are connected to a given vertex
    fn connections(&self, id: VertexId) -> HashSet<VertexId>;
    fn sorted_connections(&self, id: VertexId) -> Vec<VertexId> {
        let mut m = HashSet::<(VertexId, VertexId)>::new();
        for face in self.faces().into_iter() {
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
    /// All faces
    fn faces(&self) -> Vec<Face> {
        let all_edges = self.adjacents();
        let mut triplets = Vec::<Face>::new();
        let mut cycles = HashSet::<Face>::new();

        println!("finding triplets");
        // find all the triplets
        for u in self.vertices() {
            let adj: HashSet<VertexId> = self.connections(u.id());
            for x in adj.clone().into_iter() {
                for y in adj.clone().into_iter() {
                    if x != y && u.id().clone() < x && x < y {
                        let new_face = Face(vec![x, u.id(), y]);
                        if all_edges.contains(&(x, y).into()) {
                            cycles.insert(new_face);
                        } else {
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }
        println!("processing triplets");
        // while there are unparsed triplets
        while !triplets.is_empty() && cycles.len() < self.face_count() {
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
                            println!("found new cycle: {:?}", new_face);
                            cycles.insert(new_face);
                        } else {
                            //println!("lengthened: {:?}", new_face);
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }
        println!("done");

        cycles.into_iter().collect()
    }
    /// All edges
    fn adjacents(&self) -> HashSet<Edge> {
        self.vertices()
            .iter()
            .flat_map(|v| self.edges(v.id()))
            .fold(HashSet::<Edge>::new(), |mut acc, e| {
                acc.insert(e);
                acc
            })
    }
    /// Neighbors
    fn neighbors(&self) -> HashSet<Edge> {
        let V = self.vertices().len();
        let dist = self.distances();

        let mut neighbors = HashSet::<Edge>::new();
        for u in 0..V {
            for v in 0..V {
                if dist[u][v] == 2 || dist[v][u] == 2 {
                    neighbors.insert((u, v).into());
                }
            }
        }
        neighbors
    }

    fn distances(&self) -> Vec<Vec<u32>> {
        let V = self.vertices().len();
        // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
        let mut dist = vec![vec![u32::MAX; V]; V];

        for edge in self.adjacents() {
            // The weight of the edge (u, v)
            dist[edge.id().0][edge.id().1] = 1;
            dist[edge.id().1][edge.id().0] = 1;

            for v in self.vertices() {
                dist[v.id()][v.id()] = 0;
            }

            for k in 0..V {
                for i in 0..V {
                    for j in 0..V {
                        if dist[i][k] != u32::MAX && dist[k][j] != u32::MAX {
                            if dist[i][j] > dist[i][k] + dist[k][j] {
                                dist[i][j] = dist[i][k] + dist[k][j];
                                dist[j][i] = dist[i][k] + dist[k][j];
                            }
                        }
                    }
                }
            }
        }

        dist
    }

    /// Periphery / diameter
    fn diameter(&self) -> HashSet<Edge> {
        let V = self.vertices().len();
        let dist = self.distances();
        let max = dist
            .clone()
            .into_iter()
            .flatten()
            .filter(|&b| b < u32::MAX)
            .max()
            .unwrap();
        let mut diameter = HashSet::<Edge>::new();
        for u in 0..V {
            for v in 0..V {
                if dist[u][v] == max.clone() || dist[v][u] == max.clone() {
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
        println!("diameter N = {}: {:?}", max, diameter);
        diameter
    }
}

pub struct SimpleGraph {
    pub adjacency_matrix: Vec<Vec<bool>>,
    pub faces: Vec<Face>,
}

impl Graph<usize> for SimpleGraph {
    fn vertex(&self, id: VertexId) -> Option<usize> {
        if id < self.adjacency_matrix.len() {
            Some(id)
        } else {
            None
        }
    }
    fn new_disconnected(vertex_count: usize) -> Self {
        Self {
            adjacency_matrix: vec![vec![false; vertex_count]; vertex_count],
            faces: vec![],
        }
    }

    fn vertices(&self) -> Vec<usize> {
        (0..self.adjacency_matrix.len()).collect()
    }

    fn connect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.adjacency_matrix[edge.a][edge.b] = true;
            self.adjacency_matrix[edge.b][edge.a] = true;
        }
    }

    fn disconnect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.adjacency_matrix[edge.a][edge.b] = false;
            self.adjacency_matrix[edge.b][edge.a] = false;
        }
    }

    fn insert(&mut self, neighbor: Option<VertexId>) -> usize {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i].push(false);
        }

        self.adjacency_matrix
            .push(vec![false; self.adjacency_matrix.len() + 1]);

        self.adjacency_matrix.len() - 1
    }

    fn delete(&mut self, id: usize) {
        for i in 0..self.adjacency_matrix.len() {
            self.adjacency_matrix[i].remove(id);
        }
        self.adjacency_matrix.remove(id);
    }

    fn connections(&self, vertex: usize) -> HashSet<VertexId> {
        let mut connections = HashSet::<VertexId>::new();
        for (other, connected) in self.adjacency_matrix[vertex].iter().enumerate() {
            if *connected && other != vertex {
                connections.insert(other);
            }
        }
        connections
    }
}

impl Graph<Point> for Polyhedron {
    fn vertex(&self, id: VertexId) -> Option<Point> {
        self.points.get(id).cloned()
    }

    fn new_disconnected(vertex_count: usize) -> Self {
        Polyhedron {
            name: "".to_string(),
            points: (0..vertex_count).map(Point::new_empty).collect(),
            faces: vec![],
            enemies: HashSet::new(),
            edge_length: 1.0,
            adjacents: HashSet::new(),
            neighbors: HashSet::new(),
            diameter: HashSet::new(),
        }
    }

    fn connect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.points[edge.a].connect(edge.b);
            self.points[edge.b].connect(edge.a);
        }
    }

    fn disconnect(&mut self, id: EdgeId) {
        if let Some(edge) = self.edge(id) {
            self.points[edge.a].disconnect(edge.b);
            self.points[edge.b].disconnect(edge.a);
        }
    }

    fn insert(&mut self, neighbor: Option<VertexId>) -> Point {
        let mut point = Point::new(self.points.len(), HashSet::new());
        if let Some(v) = neighbor {
            point.xyz = self.points[v].xyz;
        }
        self.points.push(point.clone());
        point
    }

    fn delete(&mut self, id: VertexId) {
        for i in 0..self.points.len() {
            self.points[i].delete(id);
        }
        self.points.remove(id);
        self.points = self
            .points
            .clone()
            .into_iter()
            .enumerate()
            .map(|(new_id, mut v)| {
                v.id = new_id;
                v
            })
            .collect();
    }

    fn connections(&self, id: VertexId) -> HashSet<VertexId> {
        if let Some(vertex) = self.vertex(id) {
            vertex.adjacents.clone()
        } else {
            HashSet::new()
        }
    }

    fn vertices(&self) -> Vec<Point> {
        self.points.clone()
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;
    use test_case::test_case;

    #[test_case(SimpleGraph::new_disconnected(4) ; "SimpleGraph")]
    #[test_case(Polyhedron::new_disconnected(4) ; "Polyhedron")]
    fn basics<G: Graph<V>, V: Vertex>(mut graph: G) {
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
        assert_eq!(graph.connections(0), vec![1].into_iter().collect());
        assert_eq!(graph.connections(1), vec![0].into_iter().collect());
        assert_eq!(graph.connections(2), vec![].into_iter().collect());
    }

    #[test_case(SimpleGraph::new_disconnected(4) ; "SimpleGraph")]
    #[test_case(Polyhedron::new_disconnected(4) ; "Polyhedron")]
    fn chorsless_cycles<G: Graph<V>, V: Vertex>(mut graph: G) {
        // Connect
        graph.connect((0, 1));
        graph.connect((1, 2));
        graph.connect((2, 3));

        assert_eq!(graph.faces().len(), 0);

        graph.connect((2, 0));
        assert_eq!(graph.faces(), vec![Face(vec![0, 1, 2])]);
    }
}
