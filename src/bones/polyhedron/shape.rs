use crate::bones::polyhedron::*;

/// Contains all properties that need to be computed iff the structure of the graph changes
#[derive(Default, Debug, Clone)]
pub struct Shape {
    /// Graph, represented as Distance matrix
    pub distance: Distance,
    /// Cycles in the graph
    pub cycles: Cycles,
    /// Faces / chordless cycles
    pub springs: Vec<[VertexId; 2]>,
}

impl Shape {
    pub fn recompute(&mut self) {
        // Find and save cycles
        self.cycles = self.distance.simple_cycles();
        // Update the distance matrix in place
        self.distance.pst();
        // Find and save springs
        self.springs = self.distance.springs();
    }

    pub fn prism(n: usize) -> Distance {
        let mut graph = Distance::new(n * 2);
        //p.name = format!("P{n}");
        for i in 0..n {
            // Lower polygon
            graph.connect([i % n, (i + 1) % n]);
            // Upper polygon
            graph.connect([(i % n) + n, ((i + 1) % n) + n]);
            // Connect
            graph.connect([(i % n), (i % n) + n]);
            graph.connect([(i + 1) % n, ((i + 1) % n) + n]);
        }
        graph
    }
}
