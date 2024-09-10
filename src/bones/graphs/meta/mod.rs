mod face;
pub use face::*;

use polyblade_derive::SimpleGraph;
use std::collections::{HashMap, HashSet, VecDeque};

use crate::bones::graphs::simple::*;

/// The MetaGraph stores a SimpleGraph and its derived properties
///
#[derive(SimpleGraph)]
pub struct Meta {
    #[internal]
    pub simple: Simple,
    pub cycles: Vec<Face>,
    pub dist: HashMap<Edge, usize>,
}

impl Meta {
    pub fn find_cycles(&mut self) {
        let mut triplets = Vec::<Face>::new();
        let mut cycles = HashSet::<Face>::new();

        // find all the triplets
        for &u in self.vertices() {
            let adj: HashSet<VertexId> = self.vertex_connections(u);
            for &x in adj.iter() {
                for &y in adj.iter() {
                    if x != y && u < x && x < y {
                        let new_face = Face::new(vec![x, u, y]);
                        let edge: Edge = (&x, &y).into();
                        let edges = self.edges().cloned().collect::<Vec<Edge>>();
                        if edges.contains(&edge) {
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
            let p = triplets.remove(0);
            // for each v adjacent to u_t
            for v in self.vertex_connections(p[p.len() - 1]).into_iter() {
                if v > p[1] {
                    let c = self.vertex_connections(v);
                    // if v is not a neighbor of u_2..u_t-1
                    if !p[1..p.len() - 1].iter().any(|vi| c.contains(vi)) {
                        let mut new_face = p.clone();
                        new_face.push(v);
                        if self.vertex_connections(p[0]).contains(&v) {
                            //cycles.remo
                            cycles.insert(new_face);
                        } else {
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }

        self.cycles = cycles.into_iter().collect();
    }

    pub fn pst(&mut self) {
        if self.edge_count() == 0 {
            return;
        }

        let n = self.vertex_count();
        // Vertex
        //
        // d-queues associated w each vertex
        // maps from v -> ( maps from d -> u )
        let mut dqueue: HashMap<VertexId, VecDeque<(VertexId, usize)>> = Default::default();
        //
        let mut children: HashMap<VertexId, Vec<VertexId>> = Default::default();

        // Counters for vertices whos shortest paths have already been obtained
        let mut counters: HashMap<VertexId, usize> = self.vertices().map(|v| (*v, n - 1)).collect();

        // The element D[i, j] represents the distance from v_i to vj.
        let mut dist: HashMap<Edge, usize> = Default::default();

        // d = 0
        let mut depth = 1;
        // while 0 < |V|
        loop {
            let verts: HashSet<VertexId> = counters
                .iter()
                .filter_map(|(v, c)| if *c == 0 { None } else { Some(*v) })
                .collect();
            if verts.is_empty() {
                break;
            }

            let mut removed = false;

            for v in verts.into_iter() {
                // for v in V
                // START EXTEND(v, d, D, S)
                if depth == 1 {
                    //
                    for e in self.edge_connections(v).into_iter() {
                        // Connected node
                        let w = e.other(v).unwrap();
                        // D[w.id, v.id] = d
                        dist.insert(e, 1);
                        // add w' to v'.children
                        children.entry(v).or_default().push(w);
                        // v.que.enque(w', 1)
                        dqueue.entry(v).or_default().push_back((w, 1));
                        dqueue.entry(w).or_default().push_back((v, 1));
                        // v.c = v.c + 1
                        *counters.get_mut(&v).unwrap() -= 1;
                        //*counters.get_mut(&w).unwrap() -= 1;
                        removed = true;
                    }
                } else {
                    // w = v.que.deque(d - 1)
                    // while w is not None:
                    'dq: loop {
                        let vqueue = dqueue.get_mut(&v).unwrap();
                        if let Some((w, d)) = vqueue.pop_front() {
                            if d != depth - 1 {
                                dqueue.get_mut(&v).unwrap().push_back((w, d));
                                break;
                            }
                            // for x in w.children
                            for x in children.get(&w).unwrap().clone() {
                                let e: Edge = (x, v).into();
                                if x != v && !dist.contains_key(&e) {
                                    // D[x.id, v.id] = d;
                                    dist.insert(e, depth);
                                    // add x' to w' children
                                    children.entry(w).or_default().push(x);
                                    // v.que.enque(x', d)
                                    dqueue.get_mut(&v).unwrap().push_back((x, depth));
                                    dqueue.get_mut(&x).unwrap().push_back((v, depth));
                                    // v.c = v.c + 1
                                    removed = true;
                                    *counters.get_mut(&v).unwrap() -= 1;
                                    *counters.get_mut(&x).unwrap() -= 1;
                                    // if v.c == n: return
                                    if *counters.get(&x).unwrap() == 0
                                        && *counters.get(&w).unwrap() == 0
                                        && *counters.get(&v).unwrap() == 0
                                    {
                                        break 'dq;
                                    }
                                }
                            }
                        } else {
                            break;
                        }
                    }
                }
            }
            // END EXTEND
            // d = d + 1
            depth += 1;

            if !removed {
                self.dist = dist;
                println!("failed distance computation");
                return;
            }
        }

        self.dist = dist;
    }
}
