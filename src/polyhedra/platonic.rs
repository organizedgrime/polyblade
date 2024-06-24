use crate::{ConwayMessage, PresetMessage};

use super::{PolyGraph, Transaction};

/*
    // TODO: once conway ops are efficient enough- use this.
    T = Y3
    O = aT (ambo tetrahedron)
    C = jT (join tetrahedron)
    I = sT (snub tetrahedron)
    D = gT (gyro tetrahedron)
*/

impl PolyGraph {
    pub fn change_shape(&mut self, message: PresetMessage) {
        use PresetMessage::*;
        match message {
            Prism(n) => *self = PolyGraph::prism(n),
            AntiPrism(n) => *self = PolyGraph::anti_prism(n),
            Pyramid(n) => *self = PolyGraph::pyramid(n),
            Octahedron => *self = PolyGraph::octahedron(),
            Dodecahedron => *self = PolyGraph::dodecahedron(),
            Icosahedron => *self = PolyGraph::icosahedron(),
        }
    }
}

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
        let mut y = PolyGraph::pyramid(3);
        y.ambo();
        y.pst();
        y.lattice();
        y
    }
    pub fn dodecahedron() -> PolyGraph {
        let mut p = PolyGraph::anti_prism(5);
        p.dual();
        p.pst();
        p.truncate(Some(5));
        p.pst();
        p
    }
    pub fn icosahedron() -> PolyGraph {
        let mut p = PolyGraph::anti_prism(5);
        p.kis(Some(5));
        p.pst();
        p
    }
}
