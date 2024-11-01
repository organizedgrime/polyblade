use std::collections::HashMap;

use crate::render::{
    message::ColorMethodMessage,
    pipeline::{MomentVertex, ShapeVertex},
    state::{ModelState, RenderState},
};
use ultraviolet::{Vec3, Vec4};

#[derive(Debug)]
pub struct PolyhedronPrimitive {
    pub model: ModelState,
    pub render: RenderState,
}

impl PolyhedronPrimitive {
    pub fn new(model: ModelState, render: RenderState) -> Self {
        Self { model, render }
    }

    /// All the vertices that will change moment to moment
    pub fn moment_vertices(&self) -> Vec<MomentVertex> {
        let polyhedron = &self.model.polyhedron;
        let colors = &self.render.picker.palette.colors;

        match self.render.method {
            ColorMethodMessage::Vertex => todo!(),
            ColorMethodMessage::Edge => todo!(),
            ColorMethodMessage::Polygon => {
                // Polygon side count -> color
                let color_map: HashMap<usize, Vec4> =
                    polyhedron
                        .shape
                        .cycles
                        .iter()
                        .fold(HashMap::new(), |mut acc, c| {
                            if !acc.contains_key(&c.len()) {
                                acc.insert(c.len(), colors[acc.len() % colors.len()].into());
                            }
                            acc
                        });
                polyhedron
                    .shape
                    .cycles
                    .iter()
                    .map(|cycle| {
                        let color = *color_map.get(&cycle.len()).unwrap();
                        let positions: Vec<Vec3> = cycle
                            .iter()
                            .map(|&c| polyhedron.render.positions[c])
                            .collect();

                        match cycle.len() {
                            3 => positions
                                .iter()
                                .map(|&position| MomentVertex::new(position, color))
                                .collect(),
                            4 => [0usize, 1, 2, 2, 3, 0]
                                .iter()
                                .map(|&i| positions[i])
                                .map(|position| MomentVertex::new(position, color))
                                .collect(),
                            _ => {
                                let centroid: Vec3 =
                                    positions.iter().fold(Vec3::zero(), |a, &b| a + b)
                                        / positions.len() as f32;
                                (0..cycle.len())
                                    .map(|i| {
                                        vec![
                                            positions[i],
                                            centroid,
                                            positions[(i + 1) % positions.len()],
                                        ]
                                        .into_iter()
                                        .map(|position| MomentVertex::new(position, color))
                                        .collect()
                                    })
                                    .collect::<Vec<Vec<MomentVertex>>>()
                                    .concat()
                            }
                        }
                    })
                    .collect::<Vec<Vec<MomentVertex>>>()
                    .concat()
            }
            ColorMethodMessage::Face => todo!(),
        }
    }
}
