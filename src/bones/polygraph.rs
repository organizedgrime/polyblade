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
    /// Distance matrix
    pub matrix: Matrix,
    /// Faces / chordless cycles
    pub cycles: Vec<Face>,
    ///
    pub springs: HashSet<Edge>,
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
            matrix: Matrix::new(n),
            speeds: vec![Vec3::zero(); n],
            edge_length: 1.0,
            ..Default::default()
        }
    }

    // Use a Fibonacci Lattice to spread the points evenly around a sphere
    pub fn lattice(&mut self) {
        // Use a Fibonacci Lattice to evently distribute starting points on a sphere
        let phi = std::f32::consts::PI * (3.0 - 5.0f32.sqrt());
        for v in 0..self.matrix.len() {
            let y = 1.0 - (v as f32 / (self.matrix.len() - 1) as f32);
            let radius = (1.0 - y * y).sqrt();
            let theta = (phi * (v as f32)) % (std::f32::consts::PI * 2.0);
            let x = theta.cos() * radius;
            let z = theta.sin() * radius;
            self.positions.push(Vec3::new(x, y, z));
        }
    }

    pub fn connect(&mut self, e: impl Into<Edge>) {
        let e = e.into();
        if e.v() != e.u() {
            self.matrix[e] = 1;
        }
    }

    pub fn disconnect(&mut self, e: impl Into<Edge>) {
        self.matrix[e.into()] = 0;
    }

    pub fn insert(&mut self) -> VertexId {
        self.positions
            .push(Vec3::new(random(), random(), random()).normalized());
        self.speeds.push(Vec3::zero());
        self.matrix.insert()
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

    // /// All faces
    // pub fn find_cycles(&mut self) {
    //     let mut triplets = Vec::<Face>::new();
    //     let mut cycles = HashSet::<Face>::default();
    //
    //     // find all the triplets
    //     for &u in self.vertices.iter() {
    //         let adj: HashSet<VertexId> = self.connections(u);
    //         for &x in adj.iter() {
    //             for &y in adj.iter() {
    //                 if x != y && u < x && x < y {
    //                     let new_face = Face::new(vec![x, u, y]);
    //                     if self.edges.contains(&(x, y).into()) {
    //                         cycles.insert(new_face);
    //                     } else {
    //                         triplets.push(new_face);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //     // while there are unparsed triplets
    //     while !triplets.is_empty() && (cycles.len() as i64) < self.face_count() {
    //         let p = triplets.remove(0);
    //         // for each v adjacent to u_t
    //         for v in self.connections(p[p.len() - 1]) {
    //             if v > p[1] {
    //                 let c = self.connections(v);
    //                 // if v is not a neighbor of u_2..u_t-1
    //                 if !p[1..p.len() - 1].iter().any(|vi| c.contains(vi)) {
    //                     let mut new_face = p.clone();
    //                     new_face.push(v);
    //                     if self.connections(p[0]).contains(&v) {
    //                         //cycles.remo
    //                         cycles.insert(new_face);
    //                     } else {
    //                         triplets.push(new_face);
    //                     }
    //                 }
    //             }
    //         }
    //     }
    //
    //     self.cycles = cycles.into_iter().collect();
    // }
    //
    // pub fn pst(&mut self) {
    //     if self.edges.is_empty() {
    //         return;
    //     }
    //
    //     let n = self.vertices.len();
    //     // Vertex
    //     //
    //     // d-queues associated w each vertex
    //     // maps from v -> ( maps from d -> u )
    //     let mut dqueue: HashMap<VertexId, VecDeque<(VertexId, usize)>> = Default::default();
    //     //
    //     let mut children: HashMap<VertexId, Vec<VertexId>> = Default::default();
    //
    //     // Counters for vertices whos shortest paths have already been obtained
    //     let mut counters: HashMap<VertexId, usize> =
    //         self.vertices.iter().map(|v| (*v, n - 1)).collect();
    //
    //     // The element D[i, j] represents the distance from v_i to vj.
    //     let mut dist: HashMap<Edge, usize> = Default::default();
    //
    //     // d = 0
    //     let mut depth = 1;
    //     // while 0 < |V|
    //     loop {
    //         let verts: HashSet<VertexId> = counters
    //             .iter()
    //             .filter_map(|(v, c)| if *c == 0 { None } else { Some(*v) })
    //             .collect();
    //         if verts.is_empty() {
    //             break;
    //         }
    //
    //         let mut removed = false;
    //
    //         for v in verts.into_iter() {
    //             // for v in V
    //             // START EXTEND(v, d, D, S)
    //             if depth == 1 {
    //                 //
    //                 for e in self.edges(v) {
    //                     // Connected node
    //                     let w = e.other(v).unwrap();
    //                     // D[w.id, v.id] = d
    //                     dist.insert(e, 1);
    //                     // add w' to v'.children
    //                     children.entry(v).or_default().push(w);
    //                     // v.que.enque(w', 1)
    //                     dqueue.entry(v).or_default().push_back((w, 1));
    //                     dqueue.entry(w).or_default().push_back((v, 1));
    //                     // v.c = v.c + 1
    //                     *counters.entry(v).or_default() -= 1;
    //                     //*counters.entry(w).or_default() -= 1;
    //                     removed = true;
    //                 }
    //             } else {
    //                 // w = v.que.deque(d - 1)
    //                 // while w is not None:
    //                 'dq: loop {
    //                     let vqueue = dqueue.entry(v).or_default();
    //                     if let Some((w, d)) = vqueue.pop_front() {
    //                         if d != depth - 1 {
    //                             dqueue.entry(v).or_default().push_back((w, d));
    //                             break;
    //                         }
    //                         // for x in w.children
    //                         for x in children.entry(w).or_default().clone() {
    //                             let e: Edge = (x, v).into();
    //                             if x != v && !dist.contains_key(&e) {
    //                                 // D[x.id, v.id] = d;
    //                                 dist.insert(e, depth);
    //                                 // add x' to w' children
    //                                 children.entry(w).or_default().push(x);
    //                                 // v.que.enque(x', d)
    //                                 dqueue.entry(v).or_default().push_back((x, depth));
    //                                 dqueue.entry(x).or_default().push_back((v, depth));
    //                                 // v.c = v.c + 1
    //                                 removed = true;
    //                                 *counters.entry(v).or_default() -= 1;
    //                                 *counters.entry(x).or_default() -= 1;
    //                                 // if v.c == n: return
    //                                 if *counters.entry(x).or_default() == 0
    //                                     && *counters.entry(w).or_default() == 0
    //                                     && *counters.entry(v).or_default() == 0
    //                                 {
    //                                     break 'dq;
    //                                 }
    //                             }
    //                         }
    //                     } else {
    //                         break;
    //                     }
    //                 }
    //             }
    //         }
    //         // END EXTEND
    //         // d = d + 1
    //         depth += 1;
    //
    //         if !removed {
    //             self.dist = dist;
    //             log::error!("failed distance computation");
    //             return;
    //         }
    //     }
    //
    //     self.dist = dist;
    // }
    //
    //

    pub fn vertices(&self) -> std::ops::Range<VertexId> {
        (0..self.matrix.len()).into_iter()
    }

    pub fn springs(&mut self) {
        let diameter = self.matrix.diameter();
        self.springs = self
            .vertices()
            .zip(self.vertices())
            .filter(|&(v, u)| {
                v != u && (self.matrix[[v, u]] <= 2 || self.matrix[[v, u]] >= diameter - 1)
            })
            .map(|(v, u)| Edge::from((v, u)))
            .collect::<HashSet<_>>();

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

#[cfg(test)]
impl PolyGraph {
    pub fn floyd(&mut self) {
        // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
        let mut matrix: Matrix = Matrix::new(self.matrix.len());

        for k in matrix.vertices() {
            for i in matrix.vertices() {
                for j in matrix.vertices() {
                    if matrix[[i, k]] != usize::MAX && matrix[[k, j]] != usize::MAX {
                        let nv = matrix[[i, k]] + matrix[[k, j]];
                        if matrix[[i, j]] > nv || matrix[[j, i]] > nv {
                            matrix[[i, j]] = nv;
                        }
                    }
                }
            }
        }

        let mut dd = HashMap::default();
        for (v, u) in matrix.vertices().zip(matrix.vertices()) {
            let dvu = matrix[[v, u]];
            if dvu != usize::MAX && dvu != 0 {
                let e: Edge = (v, u).into();
                dd.insert(e, dvu as usize);
            }
        }

        self.matrix = matrix;
    }
}
