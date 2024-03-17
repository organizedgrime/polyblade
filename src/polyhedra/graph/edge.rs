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
impl From<&Edge> for Edge {
    fn from(value: &Edge) -> Self {
        (value.a.clone(), value.b.clone()).into()
    }
}
*/

impl<V: Vertex> From<(V, V)> for Edge {
    fn from(value: (V, V)) -> Self {
        Self {
            a: value.0.id(),
            b: value.1.id(),
        }
    }
}

impl PartialEq for Edge {
    fn eq(&self, other: &Self) -> bool {
        self.id() == other.id()
    }
}

impl Eq for Edge {}
impl std::hash::Hash for Edge {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
