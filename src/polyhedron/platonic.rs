use super::*;

impl Polyhedron {
    pub fn preset(preset: &PresetMessage) -> Polyhedron {
        use PresetMessage::*;
        let mut poly = match preset {
            Octahedron => Self::octahedron(),
            Dodecahedron => Self::dodecahedron(),
            Icosahedron => todo!(),
            _ => {
                let shape = match preset {
                    Prism(n) => Shape::prism(*n),
                    AntiPrism(n) => Shape::anti_prism(*n),
                    Pyramid(n) => Shape::pyramid(*n),
                    _ => todo!(),
                };

                let render = Render::new(shape.len());

                Polyhedron {
                    name: preset.to_string(),
                    shape,
                    render,
                    transactions: vec![],
                }
            }
        };
        poly.shape.compute_graph_svg();
        poly
    }

    fn octahedron() -> Polyhedron {
        let mut polyhedron = Polyhedron::preset(&PresetMessage::Pyramid(3));
        polyhedron.ambo_contract();
        polyhedron
    }

    fn dodecahedron() -> Polyhedron {
        // polyhedron.ambo_contract();
        // let edges = polyhedron.truncate(0);
        //polyhedron.contract(edges);
        // polyhedron.truncate(5);
        Polyhedron::preset(&PresetMessage::AntiPrism(5))
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
