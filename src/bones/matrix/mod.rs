#[cfg(test)]
mod test;

use std::ops::{Index, IndexMut, Range};

use rustc_hash::FxHashSet as HashSet;

use super::{Edge, VertexId};

/// Jagged array which represents the symmetrix distance matrix of a given Graph
/// usize::MAX    ->   disconnected
/// 0             ->   identity
/// n             ->   actual distance
#[derive(Debug, Default, Clone, PartialEq)]
pub struct Matrix(Vec<Vec<usize>>);

impl Matrix {
    /// [ 0 ]
    /// [ M | 0 ]
    /// [ M | M | 0 ]
    /// ..
    /// [ M | M | M | ... | M | M | M | 0 ]
    pub fn new(n: usize) -> Self {
        Matrix(
            (0..n)
                .into_iter()
                .map(|m| [vec![usize::MAX; m], vec![0]].concat())
                .collect(),
        )
    }
}

impl Matrix {
    /// Connect one vertex to another with length one, iff they are note the same point
    pub fn connect<T>(&mut self, i: T)
    where
        Matrix: Index<T, Output = usize> + IndexMut<T, Output = usize>,
        T: Copy,
    {
        if self[i] != 0 {
            self[i] = 1;
        }
    }

    /// Disconnect one vertex from another iff they are neighbors
    pub fn disconnect<T>(&mut self, i: T)
    where
        Matrix: Index<T, Output = usize> + IndexMut<T, Output = usize>,
        T: Copy,
    {
        if self[i] == 1 {
            self[i] = usize::MAX;
        }
    }

    /// Inserts a new vertex in the matrix
    pub fn insert(&mut self) -> VertexId {
        self.0
            .push([vec![usize::MAX; self.0.len() - 1], vec![0]].concat());
        self.0.len()
    }

    /// Deletes a vertex from the matrix
    pub fn delete(&mut self, v: VertexId) {
        for row in &mut self.0[v..] {
            row.remove(v);
        }
        self.0.remove(v);
    }

    /// Enumerates the vertices connected to v
    pub fn connections(&self, v: VertexId) -> Vec<VertexId> {
        self.vertices().filter(|&u| self[[v, u]] == 1).collect()
    }

    /// Iterable Range representing vertex IDs
    pub fn vertices(&self) -> Range<VertexId> {
        0..self.0.len()
    }

    /// Vertex Count
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Maximum distance value
    pub fn diameter(&self) -> usize {
        self.vertices()
            .zip(self.vertices())
            .map(|(v, u)| self[[v, u]])
            .max()
            .unwrap_or(0)
    }
}

impl Index<[VertexId; 2]> for Matrix {
    type Output = usize;

    fn index(&self, index: [VertexId; 2]) -> &Self::Output {
        &self.0[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl IndexMut<[VertexId; 2]> for Matrix {
    fn index_mut(&mut self, index: [VertexId; 2]) -> &mut Self::Output {
        &mut self.0[index[0].max(index[1])][index[0].min(index[1])]
    }
}

impl Index<Edge> for Matrix {
    type Output = usize;

    fn index(&self, index: Edge) -> &Self::Output {
        &self.0[index.v.max(index.u)][index.v.min(index.u)]
    }
}

impl IndexMut<Edge> for Matrix {
    fn index_mut(&mut self, index: Edge) -> &mut Self::Output {
        &mut self.0[index.v.max(index.u)][index.v.min(index.u)]
    }
}
