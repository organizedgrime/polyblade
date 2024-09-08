use std::ptr::eq;

use crate::render::message::ColorMethodMessage;
use crate::render::state::{ModelState, RenderState};
use ckmeans::ckmeans;
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

    /// All the vertices that will change moment to moment
    pub fn position_buf(&self) -> (Vec<Vec4>, Vec<u16>) {
        let polyhedron = &self.model.polyhedron;
        let mut verts: Vec<usize> = polyhedron.vertices.clone().into_iter().collect();
        verts.sort();

        // Accumulate a list of all the positions we know to expect
        let mut positions = verts.iter().fold(vec![], |mut acc, v| {
            acc.push(polyhedron.positions[v]);
            acc
        });

        // Iterate through every face and accumulate a list of indices
        let mut indices = vec![];
        for cycle in &polyhedron.cycles {
            let cycle_indices: Vec<u16> = cycle
                .iter()
                .map(|c| {
                    positions
                        .iter()
                        .position(|&v| v == polyhedron.positions[c])
                        .unwrap() as u16
                })
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
                            positions.len() as u16,
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
                    positions.push(centroid);
                }
            }
        }

        // println!(
        //     "moment_vertices_final: {:?}",
        //     moment_vertices
        //         .iter()
        //         .map(|mv| mv.position)
        //         .collect::<Vec<_>>()
        // );
        println!("indices: {:?}", indices);

        // let barycentric = [Vec4::unit_x(), Vec4::unit_y(), Vec4::unit_z()];
        // for (i, idx) in indices.iter().enumerate() {
        //     moment_vertices[*idx as usize].barycentric = barycentric[i % barycentric.len()];
        //     if *idx > verts.len() as u16 {
        //         moment_vertices[*idx as usize].sides = Vec4::zero();
        //     } else {
        //         moment_vertices[*idx as usize].sides = Vec4::new(1.0, 1.0, 1.0, 1.0);
        //     }
        // }

        (positions.iter().map(|&x| x.into()).collect(), indices)
        //(positions, indices)
    }

    pub fn color_buf(&self) -> (Vec<Vec4>, Vec<u16>) {
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
                    indices.extend(vec![(i % colors.len()) as u16; 3]);
                }
                4 => {
                    indices.extend(vec![(i % colors.len()) as u16; 6]);
                }
                _ => {
                    indices.extend(vec![(i % colors.len()) as u16; cycle.len() * 3]);
                }
            }
        }

        (colors, indices)
    }
    pub fn barycentric_buf(&self) -> (Vec<Vec4>, Vec<u16>) {
        let barycentric: Vec<Vec4> = vec![Vec4::unit_x(), Vec4::unit_y(), Vec4::unit_z()];
        let mut indices = vec![];
        let polyhedron = &self.model.polyhedron;
        for (i, cycle) in polyhedron.cycles.iter().enumerate() {
            match cycle.len() {
                3 => {
                    indices.extend(vec![(i % barycentric.len()) as u16; 3]);
                }
                4 => {
                    indices.extend(vec![(i % barycentric.len()) as u16; 6]);
                }
                _ => {
                    indices.extend(vec![(i % barycentric.len()) as u16; cycle.len() * 3]);
                }
            }
        }

        (barycentric, indices)
    }

    pub fn sides_buf(&self) -> (Vec<Vec4>, Vec<u16>) {
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
