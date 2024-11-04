use crate::render::message::PresetMessage;

use super::*;
/*
    T = Y3
    O = aT (ambo tetrahedron)
    C = jT (join tetrahedron)
    I = sT (snub tetrahedron)
    D = gT (gyro tetrahedron)
*/

// Platonic Solids
impl Distance {
    fn prism(n: usize) -> Distance {
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

    fn anti_prism(n: usize) -> Distance {
        let mut graph = Distance::new(n * 2);
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
        graph
    }

    fn pyramid(n: usize) -> Distance {
        let mut graph = Distance::new(n + 1);
        //graph.name = format!("Y{n}");
        for i in 0..n {
            graph.connect([i, (i + 1) % n]);
            graph.connect([i, n]);
        }
        graph
    }

    fn octahedron() -> Distance {
        let mut p = Distance::pyramid(3);
        let edges = p.ambo();
        p.contract_edges(edges);
        p
    }

    pub fn dodecahedron() -> Distance {
        let mut graph = Distance::anti_prism(5);
        graph.ambod().ambod();
        graph.truncate(Some(5));
        graph
    }

    // pub fn icosahedron() -> Distance {
    //     let mut graph = Distance::anti_prism(5);
    //     graph.kis(Some(5));
    //     graph
    // }

    pub fn icosahedron() -> Distance {
        let dodecahedron = Self::dodecahedron();
        //let edges = dodecahedron.ambo();
        //dodecahedron.contract_edges(edges);
        #[cfg(test)]
        dodecahedron.render("tests/", "icosahedron.svg");
        dodecahedron
    }

    pub fn preset(preset: &PresetMessage) -> Distance {
        use PresetMessage::*;
        match preset {
            Prism(n) => Self::prism(*n),
            AntiPrism(n) => Self::anti_prism(*n),
            Pyramid(n) => Self::pyramid(*n),
            Octahedron => Self::octahedron(),
            Dodecahedron => Self::dodecahedron(),
            Icosahedron => Self::icosahedron(),
        }
    }
}
