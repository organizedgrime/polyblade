use super::{Edge, VertexId};
use std::{collections::HashSet, hash::Hash};

#[derive(Clone)]
pub struct Face(pub Vec<VertexId>);

impl std::fmt::Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Face").field(&self.id()).finish()
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

    pub fn id(&self) -> Vec<VertexId> {
        let i: usize = self
            .0
            .clone()
            .into_iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        [self.0[i..].to_vec(), self.0[..i].to_vec()].concat()
    }

    pub fn get(&self, index: usize) -> VertexId {
        self.0[index % self.0.len()]
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn iter(&self) -> std::slice::Iter<usize> {
        self.0.iter()
    }

    pub fn remove(&mut self, index: usize) -> VertexId {
        self.0.remove(index)
    }

    pub fn insert(&mut self, index: usize, v: VertexId) {
        self.0.insert(index, v)
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
        let mut edges = self.edges().into_iter().collect::<Vec<_>>();
        edges.sort();
        for edge in edges {
            edge.hash(state);
        }
    }
}
