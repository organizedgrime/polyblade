use std::collections::HashMap;

use crate::render::message::ColorMethodMessage;
use crate::render::state::{ModelState, RenderState};
use iced::widget::shader::{self, wgpu};
use iced::{Rectangle, Size};
use ultraviolet::{Vec3, Vec4};

use super::{AllUniforms, FragUniforms, ModelUniforms, Pipeline, Vertex};

#[derive(Debug)]
pub struct PolyhedronPrimitive {
    pub model: ModelState,
    pub render: RenderState,
}

impl PolyhedronPrimitive {
    pub fn new(model: ModelState, render: RenderState) -> Self {
        Self { model, render }
    }

    /* pub fn indices(&self) -> Vec<u32> {
        match self.render.method {
            // Reference data is one per vertex
            ColorMethodMessage::Vertex => self
                .model
                .polyhedron
                .vertices
                .iter()
                .map(|v| self.model.polyhedron.positions[v])
                .collect(),
            // Reference data is one per unique vertex, face pair
            ColorMethodMessage::Polygon => todo!(),
            // Todo
            ColorMethodMessage::Edge => todo!(),
            ColorMethodMessage::Face => todo!(),
        }
    } */

    /// All the vertices that will change moment to moment
    pub fn vertices(&self) -> (Vec<Vertex>, Vec<u32>) {
        let polyhedron = &self.model.polyhedron;

        match self.render.method {
            ColorMethodMessage::Vertex => {
                let mut verts: Vec<usize> = polyhedron.vertices.clone().into_iter().collect();
                verts.sort();
                let colors = &self.render.picker.palette.colors;
                let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
                let sides = Vec3::new(1.0, 1.0, 1.0);

                // Accumulate a list of all the positions we know to expect
                let mut vertices = vec![];
                for (i, v) in verts.iter().enumerate() {
                    vertices.push(Vertex {
                        position: polyhedron.positions[v].into(),
                        color: colors[i % colors.len()].into(),
                        barycentric: barycentric[i % barycentric.len()].into(),
                        sides: sides.into(),
                    });
                }

                verts.iter().fold(vec![], |mut acc, v| {
                    acc.push(polyhedron.positions[v]);
                    acc
                });

                // Iterate through every face and accumulate a list of indices
                let mut indices = vec![];
                for cycle in &polyhedron.cycles {
                    let cycle_indices: Vec<u32> = cycle
                        .iter()
                        .map(|c| verts.iter().position(|v| v == c).unwrap() as u32)
                        .collect();

                    match cycle.len() {
                        3 => {
                            indices.extend(cycle_indices);
                        }
                        4 => {
                            indices.extend(vec![
                                cycle_indices[0],
                                cycle_indices[1],
                                cycle_indices[2],
                                cycle_indices[2],
                                cycle_indices[3],
                                cycle_indices[0],
                            ]);
                        }
                        _ => {
                            for i in 0..cycle_indices.len() {
                                let triangle = vec![
                                    // Before
                                    cycle_indices[i],
                                    // Centroid index
                                    vertices.len() as u32,
                                    // After
                                    cycle_indices[(i + 1) % cycle_indices.len()],
                                ];
                                indices.extend(triangle);
                            }
                            // Compute the centroid
                            let centroid = cycle
                                .iter()
                                .fold(Vec3::zero(), |acc, v| acc + polyhedron.positions[v])
                                / cycle.len() as f32;
                            // Add it to the moment vertices
                            vertices.push(Vertex {
                                position: centroid.into(),
                                color: colors[0].into(),
                                barycentric: barycentric[0],
                                sides: sides.into(),
                            });
                        }
                    }
                }
                println!("indices: {:?}", indices);
                println!("vertices: {:?}", vertices);

                (vertices, indices)
            }
            ColorMethodMessage::Edge => todo!(),
            ColorMethodMessage::Polygon => {
                // hashmap of polygon length to color
                let colors = &self.render.picker.palette.colors;
                let mut color_map: HashMap<usize, Vec4> = HashMap::new();
                for cycle in &polyhedron.cycles {
                    if !color_map.contains_key(&cycle.len()) {
                        color_map
                            .insert(cycle.len(), colors[color_map.len() % colors.len()].into());
                    }
                }

                let mut vertices = Vec::new();
                let mut indices = Vec::new();
                let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
                for (i, cycle) in polyhedron.cycles.iter().enumerate() {
                    let color = *color_map.get(&cycle.len()).unwrap();
                    let sides = self.face_sides_buffer(i);
                    let positions = self.face_triangle_positions(i);

                    for j in 0..positions.len() {
                        indices.push(vertices.len() as u32);
                        vertices.push(Vertex {
                            position: positions[j].into(),
                            sides: sides[j].into(),
                            barycentric: barycentric[j % barycentric.len()].into(),
                            color,
                        });
                    }
                }

                println!("vertices: {:?}", vertices);
                println!("indices: {:?}", indices);

                (vertices, indices)
            }
            ColorMethodMessage::Face => todo!(),
        }
    }

    fn face_triangle_positions(&self, face_index: usize) -> Vec<Vec3> {
        let positions = self.model.polyhedron.face_positions(face_index);
        let n = positions.len();
        match n {
            3 => positions,
            4 => vec![
                positions[0],
                positions[1],
                positions[2],
                positions[2],
                positions[3],
                positions[0],
            ],
            _ => {
                let centroid = self.model.polyhedron.face_centroid(face_index);
                let n = positions.len();
                (0..n).fold(vec![], |acc, i| {
                    [acc, vec![positions[i], centroid, positions[(i + 1) % n]]].concat()
                })
            }
        }
    }

    pub fn color_buf(&self) -> (Vec<Vec4>, Vec<u32>) {
        let colors: Vec<Vec4> = self
            .render
            .picker
            .palette
            .colors
            .iter()
            .map(|&c| c.into())
            .collect();

        let mut indices = vec![];
        let polyhedron = &self.model.polyhedron;
        for (i, cycle) in polyhedron.cycles.iter().enumerate() {
            match cycle.len() {
                3 => {
                    indices.extend(vec![(i % colors.len()) as u32; 3]);
                }
                4 => {
                    indices.extend(vec![(i % colors.len()) as u32; 6]);
                }
                _ => {
                    indices.extend(vec![(i % colors.len()) as u32; cycle.len() * 3]);
                }
            }
        }

        (colors, indices)
    }
    pub fn barycentric_buf(&self) -> (Vec<Vec4>, Vec<u32>) {
        let barycentric: Vec<Vec4> = vec![Vec4::unit_x(), Vec4::unit_y(), Vec4::unit_z()];
        let mut indices = vec![];
        let polyhedron = &self.model.polyhedron;
        for (i, cycle) in polyhedron.cycles.iter().enumerate() {
            match cycle.len() {
                3 => {
                    indices.extend(vec![(i % barycentric.len()) as u32; 3]);
                }
                4 => {
                    indices.extend(vec![(i % barycentric.len()) as u32; 6]);
                }
                _ => {
                    indices.extend(vec![(i % barycentric.len()) as u32; cycle.len() * 3]);
                }
            }
        }

        (barycentric, indices)
    }

    pub fn sides_buf(&self) -> (Vec<Vec4>, Vec<u32>) {
        let sides = vec![
            Vec4::new(1.0, 1.0, 1.0, 0.0),
            Vec4::new(1.0, 0.0, 1.0, 0.0),
            Vec4::new(0.0, 1.0, 0.0, 0.0),
        ];
        let mut indices = vec![];
        for cycle in &self.model.polyhedron.cycles {
            indices.extend(match cycle.len() {
                3 => vec![0; 3],
                4 => vec![1; 6],
                _ => vec![2; cycle.len() * 3],
            });
        }
        (sides, indices)
    }

    pub fn face_sides_buffer(&self, face_index: usize) -> Vec<Vec3> {
        let n = self.model.polyhedron.cycles[face_index].len();
        match n {
            3 => vec![Vec3::new(1.0, 1.0, 1.0); 3],
            4 => vec![Vec3::new(1.0, 0.0, 1.0); 6],
            _ => vec![Vec3::new(0.0, 1.0, 0.0); n * 3],
        }
    }
}

impl shader::Primitive for PolyhedronPrimitive {
    fn prepare(
        &self,
        format: wgpu::TextureFormat,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        bounds: Rectangle,
        target_size: Size<u32>,
        _scale_factor: f32,
        storage: &mut shader::Storage,
    ) {
        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(device, format, target_size));
        }
        let pipeline = storage.get_mut::<Pipeline>().unwrap();

        // Construct new Unifrom Buffer
        let uniforms = AllUniforms {
            model: ModelUniforms {
                model_mat: self.model.transform,
                view_projection_mat: self.render.camera.build_view_proj_mat(bounds),
            },
            frag: FragUniforms {
                line_thickness: self.render.line_thickness,
            },
        };

        // Update GPU data
        pipeline.update(device, queue, target_size, &uniforms, self);
    }

    fn render(
        &self,
        storage: &shader::Storage,
        target: &wgpu::TextureView,
        _target_size: Size<u32>,
        viewport: Rectangle<u32>,
        encoder: &mut wgpu::CommandEncoder,
    ) {
        // At this point our pipeline should always be initialized
        let pipeline = storage.get::<Pipeline>().unwrap();

        // Render primitive
        pipeline.render(target, encoder, viewport);
    }
}
