use serde::{Deserialize, Serialize};

use super::VertexId;

pub type EdgeId = (VertexId, VertexId);

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Edge {
    pub a: VertexId,
    pub b: VertexId,
}

impl Edge {
    pub fn id(&self) -> EdgeId {
        if self.a < self.b {
            (self.a, self.b)
        } else {
            (self.b, self.a)
        }
    }
    pub fn other(&self, v: VertexId) -> VertexId {
        if self.a == v {
            self.b
        } else {
            self.a
        }
    }
}

/*
impl<V: Vertex> From<(V, V)> for Edge {
    fn from(value: (V, V)) -> Self {
        Self {
            a: value.0.id(),
            b: value.1.id(),
        }
    }
}
*/

impl From<(VertexId, VertexId)> for Edge {
    fn from(value: (VertexId, VertexId)) -> Self {
        Self {
            a: value.0,
            b: value.1,
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl std::cmp::PartialOrd for Edge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Edge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.id().cmp(&other.id())
    }
}

impl Eq for Edge {}
impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
