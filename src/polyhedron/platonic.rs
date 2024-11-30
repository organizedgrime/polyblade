use super::*;
use PresetMessage::*;

impl Polyhedron {
    pub fn preset(preset: &PresetMessage) -> Polyhedron {
        use PresetMessage::*;
        let mut poly = match preset {
            Octahedron => Self::octahedron(),
            Dodecahedron => Self::dodecahedron(),
            Icosahedron => Self::icosahedron(),
            _ => {
                let shape = match preset {
                    Prism(n) => Shape::prism(*n),
                    AntiPrism(n) => Shape::anti_prism(*n),
                    Pyramid(n) => Shape::pyramid(*n),
                    _ => todo!(),
                };

                let render = Render::new(shape.order());

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
        let mut polyhedron = Polyhedron::preset(&Pyramid(3));
        polyhedron.ambo_contract();
        polyhedron
    }

    fn dodecahedron() -> Polyhedron {
        let mut polyhedron = Polyhedron::preset(&AntiPrism(5));
        polyhedron.ambo_contract();
        // let edges = polyhedron.truncate(0);
        // polyhedron.contract(edges);
        polyhedron.truncate(5);
        polyhedron
    }

    pub fn icosahedron() -> Polyhedron {
        let mut graph = Polyhedron::preset(&AntiPrism(5));
        graph.shape.kis(Some(5));
        graph.render.new_capacity(graph.shape.order());
        graph
    }
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
