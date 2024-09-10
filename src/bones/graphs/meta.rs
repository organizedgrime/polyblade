use super::simple::{Simple, SimpleGraph};

/// The MetaGraph stores a SimpleGraph and its derived properties
pub struct Meta {
    pub graph: Simple,
    pub cycles: Vec<Face>,
    pub dist: HashMap<VertexId, usize>,
}

pub trait MetaGraph {
    fn pst(&mut self);
    fn find_cycles(&mut self);
}
