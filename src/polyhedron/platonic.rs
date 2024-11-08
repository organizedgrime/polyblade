use super::*;

impl Polyhedron {
    pub fn preset(preset: &PresetMessage) -> Polyhedron {
        use PresetMessage::*;
        match preset {
            Octahedron => Self::octahedron(),
            Dodecahedron => Self::dodecahedron(),
            Icosahedron => todo!(),
            _ => {
                let mut shape = match preset {
                    Prism(n) => Shape::prism(*n),
                    AntiPrism(n) => Shape::anti_prism(*n),
                    Pyramid(n) => Shape::pyramid(*n),
                    // Octahedron => Self::octahedron(),
                    // Dodecahedron => Self::dodecahedron(),
                    // Icosahedron => Self::icosahedron(),
                    _ => todo!(),
                };

                shape.compute_graph_svg();
                let render = Render::new(shape.len());

                Polyhedron {
                    name: preset.to_string(),
                    shape,
                    render,
                    transactions: vec![],
                }
            }
        }
    }

    fn octahedron() -> Polyhedron {
        let mut polyhedron = Polyhedron::preset(&PresetMessage::Pyramid(3));
        polyhedron.ambo_contract();
        polyhedron
    }

    fn dodecahedron() -> Polyhedron {
        let mut polyhedron = Polyhedron::preset(&PresetMessage::AntiPrism(5));
        polyhedron.ambo_contract();
        polyhedron.ambo_contract();
        // polyhedron.truncate(Some(5));
        polyhedron
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
