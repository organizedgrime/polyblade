use crate::bones::VertexId;
use std::ops::{Index, IndexMut};

#[derive(Default, Debug, Clone)]
pub struct Cycle(Vec<VertexId>);

#[derive(Default, Debug, Clone)]
pub struct Cycles {
    // Circular lists of Vertex Ids representing faces
    cycles: Vec<Cycle>,
}

impl Cycles {
    pub fn new(cycles: Vec<Vec<VertexId>>) -> Self {
        Self {
            cycles: cycles.into_iter().map(|cycle| Cycle(cycle)).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.cycles.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Cycle> {
        self.cycles.iter()
    }
}

impl Index<usize> for Cycle {
    type Output = VertexId;

    fn index(&self, index: usize) -> &Self::Output {
        &self.0[index % self.0.len()]
    }
}

impl Index<usize> for Cycles {
    type Output = Cycle;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cycles[index % self.cycles.len()]
    }
}
impl IndexMut<usize> for Cycle {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let i = index % self.0.len();
        &mut self.0[i]
    }
}

impl Cycle {
    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn delete(&mut self, v: VertexId) {
        (*self).0 = self
            .0
            .clone()
            .into_iter()
            .filter_map(|u| {
                if v == u {
                    None
                } else if u > v {
                    Some(u - 1)
                } else {
                    Some(u)
                }
            })
            .collect::<Vec<_>>();
    }

    pub fn replace(&mut self, old: VertexId, new: VertexId) {
        (*self).0 = self
            .0
            .clone()
            .into_iter()
            .filter_map(|v| {
                if v == new {
                    None
                } else if v == old {
                    Some(new)
                } else {
                    Some(v)
                }
            })
            .collect();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, usize> {
        self.0.iter()
    }
}
impl Cycles {
    pub fn delete(&mut self, v: VertexId) {
        for cycle in &mut self.cycles {
            cycle.delete(v);
        }
    }

    /// Replace all occurrence of one vertex with another
    pub fn replace(&mut self, old: VertexId, new: VertexId) {
        for cycle in &mut self.cycles {
            cycle.replace(old, new);
        }
    }
}

/*
impl Cycles {
    /// All faces
    // pub fn find_cycles(&mut self) {
    //     self.cycles = cycles.into_iter().collect();
    // }

    pub fn delete(&mut self, v: VertexId) {
        self.distance.delete(v);

        for cycle in &mut self.cycles {
            *cycle = cycle.iter().filter(|&c| c != &v).cloned().collect();
        }

        for i in 0..self.cycles.len() {
            for j in 0..self.cycles[i].len() {
                if self.cycles[i][j] >= v {
                    self.cycles[i][j] -= 1;
                }
            }
        }
    }
}
*/
