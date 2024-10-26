#[cfg(test)]
mod test;
use super::{Edge, Face, VertexId};
use std::collections::{HashMap, HashSet};
use std::{
    collections::VecDeque,
    fmt::{Display, Write},
    ops::{Index, IndexMut, Range},
};

/// Jagged array which represents the symmetrix distance matrix of a given Graph
/// usize::MAX    ->   disconnected
/// 0             ->   identity
/// n             ->   actual distance
#[derive(Debug, Default, Clone, PartialEq)]
pub struct JagGraph {
    matrix: Vec<Vec<usize>>,
    cycles: Vec<Face>,
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
            .push([vec![usize::MAX; self.len() - 1], vec![0]].concat());
        self.len()
    }

    /// Deletes a vertex from the matrix
    pub fn delete(&mut self, v: VertexId) {
        for row in &mut self.matrix[v..] {
            row.remove(v);
            for distance in &mut row[v..] {
                if *distance > 0 && *distance != usize::MAX {
                    *distance -= 1;
                }
            }
        }
        self.matrix.remove(v);
    }

    /// Enumerates the vertices connected to v
    pub fn connections(&self, v: VertexId) -> Vec<VertexId> {
        self.vertices().filter(|&u| self[[v, u]] == 1).collect()
    }

    /// Iterable Range representing vertex IDs
    pub fn vertices(&self) -> Range<VertexId> {
        0..self.matrix.len()
    }

    /// Vertex Count
    pub fn len(&self) -> usize {
        self.matrix.len()
    }

    /// Maximum distance value
    pub fn diameter(&self) -> usize {
        self.vertices()
            .zip(self.vertices())
            .map(|(v, u)| self[[v, u]])
            .max()
            .unwrap_or(0)
    }

    /* 1  procedure BFS(G, root) is
     2      let Q be a queue
     3      label root as explored
     4      Q.enqueue(root)
     5      while Q is not empty do
     6          v := Q.dequeue()
     7          if v is the goal then
     8              return v
     9          for all edges from v to w in G.adjacentEdges(v) do
    10              if w is not labeled as explored then
    11                  label w as explored
    12                  w.parent := v
    13                  Q.enqueue(w)
        */
    /*
         * private Map<Node, Boolean>> vis = new HashMap<Node, Boolean>();

    private Map<Node, Node> prev = new HashMap<Node, Node>();

    public List getDirections(Node start, Node finish){
        List directions = new LinkedList();
        Queue q = new LinkedList();
        Node current = start;
        q.add(current);
        vis.put(current, true);
        while(!q.isEmpty()){
            current = q.remove();
            if (current.equals(finish)){
                break;
            }else{
                for(Node node : current.getOutNodes()){
                    if(!vis.contains(node)){
                        q.add(node);
                        vis.put(node, true);
                        prev.put(node, current);
                    }
                }
            }
        }
        if (!current.equals(finish)){
            System.out.println("can't reach destination");
        }
        for(Node node = finish; node != null; node = prev.get(node)) {
            directions.add(node);
        }
        directions.reverse();
        return directions;
    }
         */

    /*
    fn bfs(&mut self, start: VertexId, end: VertexId) {
        let goal = 12;
        let mut explored = vec![start];
        let mut q = VecDeque::from([start]);
        while let Some(v) = q.pop_front() {
            if v == end {
                break;
                //return v;
            }
            for w in self.connections(v) {
                if !explored.contains(&w) {
                    q.push_back(w);
                    explored.push(w);
                    //prev.put()
                }
            }
        }
    }
    */

    pub fn pst(&mut self) {
        // if self.edges.is_empty() {
        //     return;
        // }

        let n = self.len();
        // Vertex
        //
        // d-queues associated w each vertex
        // maps from v -> ( maps from d -> u )
        let mut dqueue: HashMap<VertexId, VecDeque<(VertexId, usize)>> = Default::default();
        //
        let mut children: HashMap<VertexId, Vec<VertexId>> = Default::default();

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
                        children.entry(v).or_default().push(w);
                        // v.que.enque(w', 1)
                        dqueue.entry(v).or_default().push_back((w, 1));
                        dqueue.entry(w).or_default().push_back((v, 1));
                        // v.c = v.c + 1
                        counters[v] -= 1;
                        //*counters.entry(w).or_default() -= 1;
                        removed = true;
                    }
                } else {
                    // w = v.que.deque(d - 1)
                    // while w is not None:
                    'dq: loop {
                        let vqueue = dqueue.entry(v).or_default();
                        if let Some((w, d)) = vqueue.pop_front() {
                            if d != depth - 1 {
                                dqueue.entry(v).or_default().push_back((w, d));
                                break;
                            }
                            // for x in w.children
                            for x in children.entry(w).or_default().clone() {
                                let e: Edge = (x, v).into();
                                if x != v && dist[e] == usize::MAX {
                                    // D[x.id, v.id] = d;
                                    dist[e] = depth;
                                    // add x' to w' children
                                    children.entry(w).or_default().push(x);
                                    // v.que.enque(x', d)
                                    dqueue.entry(v).or_default().push_back((x, depth));
                                    dqueue.entry(x).or_default().push_back((v, depth));
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
                *self = dist;
                log::error!("failed distance computation");
                return;
            }
        }

        *self = dist;
    }

    /// Number of faces
    pub fn face_count(&self) -> i64 {
        2 + self
            .vertices()
            .zip(self.vertices())
            .filter(|&(v, u)| self[[v, u]] == 1)
            .count() as i64
            - self.len() as i64
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
                    String::from("M")
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
