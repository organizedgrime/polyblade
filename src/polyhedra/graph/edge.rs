use super::{Vertex, VertexId};

pub type EdgeId = (VertexId, VertexId);

#[derive(Debug, Clone, Copy)]
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
            self.b.clone()
        } else {
            self.a.clone()
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
        let id1 = self.id();
        let id2 = other.id();
        match id1.0.partial_cmp(&id2.0) {
            Some(core::cmp::Ordering::Equal) => id1.1.partial_cmp(&id2.1),
            ord => ord,
        }
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
