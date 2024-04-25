use super::{Edge, VertexId};
use std::{
    collections::HashSet,
    hash::Hash,
    ops::{Index, IndexMut},
    slice::SliceIndex,
};

#[derive(Clone)]
pub struct Face(Vec<VertexId>);

impl Face {
    pub fn new(vertices: Vec<VertexId>) -> Self {
        Self(vertices)
    }
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

    pub fn push(&mut self, value: VertexId) {
        self.0.push(value)
    }
}

impl From<Vec<Edge>> for Face {
    fn from(mut value: Vec<Edge>) -> Self {
        let mut face = vec![value[0].v()];
        while !value.is_empty() {
            let prev = *face.last().unwrap();
            let i = value.iter().position(|e| e.contains(prev)).unwrap();
            let next = value[i].other(prev).unwrap();
            face.push(next);
            value.remove(i);
        }
        Self::new(face)
    }
}

impl<Idx> Index<Idx> for Face
where
    Idx: SliceIndex<[usize]>,
{
    type Output = Idx::Output;

    #[inline(always)]
    fn index(&self, index: Idx) -> &Self::Output {
        self.0.index(index)
    }
}
impl<Idx> IndexMut<Idx> for Face
where
    Idx: SliceIndex<[usize], Output = usize>,
{
    #[inline]
    fn index_mut(&mut self, index: Idx) -> &mut Self::Output {
        self.0.index_mut(index)
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

impl std::fmt::Debug for Face {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let i: usize = self
            .0
            .iter()
            .enumerate()
            .min_by(|(_, a), (_, b)| a.cmp(b))
            .map(|(index, _)| index)
            .unwrap();
        let id = [self.0[i..].to_vec(), self.0[..i].to_vec()].concat();
        f.debug_tuple("Face").field(&id).finish()
    }
}
