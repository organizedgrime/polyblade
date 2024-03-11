use std::fmt::Debug;
pub trait Vertex: Clone + Copy + PartialEq + Eq + PartialOrd + Ord + Debug {}
impl Vertex for usize {}
