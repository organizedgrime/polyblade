use serde::{Deserialize, Serialize};

use super::VertexId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Face(pub Vec<VertexId>);

impl Face {
    //pub fn new(data: )
    pub fn contains(&self, other: &Face) -> bool {
        other
            .0
            .iter()
            .fold(true, |acc, v| acc && self.0.contains(v))
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, VertexId> {
        self.0.iter()
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.contains(other) && self.0.len() == other.0.len()
    }
}
