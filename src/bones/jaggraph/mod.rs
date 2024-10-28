#[cfg(test)]
mod test;
use layout::backends::svg::SVGWriter;
use layout::core::utils::save_to_file;
use layout::gv::{self, GraphBuilder};

use super::{Edge, Face, VertexId};
use std::collections::{HashMap, HashSet};
use std::{
    collections::VecDeque,
    fmt::{Display, Write},
    ops::{Index, IndexMut, Range},
};
mod conway;
mod platonic;

/// Jagged array which represents the symmetrix distance matrix of a given Graph
/// usize::MAX    ->   disconnected
/// 0             ->   identity
/// n             ->   actual distance
#[derive(Debug, Default, Clone, PartialEq)]
pub struct JagGraph {
    matrix: Vec<Vec<usize>>,
    pub cycles: Vec<Face>,
}

impl JagGraph {
    /// [ 0 ]
    /// [ M | 0 ]
    /// [ M | M | 0 ]
    /// ..
    /// [ M | M | M | ... | M | M | M | 0 ]
    pub fn new(n: usize) -> Self {
        JagGraph {
            matrix: (0..n)
                .into_iter()
                .map(|m| [vec![usize::MAX; m], vec![0]].concat())
                .collect(),
            cycles: vec![],
        }
    }
}

impl JagGraph {
    /// Connect one vertex to another with length one, iff they are note the same point
    pub fn connect<T>(&mut self, i: T)
    where
        JagGraph: Index<T, Output = usize> + IndexMut<T, Output = usize>,
        T: Copy,
    {
        if self[i] != 0 {
            self[i] = 1;
        }
    }

    /// Disconnect one vertex from another iff they are neighbors
    pub fn disconnect<T>(&mut self, i: T)
    where
        JagGraph: Index<T, Output = usize> + IndexMut<T, Output = usize>,
        T: Copy,
    {
        if self[i] == 1 {
            self[i] = usize::MAX;
        }
    }

    /// Inserts a new vertex in the matrix
    pub fn insert(&mut self) -> VertexId {
        self.matrix
            .push([vec![usize::MAX; self.len()], vec![0]].concat());
        self.len() - 1
    }

    /// Deletes a vertex from the matrix
    pub fn delete(&mut self, v: VertexId) {
        for row in &mut self.matrix[v..] {
            row.remove(v);
        }
        self.matrix.remove(v);

        for cycle in &mut self.cycles {
            *cycle = cycle.iter().filter(|&c| c != &v).cloned().collect();
        }
        for i in 0..self.cycles.len() {
            for j in 0..self.cycles[i].len() {
                if self.cycles[i][j] >= v {
                    self.cycles[i][j] -= 1;
                }
            }
        }
    }

    /// Enumerates the vertices connected to v
    pub fn connections(&self, v: VertexId) -> Vec<VertexId> {
        self.vertices().filter(|&u| self[[v, u]] == 1).collect()
    }

    /// Iterable Range representing vertex IDs
    pub fn vertices(&self) -> Range<VertexId> {
        0..self.matrix.len()
    }

    /// All possible compbinations of vertices
    pub fn vertex_pairs(&self) -> impl Iterator<Item = [VertexId; 2]> {
        self.vertices()
            .flat_map(|v| (0..v).into_iter().map(move |u| [v, u]))
    }

    /// All actual edges of the graph (D_{ij} = 1)
    pub fn edges(&self) -> impl Iterator<Item = [VertexId; 2]> + use<'_> {
        self.vertex_pairs().filter(move |&e| self[e] == 1)
    }

    /// Vertex Count
    pub fn len(&self) -> usize {
        self.matrix.len()
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
        let mut dist: JagGraph = JagGraph::new(self.len());

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
                            let e: Edge = (x, v).into();
                            if x != v && dist[e] == usize::MAX {
                                // D[x.id, v.id] = d;
                                dist[e] = depth;
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

    /// Number of faces
    pub fn face_count(&self) -> i64 {
        2 + self.edges().count() as i64 - self.len() as i64
    }

    /// All faces
    pub fn find_cycles(&mut self) {
        let mut triplets = Vec::<Face>::new();
        let mut cycles = HashSet::<Face>::default();

        // find all the triplets
        for u in self.vertices() {
            let adj: Vec<VertexId> = self.connections(u);
            for &x in adj.iter() {
                for &y in adj.iter() {
                    if x != y && u < x && x < y {
                        let new_face = Face::new(vec![x, u, y]);
                        if self[[x, y]] == 1 {
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
            for v in self.connections(p[p.len() - 1]) {
                if v > p[1] {
                    let c = self.connections(v);
                    // if v is not a neighbor of u_2..u_t-1
                    if !p[1..p.len() - 1].iter().any(|vi| c.contains(vi)) {
                        let mut new_face = p.clone();
                        new_face.push(v);
                        if self.connections(p[0]).contains(&v) {
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
}

impl Index<[VertexId; 2]> for JagGraph {
    type Output = usize;

    fn index(&self, index: [VertexId; 2]) -> &Self::Output {
        &self.matrix[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl IndexMut<[VertexId; 2]> for JagGraph {
    fn index_mut(&mut self, index: [VertexId; 2]) -> &mut Self::Output {
        &mut self.matrix[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl Index<Edge> for JagGraph {
    type Output = usize;

    fn index(&self, index: Edge) -> &Self::Output {
        &self.matrix[index.v.max(index.u)][index.v.min(index.u)]
    }
}

impl IndexMut<Edge> for JagGraph {
    fn index_mut(&mut self, index: Edge) -> &mut Self::Output {
        &mut self.matrix[index.v.max(index.u)][index.v.min(index.u)]
    }
}

impl Display for JagGraph {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for v in self.vertices() {
            f.write_str("|")?;
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

impl JagGraph {
    pub fn graphviz(&self) -> String {
        let mut dot = format!("graph G{{\nlayout=fdp\n");

        let colors = vec!["red", "green", "blue"];

        for v in self.vertices() {
            dot.push_str(&format!(
                "\t{v} [color=\"{}\"];\n",
                colors[self.connections(v).len() % colors.len()]
            ));
        }

        for [u, v] in self.edges() {
            dot.push_str(&format!("\t{u} -- {v};\n"));
        }
        dot.push_str("}");
        dot
    }

    pub fn render(&self, filename: &str) {
        let mut parser = gv::DotParser::new(&self.graphviz());
        let tree = parser.process();
        match tree {
            Err(err) => {
                parser.print_error();
                log::error!("Error: {}", err);
            }
            Ok(g) => {
                // if dump_ast {
                // }
                //gv::dump_ast(&g);
                let mut gb = GraphBuilder::new();
                gb.visit_graph(&g);
                let mut vg = gb.get();
                let mut svg = SVGWriter::new();
                vg.do_it(false, false, true, &mut svg);
                let content = svg.finalize();

                let res = save_to_file(filename, &content);
                if let Result::Err(err) = res {
                    log::error!("Could not write the file {filename}");
                    log::error!("Error {}", err);
                    return;
                }
                log::info!("Wrote {filename}");
            }
        }
    }
}
// impl Display for JagGraph {
//     fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
//         f.write_str("graph G {\nlayout=fdp\n")?;
//         for [u, v] in self.edges() {
//             f.write_fmt(format_args!("\t{u} -- {v};\n"))?;
//         }
//         f.write_str("}")
//     }
// }
