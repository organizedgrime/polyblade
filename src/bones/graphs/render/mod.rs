mod transaction;
use std::collections::{HashMap, HashSet};
pub use transaction::*;

use crate::bones::graphs::*;
use polyblade_derive::{MetaGraph, SimpleGraph};
use ultraviolet::{Lerp, Vec3};

#[derive(SimpleGraph, MetaGraph)]
pub struct Render {
    #[internal]
    pub meta: Meta,
    /// Ongoing changes to the render schema
    pub transactions: Vec<Transaction>,
    /// 3D Position Data
    pub positions: HashMap<VertexId, Vec3>,
    /// 3D Speed Data
    pub speeds: HashMap<VertexId, Vec3>,
    /// Legnth of each edge
    pub edge_length: f32,
}

impl Render {
    const TICK_SPEED: f32 = 400.0;

    fn apply_spring_forces(&mut self) {
        let diam = *self.meta.dist.values().max().unwrap_or(&1) as f32;
        let l_diam = self.edge_length * 2.0;

        let vertices: HashSet<VertexId> = self.vertices().cloned().collect();
        for v in vertices.iter() {
            for u in vertices.iter() {
                if u != v {
                    let e: Edge = (v, u).into();
                    if let Some(Transaction::Contraction(contracting_edges)) =
                        self.transactions.first()
                    {
                        if contracting_edges.contains(&e) {
                            let v_position = self.positions[v];
                            let u_position = self.positions[u];
                            let l = (v_position - u_position).mag();
                            let f = (self.edge_length / Self::TICK_SPEED * 4.5) / l;
                            *self.positions.get_mut(v).unwrap() = v_position.lerp(u_position, f);
                            *self.positions.get_mut(u).unwrap() = u_position.lerp(v_position, f);
                            continue;
                        }
                    } else if self.meta.dist.contains_key(&e) {
                        let d = self.meta.dist[&e] as f32;
                        let l = l_diam * (d / diam);
                        let k = 1.0 / d;
                        let diff = self.positions[v] - self.positions[u];
                        let dist = diff.mag();
                        let distention = l - dist;
                        let restorative_force = k / 2.0 * distention;
                        let f = diff * restorative_force / Self::TICK_SPEED;

                        let v_speed = self.speeds.get_mut(v).unwrap();
                        *v_speed += f;
                        *v_speed *= 0.92;
                        *self.positions.get_mut(v).unwrap() += *v_speed;

                        let u_speed = self.speeds.get_mut(u).unwrap();
                        *u_speed -= f;
                        *u_speed *= 0.92;
                        *self.positions.get_mut(u).unwrap() += *u_speed;
                    }
                }
            }
        }
    }

    fn center(&mut self) {
        let shift =
            self.positions.values().fold(Vec3::zero(), |a, &b| a + b) / self.vertex_count() as f32;

        for (_, v) in self.positions.iter_mut() {
            *v -= shift;
        }
    }

    fn resize(&mut self) {
        let mean_length = self.positions.values().map(|p| p.mag()).fold(0.0, f32::max);
        let distance = mean_length - 1.0;
        self.edge_length -= distance / Self::TICK_SPEED;
    }
}
