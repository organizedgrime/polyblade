use super::*;
use crate::render::message::PresetMessage::{self, *};
use std::fs::create_dir_all;

use test_case::test_case;
#[test_case(Shape::preset(&Pyramid(3)); "T")]
#[test_case(Shape::preset(&Prism(4)); "C")]
#[test_case(Shape::preset(&Octahedron); "O")]
#[test_case(Shape::preset(&Dodecahedron); "D")]
#[test_case(Shape::preset(&Icosahedron); "I")]
#[test_case({ let mut g = Shape::preset(&Prism(4)); g.truncate(None); g.distance.pst(); g} ; "tC")]
#[test_case({ let mut g = Shape::preset(&Octahedron); g.truncate(None); g.distance.pst(); g} ; "tO")]
#[test_case({ let mut g = Shape::preset(&Dodecahedron); g.truncate(None); g.distance.pst(); g} ; "tD")]
fn polytope_pst(shape: Shape) {
    let mut pst = shape.clone();
    pst.distance.pst();
    let mut floyd = shape.clone();
    floyd.distance.floyd();
    assert_eq!(floyd.distance, pst.distance);
}

#[test]
fn truncate_contract() {
    let prefix = "tests/truncate_contract/";
    create_dir_all(prefix).unwrap();
    let mut shape = Shape::from(Distance::tetrahedron());
    //shape.distance.render(prefix, "tetrahedron.svg");
    let edges = shape.truncate(None);
    println!("edges: {edges:?}");
    //shape.distance.render(prefix, "truncated_tetrahedron.svg");
    shape.distance.contract_edges(edges);
    // shape
    //     .distance
    //     .render(prefix, "contracted_truncated_tetrahedron.svg");
    assert_eq!(shape.distance, Distance::tetrahedron());
}

#[test]
fn split_vertex() {
    let mut control = Distance::new(6);
    // Original outline
    control[[1, 2]] = 1;
    control[[2, 3]] = 1;
    control[[3, 1]] = 1;
    // Connections
    control[[5, 1]] = 1;
    control[[4, 2]] = 1;
    control[[0, 3]] = 1;
    // New face
    control[[0, 4]] = 1;
    control[[4, 5]] = 1;
    control[[5, 0]] = 1;
    control.pst();

    let prefix = "tests/split_vertex/";
    create_dir_all(prefix).unwrap();
    let mut test = Shape::from(Distance::tetrahedron());
    // test.distance.render(prefix, "test_tetrahedron.svg");
    test.split_vertex(0);
    // test.distance.render(prefix, "test_split.svg");
    test.recompute();
    // test.distance.render(prefix, "test_recompute.svg");
    // control.render(prefix, "control.svg");

    assert_eq!(test.distance, control);
}

#[test]
fn split_vertex_contract() {
    let prefix = "tests/split_vertex_contract/";
    create_dir_all(prefix).unwrap();
    let mut control = Distance::new(6);
    // Original outline
    control[[1, 2]] = 1;
    control[[2, 3]] = 1;
    control[[3, 1]] = 1;
    // Connections
    control[[0, 1]] = 1;
    control[[4, 2]] = 1;
    control[[5, 3]] = 1;
    // New face
    control[[0, 4]] = 1;
    control[[4, 5]] = 1;
    control[[5, 0]] = 1;
    let mut test = Shape::from(Distance::tetrahedron());
    // test.distance.render(prefix, "test_tetrahedron.svg");
    let edges = test.split_vertex(0);
    println!("edges: {edges:?}");
    // test.distance.render(prefix, "test_split.svg");
    test.distance.contract_edges(edges);
    // test.distance.render(prefix, "test_contracted.svg");
    // Distance::tetrahedron().render(prefix, "control.svg");
    assert_eq!(test.distance, Distance::tetrahedron());
}

// #[test]
// fn ambo() {
//     let prefix = "tests/ambo/";
//     create_dir_all(prefix).unwrap();
//     let tetrahedron = Distance::tetrahedron();
//     assert_eq!(
//         tetrahedron.ambod(),
//         Distance::preset(&PresetMessage::Octahedron)
//     );
// }
