use std::hash::Hash;

use serde::{Deserialize, Serialize};

use super::VertexId;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Face(pub Vec<VertexId>);

impl Face {
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

    pub fn id(&self) -> Vec<VertexId> {
        let m = self.0.iter().min().unwrap();
        let i = self.0.iter().find(|f| f == &m).unwrap().clone();
        [self.0[i..].to_vec(), self.0[..i].to_vec()].concat()
    }
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.contains(other) && self.0.len() == other.0.len()
    }
}

impl Eq for Face {}
impl Hash for Face {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state);
    }
}
