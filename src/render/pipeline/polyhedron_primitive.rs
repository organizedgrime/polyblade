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
            for i in 0..cycle.len() {
                let triangle = vec![
                    // Before
                    verts.iter().position(|&v| v == cycle[i]).unwrap() as u16,
                    // Centroid index
                    moment_vertices.len() as u16,
                    // After
                    verts
                        .iter()
                        .position(|&v| v == cycle[(i + 1) % cycle.len()])
                        .unwrap() as u16,
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
        println!("moment_vertices_final: {:?}", moment_vertices);
        println!("indices: {:?}", indices);

        (moment_vertices, indices)
    }

    /*
        pub fn positions(&self) -> Vec<MomentVertex> {
            let mut kv = vec![];

            // println!("areas: {:?}", areas);

            match self.render.method {
                ColorMethodMessage::Vertex => {
                    (0..self.model.polyhedron.cycles.len()).fold(Vec::new(), |mut acc, i| {
                        for position in self.face_triangle_positions(i) {
                            let color: Vec4 = self.render.picker.palette.colors
                                [acc.len() % self.render.picker.palette.colors.len()]
                            .into();
                            acc.push(MomentVertex { position, color });
                        }
                        acc
                    })
                }
                ColorMethodMessage::Edge => {
                    (0..self.model.polyhedron.cycles.len()).fold(Vec::new(), |mut acc, i| {
                        for position in self.face_triangle_positions(i) {
                            let color: Vec4 = self.render.picker.palette.colors
                                [i % self.render.picker.palette.colors.len()]
                            .into();
                            acc.push(MomentVertex { position, color });
                        }
                        acc
                    })
                }
                ColorMethodMessage::Polygon => todo!(),
                ColorMethodMessage::Face => {
                    let areas: Vec<f32> =
                        (0..self.model.polyhedron.cycles.len()).fold(Vec::new(), |mut acc, i| {
                            let positions = self.face_triangle_positions(i);
                            let mut area = 0.0;
                            for i in 0..positions.len() / 3 {
                                let j = i * 3;
                                let a = (positions[j] - positions[j + 1]).mag();
                                let b = (positions[j + 1] - positions[j + 2]).mag();
                                let c = (positions[j + 2] - positions[j]).mag();
                                let s = (a + b + c) / 2.0;
                                area += (s * (s - a) * (s - b) * (s - c)).sqrt();
                            }
                            let mut log = ((area * area).log10() * 20.0).abs();
                            if log.is_nan() || log.is_infinite() {
                                log = 0.0;
                            }
                            kv.push((positions, log));
                            acc.push(log);
                            acc
                        });
                    let clusters =
                        ckmeans(&areas[..], self.render.picker.colors as u8).unwrap_or(vec![areas]);
                    kv.into_iter().fold(vec![], |acc, (positions, approx)| {
                        for (i, cluster) in clusters.iter().enumerate() {
                            if cluster.contains(&approx) {
                                let color = self.render.picker.palette.colors
                                    [i % self.render.picker.palette.colors.len()];
                                return [
                                    acc,
                                    positions
                                        .into_iter()
                                        .map(|p| MomentVertex {
                                            position: p,
                                            color: color.into(),
                                        })
                                        .collect(),
                                ]
                                .concat();
                            }
                        }

                        acc
                    })
                }
            }
        }
    */

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
        let mut vertices = Vec::new();
        for x in 0..x.len() {
            vertices.push(ShapeVertex {
                normal: Vec4::new(1.0, 1.0, 1.0, 0.0),
                sides: Vec4::new(1.0, 1.0, 1.0, 0.0),
                barycentric: Vec4::new(1.0, 1.0, 1.0, 0.0),
            });
        }

        vertices
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
