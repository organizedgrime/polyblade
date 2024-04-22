pub use super::*;
use cgmath::{vec3, InnerSpace, Vector3, Zero};
use rand::random;
use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
    u32,
};
type VertMap<T> = HashMap<VertexId, T>;

#[derive(Debug, Default)]
pub struct PolyGraph {
    /// Conway Polyhedron Notation
    pub name: String,

    /// [Actual Graph]
    pub vertices: HashSet<VertexId>,
    /// Vertices that are adjacent
    pub adjacents: HashSet<Edge>,
    /// Edges that have had Vertices split
    pub ghost_edges: HashMap<Edge, Edge>,

    /// [Derived Properties]
    /// Faces
    pub faces: Vec<Face>,
    /// Edge sets
    pub neighbors: HashSet<Edge>,
    pub diameter: HashSet<Edge>,
    /// Distances between all points
    pub dist: HashMap<Edge, usize>,

    /// [Render Properties]
    /// Positions in 3D space
    pub positions: VertMap<Vector3<f32>>,
    /// Speeds
    pub speeds: VertMap<Vector3<f32>>,
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
        self.adjacents.insert(e.into());
    }

    pub fn disconnect(&mut self, e: impl Into<Edge>) {
        self.adjacents.remove(&e.into());
    }

    pub fn insert(&mut self) -> VertexId {
        let new_id = self.vertices.iter().max().unwrap() + 1;
        self.vertices.insert(new_id);
        // Position and speed
        self.positions.insert(
            new_id,
            Vector3::new(random(), random(), random()).normalize(),
        );
        self.speeds.insert(new_id, Vector3::zero());
        new_id
    }

    pub fn delete(&mut self, v: &VertexId) {
        self.vertices.remove(v);
        self.adjacents = self
            .adjacents
            .clone()
            .into_iter()
            .filter(|e| e.id().0 != *v && e.id().1 != *v)
            .collect();
        self.positions.remove(v);
        self.speeds.remove(v);
    }

    /// Edges of a vertex
    pub fn edges(&self, v: &VertexId) -> Vec<Edge> {
        self.adjacents
            .iter()
            .filter_map(|e| if e.other(v).is_some() { Some(*e) } else { None })
            .collect()
    }

    /// Number of faces
    pub fn face_count(&mut self) -> i64 {
        2 + self.adjacents.len() as i64 - self.vertices.len() as i64
    }

    // Vertices that are connected to a given vertex
    pub fn connections(&self, v: &VertexId) -> HashSet<VertexId> {
        self.adjacents
            .iter()
            .filter(|e| e.id().0 == *v || e.id().1 == *v)
            .map(|e| e.other(v).unwrap())
            .collect()
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

    /// Neighbors
    pub fn neighbors(&mut self) {
        let mut neighbors = HashSet::<Edge>::new();
        for u in self.vertices.iter() {
            for v in self.vertices.iter() {
                let e: Edge = (v, u).into();
                if self.dist.get(&e) == Some(&2) {
                    neighbors.insert(e);
                }
            }
        }
        self.neighbors = neighbors
    }

    /*
    fn extend(
        &self,
        mut v: Vertex,
        d: usize,
        mut dist: VertMatrix<u32>,
        mut paths: VertMatrix<usize>,
    ) -> (VertMatrix<u32>, VertMatrix<usize>) {
        if d == 1 {
            for w in v.adj {
                dist.get_mut(&w.id).unwrap().insert(v.id, d as u32);
                paths.get_mut(&w.id).unwrap().insert(v.id, v.id);
                let mut w_prime = T_Vertex::new(w.id);
                w_prime.cor = Some(w.root);
                v.v_prime.children.push(w_prime);
            }
        } else {
            let n = dist.len();
            while let Some((i, w_prime)) = v
                .dqueue
                .iter()
                .enumerate()
                .filter(|(i, (v, dx))| *dx == (d - 1))
                .map(|(i, (v, dx))| (i, v))
                .last()
            {
                // remove it
                //v.dqueue.

                if let Some(cor) = &w_prime.cor {
                    for x_pprime in &cor.children {
                        let x = &x_pprime.vertex;

                        if paths[&x.id][&v.id] == VertexId::MAX {
                            dist.get_mut(&x.id).unwrap().insert(v.id, d as u32);
                            //dist[&x.id][&v.id] = d as u32;
                            let w = &w_prime.vertex;
                            paths.get_mut(&x.id).unwrap().insert(v.id, w.id);
                            //paths[&x.id][&v.id] = w.id;
                            let mut x_prime = T_Vertex::new(x.id);
                            x_prime.cor = Some(Rc::new(x_pprime.clone()));
                        }
                    }
                }
            }
        }

        (dist, paths)
    }
    */

    fn pst(&mut self) {
        let n = self.vertices.len();
        /// Vertex
        //
        // d-queues associated w each vertex
        let mut dqueues: HashMap<VertexId, Vec<(VertexId, usize)>> = Default::default();
        let mut children: HashMap<VertexId, Vec<VertexId>> = Default::default();
        let mut parents: HashMap<VertexId, VertexId> = Default::default();
        // Counters for vertices whos shortest paths have already been obtained
        let mut counters: HashMap<VertexId, usize> =
            self.vertices.iter().map(|v| (*v, 1)).collect();
        let mut cors: HashMap<VertexId, usize> = Default::default();
        let mut roots: HashMap<VertexId, VertexId> = Default::default();

        // The element D[i, j] represents the distance from v_i to vj.
        let mut dist: HashMap<Edge, usize> = Default::default();
        // The element S[i,j] represents the parent of v_i on the shortest path from v_i to a source
        // vertex v_j.
        let mut paths: HashMap<Edge, usize> = Default::default();

        // let the diagonal elements of S already be initialized to NO_PARENT (-1) and all other
        // elements to NOT_SEARCHED (0). NO_PARENT means v_i is a source vertex.
        for i in self.vertices.iter() {
            for j in self.vertices.iter() {
                if i != j {
                    let e = (i, j).into();
                    dist.insert(e, 0);
                    paths.insert(e, 0);
                }
            }
            paths.insert((i, i).into(), VertexId::MAX);
        }

        let mut verts: HashSet<VertexId> = self.vertices.clone();
        let mut depth = 0;
        while 0 < verts.len() {
            println!("d: {depth}, D: {dist:?}, S: {paths:?}");
            println!("dqueues: {dqueues:?}, children: {children:?}, parents: {parents:?}");
            println!("cors: {cors:?}, roots: {roots:?}");
            depth += 1;
            let mut v_new = HashSet::new();
            for v in verts.iter() {
                if depth == 1 {
                    for w in self.connections(v) {
                        let e: Edge = (w, *v).into();
                        // D[w.id, v.id] = d
                        dist.insert(e, depth);
                        // S[w.id, v.id] = v.id
                        paths.insert(e, *v);
                        //w'=T_V(w)
                        //w'.cor = w.root
                        cors.insert(w, roots.get(&w).unwrap_or(&w).clone());
                        // add w' to v'.children
                        children.entry(*v).or_default().push(w);
                        // w'.parent = v'
                        parents.insert(w, *v);
                        // v.que.enque(w', 1)
                        dqueues.entry(*v).or_default().push((w, 1));
                        // v.c = v.c + 1
                        *counters.get_mut(v).unwrap() += 1;
                    }
                } else {
                    // n = len(D)
                    // w' = v.que.deque(d - 1)
                    // while w' is not None:
                    while let Some((wp, _d)) = dqueues
                        .entry(*v)
                        .or_default()
                        .iter()
                        .filter(|(_, vd)| *vd == depth - 1)
                        .last()
                        .map(|x| x.clone())
                    //.map(|(w, _)| w.clone())
                    {
                        // actually finish the dequeuque
                        //vqueue.remove(wp);
                        let l = dqueues.entry(*v).or_default();
                        *l = l.clone().into_iter().filter(|(xx, _)| xx != &wp).collect();

                        // for x'' in w'.cor.children
                        for xpp in children.get(cors.get(&wp).unwrap()).unwrap().clone() {
                            let e: Edge = (xpp, *v).into();
                            if *paths.entry(e).or_insert(0) == 0 {
                                // D[x.id, v.id] = d;
                                dist.insert(e, depth);
                                // S[x.id, v.id] = w.id;
                                paths.insert(e, wp);
                                // x' = T_V(x)
                                // x.cor = x''
                                cors.insert(xpp, xpp);
                                // add w' to w' children
                                children.entry(wp).or_default().push(xpp);
                                // x'.parent = w'
                                parents.insert(xpp, wp);
                                // v.que.enque(x', d)
                                dqueues.entry(*v).or_default().push((xpp, depth));
                                // v.c = v.c + 1
                                let vc = counters.entry(*v).or_insert(1);
                                *vc += 1;
                                if *vc == n {
                                    return;
                                }
                            }
                        }

                        // w' = v.que.deque(d-1)
                    }
                }

                if *counters.entry(*v).or_default() < n {
                    v_new.insert(*v);
                }
            }
            verts = v_new;
        }

        self.dist = dist;
    }

    pub fn distances(&mut self) {
        self.pst();

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

        let mut dd = HashMap::new();
        for v in self.vertices.iter() {
            for u in self.vertices.iter() {
                let dvu = dist[&v][&u];
                if dvu != u32::MAX && dvu != 0 {
                    let e = (v, u).into();
                    dd.insert(e, dvu as usize);
                }
            }
        }

        self.dist = dd;
    }

    /// Periphery / diameter
    pub fn diameter(&mut self) {
        if let Some(max) = self.dist.values().max() {
            self.diameter = self
                .dist
                .iter()
                .filter_map(|(e, d)| if d == max { Some(*e) } else { None })
                .collect();
        }
    }

    //pub fn nearest(&)

    pub fn recompute_qualities(&mut self) {
        //
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
