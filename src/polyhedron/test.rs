use super::*;
use crate::render::message::PresetMessage::{self, *};
use std::fs::create_dir_all;
//
use test_case::test_case;
#[test_case(Polyhedron::preset(&Pyramid(3)); "T")]
#[test_case(Polyhedron::preset(&Prism(4)); "C")]
#[test_case(Polyhedron::preset(&Octahedron); "O")]
#[test_case(Polyhedron::preset(&Dodecahedron); "D")]
#[test_case(Polyhedron::preset(&Icosahedron); "I")]
// #[test_case({ let mut g = Polyhedron::preset(&Prism(4)); g.truncate(0); g} ; "tC")]
// #[test_case({ let mut g = Polyhedron::preset(&Octahedron); g.truncate(0); g} ; "tO")]
// #[test_case({ let mut g = Polyhedron::preset(&Dodecahedron); g.truncate(0); g} ; "tD")]
fn polytope_apsp(poly: Polyhedron) {
    let mut bfs = poly.clone();
    bfs.shape.recompute();
    let mut floyd = poly.clone();
    floyd.shape.floyd();
    assert_eq!(bfs.shape, poly.shape);
    assert_eq!(bfs.shape, floyd.shape);
}

#[test]
fn ambo() {
    use PresetMessage::*;
    let prefix = "tests/ambo/";
    create_dir_all(prefix).unwrap();
    let mut polyhedron = Polyhedron::preset(&Pyramid(3));
    // polyhedron.shape.png();
    polyhedron.ambo_contract();
    // polyhedron.shape.png();
    let octahedron = Polyhedron::preset(&Octahedron);
    // octahedron.shape.png();
    assert_eq!(polyhedron.shape, octahedron.shape);
}
