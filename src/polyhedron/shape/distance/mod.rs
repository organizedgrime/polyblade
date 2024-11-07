mod conway;
mod platonic;
mod svg;
pub use conway::*;
pub use platonic::*;
pub use svg::*;

#[cfg(test)]
mod test;

use crate::polyhedron::VertexId;
use std::collections::HashSet;
use std::{
    collections::VecDeque,
    fmt::Display,
    ops::{Index, IndexMut, Range},
};

/// Jagged array which represents the symmetrix distance matrix of a given Graph
/// usize::MAX    ->   disconnected
/// 0             ->   identity
/// n             ->   actual distance
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Distance {
    distance: Vec<Vec<usize>>,
}

impl Distance {
    /// [ 0 ]
    /// [ M | 0 ]
    /// [ M | M | 0 ]
    /// ..
    /// [ M | M | M | ... | M | M | M | 0 ]
    pub fn new(n: usize) -> Self {
        Distance {
            distance: (0..n)
                .map(|m| [vec![usize::MAX; m], vec![0]].concat())
                .collect(),
        }
    }
}

impl Distance {
    /// Connect one vertex to another with length one, iff they are note the same point
    pub fn connect(&mut self, [v, u]: [VertexId; 2]) {
        if self[[v, u]] != 0 {
            self[[v, u]] = 1;
        }
    }

    /// Disconnect one vertex from another iff they are neighbors
    pub fn disconnect(&mut self, [v, u]: [VertexId; 2]) {
        if self[[v, u]] == 1 {
            self[[v, u]] = usize::MAX;
        }
    }

    /// Inserts a new vertex in the matrix
    pub fn insert(&mut self) -> VertexId {
        self.distance
            .push([vec![usize::MAX; self.len()], vec![0]].concat());
        self.len() - 1
    }

    /// Deletes a vertex from the matrix
    pub fn delete(&mut self, v: VertexId) {
        for row in &mut self.distance[v..] {
            row.remove(v);
        }
        self.distance.remove(v);
    }

    /// Enumerates the vertices connected to v
    pub fn connections(&self, v: VertexId) -> Vec<VertexId> {
        self.vertices().filter(|&u| self[[v, u]] == 1).collect()
    }

    /// Iterable Range representing vertex IDs
    pub fn vertices(&self) -> Range<VertexId> {
        0..self.distance.len()
    }

    /// All possible compbinations of vertices
    pub fn vertex_pairs(&self) -> impl Iterator<Item = [VertexId; 2]> {
        self.vertices().flat_map(|v| (0..v).map(move |u| [v, u]))
    }

    /// All actual edges of the graph (D_{ij} = 1)
    pub fn edges(&self) -> impl Iterator<Item = [VertexId; 2]> + use<'_> {
        self.vertex_pairs().filter(move |&e| self[e] == 1)
    }

    /// Vertex Count
    pub fn len(&self) -> usize {
        self.distance.len()
    }

    /// Maximum distance value
    pub fn diameter(&self) -> usize {
        self.vertex_pairs().map(|e| self[e]).max().unwrap_or(0)
    }

    pub fn pst(&mut self) {
        // if self.edges.is_empty() {
        //     return;
        // }

        let n = self.len();
        // Vertex
        //
        // d-queues associated w each vertex
        // maps from v -> ( maps from d -> u )
        let mut dqueue: Vec<VecDeque<(VertexId, usize)>> = vec![Default::default(); self.len()];
        //
        let mut children: Vec<Vec<VertexId>> = vec![Default::default(); self.len()];

        // Counters for vertices whos shortest paths have already been obtained
        let mut counters: Vec<usize> = vec![n - 1; self.len()];

        // The element D[i, j] represents the distance from v_i to vj.
        let mut dist: Distance = Distance::new(self.len());

        // d = 0
        let mut depth = 1;
        // while 0 < |V|
        loop {
            let verts: HashSet<VertexId> = counters
                .iter()
                .enumerate()
                .filter_map(|(v, c)| if *c == 0 { None } else { Some(v) })
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
                    for w in self.connections(v) {
                        // Connected node
                        // D[w.id, v.id] = d
                        dist[[v, w]] = 1;
                        // add w' to v'.children
                        children[v].push(w);
                        // v.que.enque(w', 1)
                        dqueue[v].push_back((w, 1));
                        dqueue[w].push_back((v, 1));
                        // v.c = v.c + 1
                        counters[v] -= 1;
                        removed = true;
                    }
                } else {
                    // w = v.que.deque(d - 1)
                    // while w is not None:
                    'dq: loop {
                        // let vqueue = dqueue[v];
                        let Some((w, d)) = dqueue[v].pop_front() else {
                            break;
                        };
                        if d != depth - 1 {
                            dqueue[v].push_back((w, d));
                            break;
                        }
                        // for x in w.children
                        for x in children[w].clone() {
                            //let e: Edge = (x, v).into();
                            if x != v && dist[[x, v]] == usize::MAX {
                                // D[x.id, v.id] = d;
                                dist[[x, v]] = depth;
                                // add x' to w' children
                                children[w].push(x);
                                // v.que.enque(x', d)
                                dqueue[v].push_back((x, depth));
                                dqueue[x].push_back((v, depth));
                                // v.c = v.c + 1
                                removed = true;
                                counters[v] -= 1;
                                counters[x] -= 1;
                                // if v.c == n: return
                                if counters[x] == 0 && counters[w] == 0 && counters[v] == 0 {
                                    break 'dq;
                                }
                            }
                        }
                    }
                }
            }
            // END EXTEND
            // d = d + 1
            depth += 1;

            if !removed {
                *self = dist;
                log::error!("failed distance computation");
                return;
            }
        }

        *self = dist;
    }

    pub fn springs(&self) -> Vec<[VertexId; 2]> {
        let diameter = self.diameter();
        self.vertex_pairs()
            .filter(|&[v, u]| v != u && (self[[v, u]] <= 2 || self[[v, u]] >= diameter - 1))
            .collect::<Vec<_>>()
    }

    /// Number of faces
    pub fn face_count(&self) -> i64 {
        2 + self.edges().count() as i64 - self.len() as i64
    }
}

impl Index<[VertexId; 2]> for Distance {
    type Output = usize;

    fn index(&self, index: [VertexId; 2]) -> &Self::Output {
        &self.distance[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl IndexMut<[VertexId; 2]> for Distance {
    fn index_mut(&mut self, index: [VertexId; 2]) -> &mut Self::Output {
        &mut self.distance[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl Display for Distance {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("\t|"))?;
        for i in 0..self.len() {
            f.write_fmt(format_args!(" {i} |"))?;
        }
        f.write_fmt(format_args!("\n\t"))?;
        for _ in 0..self.len() {
            f.write_fmt(format_args!("____"))?;
        }
        f.write_fmt(format_args!("\n"))?;
        for v in self.vertices() {
            f.write_fmt(format_args!("{v}:\t|"))?;
            for u in self.vertices() {
                let value = if self[[v, u]] == usize::MAX {
                    String::from("_")
                } else {
                    self[[v, u]].to_string()
                };
                f.write_fmt(format_args!(" {value} |"))?;
            }
            f.write_fmt(format_args!("\n"))?;
        }
        Ok(())
    }
}
