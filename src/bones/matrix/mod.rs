#[cfg(test)]
mod test;
use super::{Edge, VertexId};
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
pub struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    /// [ 0 ]
    /// [ M | 0 ]
    /// [ M | M | 0 ]
    /// ..
    /// [ M | M | M | ... | M | M | M | 0 ]
    pub fn new(n: usize) -> Self {
        Matrix(
            (0..n)
                .into_iter()
                .map(|m| [vec![usize::MAX; m], vec![0]].concat())
                .collect(),
        )
    }
}

impl Matrix {
    /// Connect one vertex to another with length one, iff they are note the same point
    pub fn connect<T>(&mut self, i: T)
    where
        Matrix: Index<T, Output = usize> + IndexMut<T, Output = usize>,
        T: Copy,
    {
        if self[i] != 0 {
            self[i] = 1;
        }
    }

    /// Disconnect one vertex from another iff they are neighbors
    pub fn disconnect<T>(&mut self, i: T)
    where
        Matrix: Index<T, Output = usize> + IndexMut<T, Output = usize>,
        T: Copy,
    {
        if self[i] == 1 {
            self[i] = usize::MAX;
        }
    }

    /// Inserts a new vertex in the matrix
    pub fn insert(&mut self) -> VertexId {
        self.0
            .push([vec![usize::MAX; self.0.len() - 1], vec![0]].concat());
        self.0.len()
    }

    /// Deletes a vertex from the matrix
    pub fn delete(&mut self, v: VertexId) {
        for row in &mut self.0[v..] {
            row.remove(v);
            for distance in &mut row[v..] {
                if *distance > 0 && *distance != usize::MAX {
                    *distance -= 1;
                }
            }
        }
        self.0.remove(v);
    }

    /// Enumerates the vertices connected to v
    pub fn connections(&self, v: VertexId) -> Vec<VertexId> {
        self.vertices().filter(|&u| self[[v, u]] == 1).collect()
    }

    /// Iterable Range representing vertex IDs
    pub fn vertices(&self) -> Range<VertexId> {
        0..self.0.len()
    }

    /// Vertex Count
    pub fn len(&self) -> usize {
        self.0.len()
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
        use std::collections::{HashMap, HashSet};

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
        let mut dist: Matrix = Matrix::new(self.len());

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
}

impl Index<[VertexId; 2]> for Matrix {
    type Output = usize;

    fn index(&self, index: [VertexId; 2]) -> &Self::Output {
        &self.0[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl IndexMut<[VertexId; 2]> for Matrix {
    fn index_mut(&mut self, index: [VertexId; 2]) -> &mut Self::Output {
        &mut self.0[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl Index<Edge> for Matrix {
    type Output = usize;

    fn index(&self, index: Edge) -> &Self::Output {
        &self.0[index.v.max(index.u)][index.v.min(index.u)]
    }
}

impl IndexMut<Edge> for Matrix {
    fn index_mut(&mut self, index: Edge) -> &mut Self::Output {
        &mut self.0[index.v.max(index.u)][index.v.min(index.u)]
    }
}

impl Display for Matrix {
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
