use std::ptr::eq;

use crate::render::message::ColorMethodMessage;
use crate::render::state::{ModelState, RenderState};
use ckmeans::ckmeans;
use iced::widget::shader::{self, wgpu};
use iced::{Rectangle, Size};
use ultraviolet::{Vec3, Vec4};

use super::{AllUniforms, FragUniforms, ModelUniforms, MomentVertex, Pipeline, ShapeVertex};

#[derive(Debug)]
pub struct PolyhedronPrimitive {
    pub model: ModelState,
    pub render: RenderState,
}

impl PolyhedronPrimitive {
    pub fn new(model: ModelState, render: RenderState) -> Self {
        Self { model, render }
    }

    /* pub fn indices(&self) -> Vec<u16> {
        //let mut vertices = self.model.polyhedron.vertices.iter().collect::<Vec<_>>();
        //vertices.sort();

        self.model
            .polyhedron
            .cycles
            .iter()
            .map(|cycle| {
                cycle
                    .iter()
                    .map(|c| vertices.iter().position(|&v| v == c).unwrap() as u16)
                    .collect()
            })
            .fold(vec![], |acc, indices| match indices.len() {
                3 => indices,
                4 => vec![
                    indices[0], indices[1], indices[2], indices[2], indices[3], indices[0],
                ],
                _ => {
                    let centroid_color = face_moments
                        .iter()
                        .fold(Vec4::zero(), |acc, fm| acc + fm.color)
                        / face_moments.len() as f32;

                    let centroid = MomentVertex {
                        position: polyhedron.face_centroid(i),
                        color: centroid_color,
                    };

                    (0..face_moments.len()).fold(vec![], |acc, j| {
                        [
                            acc,
                            vec![
                                face_moments[j],
                                centroid,
                                face_moments[(j + 1) % face_moments.len()],
                            ],
                        ]
                        .concat()
                    })
                }
            })
    } */

    /// All the vertices that will change moment to moment
    pub fn moment_vertices(&self) -> (Vec<MomentVertex>, Vec<u16>) {
        let polyhedron = &self.model.polyhedron;
        let mut verts: Vec<usize> = polyhedron.vertices.clone().into_iter().collect();
        verts.sort();

        // Accumulate a list of all the positions we know to expect
        let mut moment_vertices = verts.iter().fold(vec![], |mut acc, v| {
            acc.push(MomentVertex {
                position: polyhedron.positions[v],
                color: self.render.picker.palette.colors
                    [v % self.render.picker.palette.colors.len()]
                .into(),
            });
            acc
        });

        println!("moment_vertices: {:?}", moment_vertices);

        // Iterate through every face and accumulate a list of indices
        let mut indices = vec![];
        for cycle in &polyhedron.cycles {
            let cycle_indices: Vec<u16> = cycle
                .iter()
                .map(|c| verts.iter().position(|v| v == c).unwrap() as u16)
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
                            moment_vertices.len() as u16,
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
                    moment_vertices.push(MomentVertex {
                        position: centroid,
                        color: Vec4::new(1.0, 1.0, 1.0, 1.0),
                    })
                }
            }
        }

        println!(
            "moment_vertices_final: {:?}",
            moment_vertices
                .iter()
                .map(|mv| mv.position)
                .collect::<Vec<_>>()
        );
        println!("indices: {:?}", indices);

        (moment_vertices, indices)
    }

    pub fn face_sides_buffer(&self, face_index: usize) -> Vec<Vec3> {
        let positions = self.model.polyhedron.face_positions(face_index);
        let n = positions.len();
        match n {
            3 => vec![Vec3::new(1.0, 1.0, 1.0); 3],
            4 => vec![Vec3::new(1.0, 0.0, 1.0); 6],
            _ => vec![Vec3::new(0.0, 1.0, 0.0); n * 3],
        }
    }

    pub fn vertices(&self) -> Vec<ShapeVertex> {
        let (x, _) = self.moment_vertices();
        vec![
            ShapeVertex {
                normal: Vec4::new(1.0, 1.0, 1.0, 0.0),
                sides: Vec4::new(1.0, 1.0, 1.0, 0.0),
                barycentric: Vec4::new(1.0, 1.0, 1.0, 0.0),
            };
            x.len()
        ]
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
