use super::*;

// #[test_case(PolyGraph::pyramid(3); "T")]
// #[test_case(PolyGraph::prism(4); "C")]
// // #[test_case(PolyGraph::octahedron(); "O")]
// // #[test_case(PolyGraph::dodecahedron(); "D")]
// // #[test_case(PolyGraph::icosahedron(); "I")]
// // #[test_case({ let mut g = PolyGraph::prism(4); g.truncate(None); g.pst(); g} ; "tC")]
// // #[test_case({ let mut g = PolyGraph::octahedron(); g.truncate(None); g.pst(); g} ; "tO")]
// // #[test_case({ let mut g = PolyGraph::dodecahedron(); g.truncate(None); g.pst(); g} ; "tD")]
// fn pst(mut graph: PolyGraph) {
//     let new_dist = graph.matrix.clone();
//     graph.matrix = Default::default();
//     graph.floyd();
//     let old_dist = graph.matrix.clone();
//
//     //assert_eq!(old_dist, graph.dist);
//     assert_eq!(
//         old_dist
//             .clone()
//             .into_keys()
//             .collect::<HashSet<_>>()
//             .difference(&new_dist.clone().into_keys().collect::<HashSet<_>>())
//             .collect::<HashSet<_>>(),
//         HashSet::new()
//     );
//
//     let o1 = old_dist
//         .clone()
//         .into_iter()
//         .map(|(k, v)| (k.id().0, k.id().1, v))
//         .collect::<HashSet<_>>();
//     let o2 = &new_dist
//         .clone()
//         .into_iter()
//         .map(|(k, v)| (k.id().0, k.id().1, v))
//         .collect::<HashSet<_>>();
//
//     assert_eq!(
//         o1.difference(o2).collect::<HashSet<_>>(),
//         o2.difference(&o1).collect::<HashSet<_>>()
//     );
//     assert_eq!(old_dist, new_dist);
// }

#[test]
fn basics() {
    let mut graph = JagGraph::new(4);
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
    let mut graph = JagGraph::new(4);
    // Connect
    graph.connect([0, 1]);
    graph.connect([1, 2]);
    graph.connect([2, 3]);

    println!("chordless_cycles:");
    println!("{graph}");
    graph.pst();
    println!("{graph}");

    graph.connect([2, 0]);
    //graph.pst();
    graph.find_cycles();
    assert_eq!(graph.cycles, vec![Face::new(vec![0, 1, 2])]);
}

// #[test]
// fn truncate() {
//     let mut shape = JagGraph::icosahedron();
//     shape.truncate(None);
// }

#[test]
fn contract_edge() {
    println!("contract edge");
    let mut graph = JagGraph::prism(4);
    assert_eq!(graph.len(), 8);
    assert_eq!(graph.edges().count(), 12);
    println!("{graph}");
    graph.contract_edge([0, 1]);
    println!("{graph}");
    assert_eq!(graph.len(), 7);
    assert_eq!(graph.edges().count(), 11);
}

#[test]
fn split_vertex() {
    let mut graph = JagGraph::prism(4);
    assert_eq!(graph.len(), 8);
    assert_eq!(graph.edges().count(), 12);

    graph.split_vertex(0);

    assert_eq!(graph.len(), 10);
    assert_eq!(graph.edges().count(), 15);
}
