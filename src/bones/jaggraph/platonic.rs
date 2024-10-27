use super::*;
/*
    T = Y3
    O = aT (ambo tetrahedron)
    C = jT (join tetrahedron)
    I = sT (snub tetrahedron)
    D = gT (gyro tetrahedron)
*/

// Platonic Solids
impl JagGraph {
    pub fn prism(n: usize) -> JagGraph {
        let mut graph = JagGraph::new(n * 2);
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
        graph.pst();
        graph.find_cycles();
        graph
    }

    pub fn anti_prism(n: usize) -> JagGraph {
        let mut graph = JagGraph::new(n * 2);
        //p.name = format!("A{n}");
        for i in 0..n {
            // Lower polygon
            graph.connect([i % n, (i + 1) % n]);
            // Upper polygon
            graph.connect([(i % n) + n, ((i + 1) % n) + n]);
            // Connect
            graph.connect([(i % n), (i % n) + n]);
            graph.connect([(i + 1) % n, ((i + 1) % n) + n]);

            graph.connect([(i % n), ((i + 1) % n) + n]);
        }
        graph.pst();
        graph.find_cycles();
        graph
    }

    pub fn pyramid(n: usize) -> JagGraph {
        let mut graph = JagGraph::new(n + 1);
        //graph.name = format!("Y{n}");
        for i in 0..n {
            graph.connect([i, (i + 1) % n]);
            graph.connect([i, n]);
        }
        graph.pst();
        // p.springs();
        graph.find_cycles();
        // graph.lattice();
        graph
    }

    pub fn octahedron() -> JagGraph {
        let mut p = JagGraph::pyramid(3);
        let edges = p.ambo();
        p.contract_edges(edges);
        p.pst();
        //p.springs();
        //p.lattice();
        //p.name = "O".into();
        p
    }

    // pub fn dodecahedron() -> PolyGraph {
    //     let mut p = PolyGraph::anti_prism(5);
    //     let edges = p.expand(false);
    //     p.contract_edges(edges);
    //     p.truncate(Some(5));
    //     //p.pst();
    //     //p.springs();
    //     p.name = "D".into();
    //     p
    // }
    // pub fn icosahedron() -> PolyGraph {
    //     let mut p = PolyGraph::anti_prism(5);
    //     p.kis(Some(5));
    //     //p.pst();
    //     //p.springs();
    //     p.name = "I".into();
    //     p
    // }
}
