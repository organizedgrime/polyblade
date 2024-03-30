use cgmath::Zero;

pub use super::*;

impl Graph {
    pub fn contract_edge(&mut self, id: EdgeId) {
        // This operation requires up to date edges
        self.update();

        //println
        let c0 = self.ghosts(id.0);
        let c1 = self.ghosts(id.1);

        println!("c0: {:?}, c1: {:?}", c0, c1);

        //let all_edges = self.adjacents;
        for x in c0.iter() {
            for y in c1.iter() {
                let true_edge = (*x, *y).into();
                if self.adjacents.contains(&true_edge) {
                    println!("FOUND A TRUE EDGE!");
                    // Give b all the same connections as a
                    let adj = self.connections(true_edge.id().0).clone();
                    for b in adj.into_iter() {
                        self.connect((b, true_edge.id().1))
                    }
                    // Delete a
                    self.delete(true_edge.id().0);
                    break;
                }
            }
        }
    }

    /*
    pub fn split_vertex(&mut self, v: VertexId) {
        //-> Face {
        let mut new_face = vec![];
        let mut danglers = vec![];

        // Remove all connections
        for u in self.connections(v) {
            // Remove existing connection
            self.disconnect((v, u));
            // Track the free hangers
            danglers.push(u);
        }

        //danglers.sort();

        for d in danglers {
            // Create new node and connect to dangler
            let n = self.insert(Some(v)).id();
            self.connect((d, n));
            new_face.push(n);
        }

        // Link all the
        for i in 0..new_face.len() {
            self.connect((new_face[i], new_face[(i + 1) % new_face.len()]));
        }

        //self.delete(v);

        //Face(new_face.into_iter().collect())
    }
    */

    // /*
    pub fn split_vertex(&mut self, id: VertexId) {
        let connections: Vec<VertexId> = self.connections(id).into_iter().collect();

        // Add the connections to the ghost matrix
        if !self.ghost_matrix.contains_key(&id) {
            self.ghost_matrix.insert(id, HashSet::new());
        }
        for c in &connections {
            println!("removing {id}, adding {c} to ghost");
            self.ghost_matrix.get_mut(&id).unwrap().insert(*c);
        }

        //self.delete(id);
        let mut new_face = Vec::new();

        for v2 in &connections {
            // Insert a new node in the same location
            let new_vertex = self.insert(Some(Vector3::zero()));
            //
            new_face.push(new_vertex);
            // Reform old connection
            self.connect((*v2, new_vertex.id()));
        }

        // Link all the
        for i in 0..new_face.len() {
            self.connect((new_face[i], new_face[(i + 1) % new_face.len()]));
        }

        self.delete(id);
    }
    //*/
    /// `t` truncate is equivalent to vertex splitting
    pub fn truncate(&mut self) {
        for vertex in self.vertices() {
            self.split_vertex(vertex.id());
        }
    }

    /// `a` ambo is equivalent to the composition of vertex splitting and edge contraction vefore
    /// applying vertex splitting.
    pub fn ambo(&mut self) {
        let original_edges = self.adjacents.clone();
        for vertex in self.vertices() {
            self.update();
            self.split_vertex(vertex.id());
        }

        for edge in original_edges.iter() {
            self.contract_edge(edge.id());
        }
        println!("edges: {:?}", self.adjacents);
        println!("verts: {:?}", self.vertices());
    }

    //
    //fn dual(&mut self) {}
    /// `b` bevel is equivalent to `ta`
    fn bevel(&mut self) {
        self.truncate();
        self.ambo();
    }

    /// `e` expand is equal to `aa`
    fn expand(&mut self) {
        self.ambo();
        self.ambo();
    }

    /// `s` snub is applying `e` followed by diagonal addition
    fn snub(&mut self) {
        self.expand();
        //self.diagonal_addition();
    }
}

#[cfg(test)]
mod test {
    use crate::prelude::*;

    #[test]
    fn poly() {
        let mut dodeca = Polyhedron::icosahedron();
        dodeca.graph.contract_edge((0, 1));
    }

    #[test]
    fn contract_edge() {
        let mut graph = Graph::new_disconnected(6);
        graph.connect((1, 0));
        graph.connect((1, 2));

        graph.connect((1, 3));

        graph.connect((3, 4));
        graph.connect((3, 5));
        graph.update();

        assert_eq!(graph.vertices().len(), 6);
        assert_eq!(graph.adjacents.len(), 5);

        graph.contract_edge((1, 3));
        graph.update();

        println!("g: {:?}", graph);
        assert_eq!(graph.vertices().len(), 5);
        assert_eq!(graph.adjacents.len(), 4);

        assert_eq!(graph.connections(0), vec![3].into_iter().collect());
        assert_eq!(graph.connections(2), vec![3].into_iter().collect());

        assert_eq!(graph.connections(3), vec![0, 2, 4, 5].into_iter().collect());

        assert_eq!(graph.connections(4), vec![3].into_iter().collect());
        assert_eq!(graph.connections(5), vec![3].into_iter().collect());
    }

    #[test]
    fn split_vertex() {
        let mut graph = Graph::new_disconnected(5);
        graph.connect((1, 0));
        graph.connect((1, 2));

        graph.connect((1, 3));
        graph.connect((1, 4));
        graph.update();

        assert_eq!(graph.vertices().len(), 5);
        assert_eq!(graph.adjacents.len(), 4);

        graph.split_vertex(1);
        graph.update();

        println!("g: {:?}", graph);
        assert_eq!(graph.vertices().len(), 8);
        assert_eq!(graph.adjacents.len(), 8);
    }
}
