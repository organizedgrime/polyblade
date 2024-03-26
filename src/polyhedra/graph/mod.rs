mod conway;
mod edge;
mod face;
mod vertex;

use std::collections::HashSet;

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
    fn insert(&mut self) -> V;
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
        let vertices = self.vertices();
        let adjacents = self.adjacents();

        // Track all neighbors
        let mut neighbors = HashSet::new();

        // For each point
        for v1 in vertices {
            // Grab its adjacents
            for v2 in self.connections(v1.id()) {
                for v3 in self.connections(v1.id()) {
                    let e = (v2, v3).into();
                    if v2 != v3 && !adjacents.contains(&e) {
                        neighbors.insert(e);
                    }
                }
            }
        }

        neighbors
    }
    /// Periphery / diameter
    fn diameter(&self) -> HashSet<Edge> {
        let mut non_diameter = self.adjacents(); //HashSet::new();
        let mut diameter = HashSet::new();

        let mut modi = true;
        while modi {
            modi = false;
            for edge in non_diameter.clone().iter() {
                let id = edge.id();
                for j in self.connections(id.1) {
                    if id.0 != j {
                        let l = non_diameter.len();
                        non_diameter.insert((id.0, j).into());
                        diameter.insert((id.0, j).into());

                        if non_diameter.len() > l {
                            modi = true;
                        }
                    }
                }
            }

            if modi {
                diameter = HashSet::new();
            }
        }

        &non_diameter - &diameter
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

    fn insert(&mut self) -> usize {
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

    fn insert(&mut self) -> Point {
        let point = Point::new(self.points.len(), HashSet::new());
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
