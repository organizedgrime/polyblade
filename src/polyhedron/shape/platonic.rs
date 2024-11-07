use super::*;

impl Shape {
    pub fn preset(preset: &PresetMessage) -> Shape {
        use PresetMessage::*;
        let mut preset = match preset {
            Prism(n) => Self::from(Distance::prism(*n)),
            AntiPrism(n) => Self::from(Distance::anti_prism(*n)),
            Pyramid(n) => Self::from(Distance::pyramid(*n)),
            Octahedron => Self::octahedron(),
            Dodecahedron => Self::dodecahedron(),
            // Icosahedron => Self::icosahedron(),
            _ => todo!(),
        };
        preset.compute_graph_svg();
        preset
    }

    fn octahedron() -> Shape {
        let mut p = Shape::from(Distance::pyramid(3));
        let edges = p.ambo();
        p.distance.contract_edges(edges);
        p.recompute();
        p
    }

    fn dodecahedron() -> Shape {
        let mut graph = Shape::from(Distance::anti_prism(5));
        graph.ambod().ambod();
        graph.truncate(Some(5));
        graph
    }

    //
    // pub fn icosahedron() -> Distance {
    //     let mut graph = Distance::anti_prism(5);
    //     graph.kis(Some(5));
    //     graph
    // }
    //
    // pub fn icosahedron() -> Distance {
    //     let dodecahedron = Self::dodecahedron();
    //     //let edges = dodecahedron.ambo();
    //     //dodecahedron.contract_edges(edges);
    //     #[cfg(test)]
    //     dodecahedron.render("tests/", "icosahedron.svg");
    //     dodecahedron
    // }
}
