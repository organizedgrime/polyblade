use super::{Edge, VertexId};
use std::{collections::HashSet, hash::Hash};

#[derive(Clone)]
pub struct Face(pub Vec<VertexId>);

impl std::fmt::Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Face").field(&self.0).finish()
    }
}

impl Face {
    pub fn contains(&self, other: &Face) -> bool {
        other.0.iter().all(|v| self.0.contains(v))
    }

    pub fn edges(&self) -> HashSet<Edge> {
        let mut edges = HashSet::new();
        for i in 0..self.0.len() {
            edges.insert((self.0[i], self.0[(i + 1) % self.0.len()]).into());
        }
        edges
    }

    //pub fn replace(&mut self, )
}

impl PartialEq for Face {
    fn eq(&self, other: &Self) -> bool {
        self.contains(other) && self.0.len() == other.0.len()
    }
}

impl Eq for Face {}
impl Hash for Face {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        let mut edges = self.edges().into_iter().collect::<Vec<_>>();
        edges.sort();
        for edge in edges {
            edge.hash(state);
        }
    }
}
