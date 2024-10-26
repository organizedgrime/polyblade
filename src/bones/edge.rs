use crate::bones::VertexId;
use std::{
    fmt::Display,
    ops::{Index, IndexMut},
};

pub type EdgeId = (VertexId, VertexId);

#[derive(Debug, Clone, Copy)]
pub struct Edge {
    pub(crate) v: VertexId,
    pub(crate) u: VertexId,
}

impl Edge {
    pub fn id(&self) -> EdgeId {
        if self.v < self.u {
            (self.v, self.u)
        } else {
            (self.u, self.v)
        }
    }

    pub fn v(&self) -> VertexId {
        self.id().0
    }

    pub fn u(&self) -> VertexId {
        self.id().1
    }

    pub fn contains(&self, v: VertexId) -> bool {
        self.v == v || self.u == v
    }

    pub fn other(&self, v: VertexId) -> Option<VertexId> {
        if self.v == v {
            Some(self.u)
        } else if self.u == v {
            Some(self.v)
        } else {
            None
        }
    }
}

impl Index<Edge> for Vec<Vec<VertexId>> {
    type Output = VertexId;
    fn index(&self, idx: Edge) -> &Self::Output {
        &self[idx.v()][idx.u()]
    }
}

impl IndexMut<Edge> for Vec<Vec<VertexId>> {
    fn index_mut(&mut self, idx: Edge) -> &mut Self::Output {
        &mut self[idx.v()][idx.u()]
    }
}

impl Display for Edge {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("({}, {})", self.id().0, self.id().1))
    }
}

impl From<(VertexId, VertexId)> for Edge {
    fn from(value: (VertexId, VertexId)) -> Self {
        Self {
            v: value.0,
            u: value.1,
        }
    }
}

impl From<(&VertexId, &VertexId)> for Edge {
    fn from(value: (&VertexId, &VertexId)) -> Self {
        Self {
            v: *value.0,
            u: *value.1,
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
