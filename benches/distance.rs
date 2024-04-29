use divan::{black_box, Bencher};
use polyblade::prelude::PolyGraph;
fn main() {
    // Run registered benchmarks.
    divan::main();
}

#[divan::bench(args = [truncated(0), truncated(5), truncated(10)])]
fn distance(t: &&mut PolyGraph) {
    let mut t = black_box(t);
    t.pst();
}

fn truncated(n: usize) -> &mut PolyGraph {
    let mut t = PolyGraph::tetrahedron();
    for i in 0..n {
        t.truncate();
    }
    &mut t
}
