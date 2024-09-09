use super::PolyGraph;

/*
    // TODO: once conway ops are efficient enough- use this.
    T = Y3
    O = aT (ambo tetrahedron)
    C = jT (join tetrahedron)
    I = sT (snub tetrahedron)
    D = gT (gyro tetrahedron)
*/

// Platonic Solids
impl PolyGraph {
    pub fn prism(n: usize) -> PolyGraph {
        let mut p = PolyGraph::new_disconnected(n * 2);
        p.name = format!("P{n}");
        for i in 0..n {
            // Lower polygon
            p.connect((i % n, (i + 1) % n));
            // Upper polygon
            p.connect(((i % n) + n, ((i + 1) % n) + n));
            // Connect
            p.connect(((i % n), (i % n) + n));
            p.connect(((i + 1) % n, ((i + 1) % n) + n));
        }
        p.pst();
        p.find_cycles();
        p.lattice();
        p
    }

    pub fn anti_prism(n: usize) -> PolyGraph {
        let mut p = PolyGraph::new_disconnected(n * 2);
        p.name = format!("A{n}");
        for i in 0..n {
            // Lower polygon
            p.connect((i % n, (i + 1) % n));
            // Upper polygon
            p.connect(((i % n) + n, ((i + 1) % n) + n));
            // Connect
            p.connect(((i % n), (i % n) + n));
            p.connect(((i + 1) % n, ((i + 1) % n) + n));

            p.connect(((i % n), ((i + 1) % n) + n));
        }
        p.pst();
        p.find_cycles();
        p.lattice();
        p
    }

    pub fn pyramid(n: usize) -> PolyGraph {
        let mut p = PolyGraph::new_disconnected(n + 1);
        p.name = format!("Y{n}");
        for i in 0..n {
            p.connect((i, (i + 1) % n));
            p.connect((i, n));
        }
        p.pst();
        p.find_cycles();
        p.lattice();
        p
    }

    pub fn octahedron() -> PolyGraph {
        let mut p = PolyGraph::pyramid(3);
        let edges = p.ambo();
        p.contract_edges(edges);
        p.pst();
        p.lattice();
        p.name = "O".into();
        p
    }
    pub fn dodecahedron() -> PolyGraph {
        let mut p = PolyGraph::anti_prism(5);
        let edges = p.expand(false);
        p.contract_edges(edges);
        p.truncate(Some(5));
        p.pst();
        p.name = "D".into();
        p
    }
    pub fn icosahedron() -> PolyGraph {
        let mut p = PolyGraph::anti_prism(5);
        p.kis(Some(5));
        p.pst();
        p.name = "I".into();
        p
    }
}
