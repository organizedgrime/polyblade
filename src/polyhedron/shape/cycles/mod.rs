mod cycle;
use crate::{polyhedron::VertexId, render::pipeline::ShapeVertex};
pub use cycle::*;
use std::{collections::HashSet, ops::Index};
use ultraviolet::{Vec3, Vec4};

use super::Distance;

#[derive(Default, Debug, Clone)]
pub(in super::super) struct Cycles {
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
        // We only care about cycles that contain the vertex
        let mut relevant = self
            .iter()
            .filter_map(move |cycle| {
                if let Some(p) = cycle.iter().position(|&x| x == v) {
                    Some([cycle[p + cycle.len() - 1], cycle[p + 1]])
                } else {
                    None
                }
            })
            .collect::<Vec<[VertexId; 2]>>();

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

    pub fn shape_vertices(&self) -> Vec<ShapeVertex> {
        let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
        self.iter()
            .map(|cycle| {
                let sides: Vec4 = match cycle.len() {
                    3 => Vec3::new(1.0, 1.0, 1.0),
                    4 => Vec3::new(1.0, 0.0, 1.0),
                    _ => Vec3::new(0.0, 1.0, 0.0),
                }
                .into();

                let b_shapes: Vec<ShapeVertex> = barycentric
                    .iter()
                    .map(|&b| ShapeVertex {
                        barycentric: b.into(),
                        sides,
                    })
                    .collect();

                match cycle.len() {
                    3 => b_shapes.clone(),
                    4 => (0..6)
                        .map(|i| ShapeVertex {
                            barycentric: barycentric[i % 3].into(),
                            sides,
                        })
                        .collect(),
                    _ => vec![b_shapes; cycle.len()].concat(),
                }
            })
            .collect::<Vec<Vec<ShapeVertex>>>()
            .concat()
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
    fn from(distance: &Distance) -> Self {
        let mut triplets: Vec<Vec<_>> = Default::default();
        let mut cycles: HashSet<Vec<_>> = Default::default();
        let mut edge_incidents: Distance = distance.clone();
        for [v, u] in edge_incidents.vertex_pairs() {
            if edge_incidents[[v, u]] == 1 {
                edge_incidents[[v, u]] = 0;
            } else {
                edge_incidents[[v, u]] = usize::MAX;
            }
        }
        println!("distance:\n{distance}");
        println!("edge_incidents_starting:\n{edge_incidents}");

        // find all the triplets
        for u in 0..distance.len() {
            for x in (u + 1)..distance.len() {
                for y in (x + 1)..distance.len() {
                    if distance[[u, x]] == 1 && distance[[u, y]] == 1 {
                        if distance[[x, y]] == 1 {
                            cycles.insert(vec![x, u, y]);
                            edge_incidents[[x, u]] += 1;
                            edge_incidents[[u, y]] += 1;
                            edge_incidents[[y, x]] += 1;
                        } else {
                            triplets.push(vec![x, u, y]);
                        }
                    }
                }
            }
        }
        // find all the triplets
        // for u in distance.vertices() {
        //     let adj: Vec<VertexId> = distance.connections(u);
        //     for &x in adj.iter() {
        //         for &y in adj.iter() {
        //             if u < x && x < y {
        //                 let new_face = vec![x, u, y];
        //                 if distance[[x, y]] == 1 {
        //                     cycles.insert(new_face);
        //                 } else {
        //                     triplets.push(new_face);
        //                 }
        //             }
        //         }
        //     }
        // }
        println!("triplets:\n{triplets:?}");
        println!("cycles:\n{cycles:?}");

        // while there are unparsed triplets
        while !triplets.is_empty() {
            let p = triplets.remove(0);

            // for each v adjacent to u_t
            for v in distance.connections(p[p.len() - 1]) {
                if v > p[1] {
                    let adj_v = distance.connections(v);
                    // if v is not a neighbor of u_2..u_t-1
                    if !p[1..p.len() - 1].iter().any(|i| adj_v.contains(i)) {
                        // let mut new_face = p.clone();
                        // new_face.push(v);
                        let new = [p.clone(), vec![v]].concat();
                        if distance.connections(p[0]).contains(&v) {
                            if edge_incidents[[p[0], v]] >= 2 {
                                log::info!("i was about to send {new:?} but [{}, {}] is already greater than 2", p[0], v)
                            } else {
                                for i in 0..new.len() {
                                    edge_incidents[[new[i], new[(i + 1) % new.len()]]] += 1;
                                }
                                cycles.insert(new);
                            }
                        } else {
                            triplets.push(new);
                        }
                    }
                }
            }
        }

        let mut cycles = cycles.into_iter().collect::<Vec<_>>();
        cycles.sort_by_key(|c| c.len());
        println!("done_cycles:\n{cycles:?}");
        //let mut cycles = cycles[0..distance.face_count() as usize].to_vec();
        println!("done_cycles_filterd:\n{cycles:?}");
        Cycles::new(cycles)
    }
}
