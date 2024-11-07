mod cycle;
use crate::polyhedron::VertexId;
pub use cycle::*;
use std::{collections::HashSet, ops::Index};

use super::Distance;

#[derive(Default, Debug, Clone)]
pub struct Cycles {
    // Circular lists of Vertex Ids representing faces
    cycles: Vec<Cycle>,
}

impl Cycles {
    pub fn new(cycles: Vec<Vec<VertexId>>) -> Self {
        Self {
            cycles: cycles.into_iter().map(Cycle).collect(),
        }
    }

    pub fn len(&self) -> usize {
        self.cycles.len()
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Cycle> {
        self.cycles.iter()
    }
    pub fn into_iter(&self) -> std::vec::IntoIter<Cycle> {
        self.cycles.clone().into_iter()
    }
    /// Returns the
    pub fn sorted_connections(&self, v: VertexId) -> Vec<VertexId> {
        log::info!("cycles: {:?}", self.cycles);
        log::info!("hunting for {v}");
        // We only care about cycles that contain the vertex
        let mut relevant = self
            .iter()
            .filter_map(move |cycle| {
                if let Some(p) = cycle.iter().position(|&x| x == v) {
                    log::info!("finding {p}");
                    Some([cycle[p + cycle.len() - 1], cycle[p + 1]])
                } else {
                    None
                }
            })
            .collect::<Vec<[VertexId; 2]>>();
        //.collect::<HashMap<VertexId, VertexId>>();

        log::info!("RELEVANT: {relevant:?}");
        let mut sorted_connections = vec![relevant[0][0]];
        loop {
            let previous = sorted_connections.last().unwrap();
            match relevant
                .iter()
                .position(|[v, u]| v == previous || u == previous)
            {
                Some(i) => {
                    let [v, u] = relevant.remove(i);
                    let next = if v == *previous { u } else { v };
                    sorted_connections.push(next);
                }
                None => {
                    break;
                }
            }
        }
        sorted_connections[1..].to_vec()
    }
}

impl Index<usize> for Cycles {
    type Output = Cycle;

    fn index(&self, index: usize) -> &Self::Output {
        &self.cycles[index.rem_euclid(self.cycles.len())]
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

impl From<&Distance> for Cycles {
    fn from(value: &Distance) -> Self {
        let mut triplets: Vec<Vec<_>> = Default::default();
        let mut cycles: HashSet<Vec<_>> = Default::default();

        // find all the triplets
        for u in value.vertices() {
            let adj: Vec<VertexId> = value.connections(u);
            for &x in adj.iter() {
                for &y in adj.iter() {
                    if x != y && u < x && x < y {
                        let new_face = vec![x, u, y];
                        if value[[x, y]] == 1 {
                            cycles.insert(new_face);
                        } else {
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }

        // while there are unparsed triplets
        while !triplets.is_empty() && (cycles.len() as i64) < value.face_count() {
            let p = triplets.remove(0);
            // for each v adjacent to u_t
            for v in value.connections(p[p.len() - 1]) {
                if v > p[1] {
                    let c = value.connections(v);
                    // if v is not a neighbor of u_2..u_t-1
                    if !p[1..p.len() - 1].iter().any(|vi| c.contains(vi)) {
                        let mut new_face = p.clone();
                        new_face.push(v);
                        if value.connections(p[0]).contains(&v) {
                            //cycles.remo
                            cycles.insert(new_face);
                        } else {
                            triplets.push(new_face);
                        }
                    }
                }
            }
        }

        Cycles::new(cycles.into_iter().collect::<Vec<_>>())
    }
}
