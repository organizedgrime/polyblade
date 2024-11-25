use super::*;
use crate::render::message::PresetMessage::{self, *};
use std::fs::create_dir_all;
//
// use test_case::test_case;
// #[test_case(Polyhedron::preset(&Pyramid(3)); "T")]
// #[test_case(Polyhedron::preset(&Prism(4)); "C")]
// #[test_case(Polyhedron::preset(&Octahedron); "O")]
// #[test_case(Polyhedron::preset(&Dodecahedron); "D")]
// #[test_case(Polyhedron::preset(&Icosahedron); "I")]
// #[test_case({ let mut g = Polyhedron::preset(&Prism(4)); g.truncate(); g} ; "tC")]
// #[test_case({ let mut g = Polyhedron::preset(&Octahedron); g.truncate(); g} ; "tO")]
// #[test_case({ let mut g = Polyhedron::preset(&Dodecahedron); g.truncate(); g} ; "tD")]
// fn polytope_pst(shape: Polyhedron) {
//     // let mut pst = shape.clone();
//     // shape.shape
//     // pst.distance.pst();
//     // let mut floyd = shape.clone();
//     // floyd.distance.floyd();
//     // assert_eq!(floyd.distance, pst.distance);
// }

#[test]
fn ambo() {
    use PresetMessage::*;
    let prefix = "tests/ambo/";
    create_dir_all(prefix).unwrap();
    let mut polyhedron = Polyhedron::preset(&Pyramid(3));
    polyhedron.shape.png();
    polyhedron.ambo_contract();
    polyhedron.shape.png();
    let octahedron = Polyhedron::preset(&Octahedron);
    octahedron.shape.png();
    assert_eq!(polyhedron.shape, octahedron.shape);
}
