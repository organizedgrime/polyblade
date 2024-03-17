use std::fmt::Debug;
pub type VertexId = usize;

pub trait Vertex: Clone + PartialEq + PartialOrd + Debug {
    fn id(&self) -> VertexId;
}

impl Vertex for usize {
    fn id(&self) -> VertexId {
        *self
    }
}
