use super::*;
use test_case::test_case;

impl JagGraph {
    pub fn floyd(&mut self) {
        // let dist be a |V| × |V| array of minimum distances initialized to ∞ (infinity)
        let mut graph: JagGraph = JagGraph::new(self.matrix.len());

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

        let mut dd: HashMap<Edge, usize> = HashMap::default();
        for [v, u] in graph.vertex_pairs() {
            let dvu = graph[[v, u]];
            if dvu != usize::MAX && dvu != 0 {
                let e: Edge = (v, u).into();
                dd.insert(e, dvu as usize);
            }
        }

        *self = graph;
    }
}

#[test_case(JagGraph::pyramid(3); "T")]
#[test_case(JagGraph::prism(4); "C")]
#[test_case(JagGraph::octahedron(); "O")]
#[test_case(JagGraph::dodecahedron(); "D")]
#[test_case(JagGraph::icosahedron(); "I")]
#[test_case({ let mut g = JagGraph::prism(4); g.truncate(None); g.pst(); g} ; "tC")]
#[test_case({ let mut g = JagGraph::octahedron(); g.truncate(None); g.pst(); g} ; "tO")]
#[test_case({ let mut g = JagGraph::dodecahedron(); g.truncate(None); g.pst(); g} ; "tD")]
fn pst(mut graph: JagGraph) {
    let pst = graph.clone();
    graph.floyd();
    let floyd = graph.clone();
    assert_eq!(floyd.matrix, pst.matrix);
}

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

#[test]
fn truncate() {
    let mut shape = JagGraph::icosahedron();
    shape.truncate(None);
}

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
