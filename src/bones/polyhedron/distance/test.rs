use std::fs::create_dir_all;

use super::*;
use crate::render::message::PresetMessage::{self, *};
use test_case::test_case;

impl Distance {
    pub fn floyd(&mut self) {
        // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
        let mut graph: Distance = Distance::new(self.distance.len());
        for e in self.edges() {
            graph[e] = 1;
        }
        for k in graph.vertices() {
            for i in graph.vertices() {
                for j in graph.vertices() {
                    if graph[[i, k]] != usize::MAX && graph[[k, j]] != usize::MAX {
                        let nv = graph[[i, k]] + graph[[k, j]];
                        if graph[[i, j]] > nv || graph[[j, i]] > nv {
                            graph[[i, j]] = nv;
                        }
                    }
                }
            }
        }
        *self = graph;
    }

    /// Hardcoded Tetrahedron construction to isolate testing
    pub fn tetrahedron() -> Self {
        let mut tetra = Distance::new(4);
        tetra[[0, 1]] = 1;
        tetra[[0, 2]] = 1;
        tetra[[0, 3]] = 1;
        tetra[[1, 2]] = 1;
        tetra[[1, 3]] = 1;
        tetra[[2, 3]] = 1;
        tetra
    }
}

#[test_case(Distance::preset(&Pyramid(3)); "T")]
#[test_case(Distance::preset(&Prism(4)); "C")]
#[test_case(Distance::preset(&Octahedron); "O")]
#[test_case(Distance::preset(&Dodecahedron); "D")]
#[test_case(Distance::preset(&Icosahedron); "I")]
#[test_case({ let mut g = Distance::preset(&Prism(4)); g.truncate(None); g.pst(); g} ; "tC")]
#[test_case({ let mut g = Distance::preset(&Octahedron); g.truncate(None); g.pst(); g} ; "tO")]
#[test_case({ let mut g = Distance::preset(&Dodecahedron); g.truncate(None); g.pst(); g} ; "tD")]
fn pst(graph: Distance) {
    let mut pst = graph.clone();
    pst.pst();

    let mut floyd = graph.clone();
    floyd.floyd();

    assert_eq!(floyd.distance, pst.distance);
}

#[test]
fn basics() {
    let mut graph = Distance::new(4);
    println!("basics:");
    // Connect
    graph.connect([0, 1]);
    graph.connect([0, 2]);
    graph.connect([1, 2]);
    assert_eq!(graph.connections(0), vec![1, 2]);
    assert_eq!(graph.connections(1), vec![0, 2]);
    assert_eq!(graph.connections(2), vec![0, 1]);
    assert_eq!(graph.connections(3), vec![]);

    // Disconnect
    graph.disconnect([0, 1]);
    assert_eq!(graph.connections(0), vec![2]);
    assert_eq!(graph.connections(1), vec![2]);

    // Delete
    graph.delete(1);
    assert_eq!(graph.connections(0), vec![1]);
    assert_eq!(graph.connections(2), vec![]);
    assert_eq!(graph.connections(1), vec![0]);
}

#[test]
fn chordless_cycles() {
    let mut graph = Distance::new(4);
    // Connect
    graph.connect([0, 1]);
    graph.connect([1, 2]);
    graph.connect([2, 3]);

    println!("chordless_cycles:");
    println!("{graph}");
    graph.pst();
    println!("{graph}");

    graph.connect([2, 0]);
}

#[test]
fn truncate_contract() {
    let prefix = "tests/truncate_contract/";
    create_dir_all(prefix).unwrap();
    let mut shape = Distance::tetrahedron();
    shape.render(prefix, "tetrahedron.svg");
    let edges = shape.truncate(None);
    println!("edges: {edges:?}");
    shape.render(prefix, "truncated_tetrahedron.svg");
    shape.contract_edges(edges);
    shape.render(prefix, "contracted_truncated_tetrahedron.svg");
    assert_eq!(shape, Distance::tetrahedron());
}

#[test]
fn contract_edge() {
    let mut graph = Distance::tetrahedron();
    graph.contract_edge([0, 2]);
    let mut triangle = Distance::new(3);
    triangle[[0, 1]] = 1;
    triangle[[1, 2]] = 1;
    triangle[[2, 0]] = 1;
    assert_eq!(graph, triangle);
}

#[test]
fn split_vertex() {
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
    let mut test = Distance::tetrahedron();
    test.split_vertex(0);
    assert_eq!(test.distance, control.distance);
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
    let mut test = Distance::tetrahedron();
    test.render(prefix, "tetrahedron.svg");
    let edges = test.split_vertex(0);
    println!("edges: {edges:?}");
    test.render(prefix, "split.svg");
    test.contract_edges(edges);
    test.render(prefix, "contracted.svg");
    assert_eq!(test.distance, Distance::tetrahedron().distance);
}

#[test]
fn ambo() {
    let prefix = "tests/ambo/";
    create_dir_all(prefix).unwrap();
    let tetrahedron = Distance::tetrahedron();
    assert_eq!(
        tetrahedron.ambod(),
        Distance::preset(&PresetMessage::Octahedron)
    );
}
