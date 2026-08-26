use super::Cycles;
use crate::polyhedron::{FaceId, VertexId};
use std::collections::HashMap;

/// Order-independent key for an undirected edge.
/// TODO: maybe we should rewrite this as a custom type
pub(super) fn undirected(a: VertexId, b: VertexId) -> [VertexId; 2] {
    if a < b { [a, b] } else { [b, a] }
}

/// Maps each directed edge to the one face traversing it, valid under the winding invariant.
pub(super) fn directed_edge_faces(
    cycles: &[Vec<VertexId>],
) -> HashMap<(VertexId, VertexId), usize> {
    let mut directed = HashMap::new();
    for (f, cycle) in cycles.iter().enumerate() {
        let n = cycle.len();
        for k in 0..n {
            directed.insert((cycle[k], cycle[(k + 1) % n]), f);
        }
    }
    directed
}

/// Read-only snapshot of a shape's faces taken at the start of a Conway operation
pub(super) struct FaceTopology {
    /// The original faces
    pub(super) cycles: Vec<Vec<VertexId>>,
    /// The IDs of those original faces
    pub(super) ids: Vec<FaceId>,
    /// Vertex `v` in face `f` = pos[f][&v]
    pub(super) pos: Vec<HashMap<VertexId, usize>>,
    /// Directed original edge to the face traversing it
    directed: HashMap<(VertexId, VertexId), usize>,
}

impl FaceTopology {
    pub(super) fn snapshot(cycles: &Cycles) -> Self {
        let ids = cycles.ids().to_vec();
        let cycles: Vec<Vec<VertexId>> =
            cycles.iter().map(|c| c.iter().copied().collect()).collect();
        let pos = cycles
            .iter()
            .map(|cycle| cycle.iter().enumerate().map(|(k, &v)| (v, k)).collect())
            .collect();
        let directed = directed_edge_faces(&cycles);
        Self {
            cycles,
            ids,
            pos,
            directed,
        }
    }

    /// Index of vertex `v` within face `f`.
    pub(super) fn pos(&self, f: usize, v: VertexId) -> usize {
        self.pos[f][&v]
    }

    /// The face across the edge `a,b`, aka the unique face winding `b -> a`.
    pub(super) fn face_across(&self, a: VertexId, b: VertexId) -> usize {
        self.directed[&(b, a)]
    }

    /// Visits each edge exactly once in face order then corner order.
    /// Yields the face `f` it was found in, its endpoints `a,b` in `f`'s winding, and the opposite face `g`.
    pub(super) fn for_each_edge(&self, mut visit: impl FnMut(usize, VertexId, VertexId, usize)) {
        for (f, cycle) in self.cycles.iter().enumerate() {
            let n = cycle.len();
            for k in 0..n {
                let (a, b) = (cycle[k], cycle[(k + 1) % n]);
                if a < b {
                    visit(f, a, b, self.face_across(a, b));
                }
            }
        }
    }
}
