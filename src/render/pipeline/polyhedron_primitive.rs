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

    pub fn moment_vertices(&self) -> Vec<MomentVertex> {
        let mut moment_vertices = Vec::new();
        let polyhedron = &self.model.polyhedron;
        // Assume some ordering?
        //let vertices = polyhedron.vertices.iter().collect::<Vec<_>>();
        for i in 0..polyhedron.cycles.len() {
            let face_vertices = &polyhedron.cycles[i];
            let face_moments: Vec<MomentVertex> = face_vertices
                .iter()
                .map(|v| MomentVertex {
                    position: polyhedron.positions[v],
                    color: self.render.picker.palette.colors
                        [v % self.render.picker.palette.colors.len()]
                    .into(),
                })
                .collect();

            let face_moment_vertices = match face_moments.len() {
                3 => face_moments,
                4 => vec![
                    face_moments[0],
                    face_moments[1],
                    face_moments[2],
                    face_moments[2],
                    face_moments[3],
                    face_moments[0],
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
            };
            moment_vertices.extend(face_moment_vertices);
        }

        moment_vertices
    }

    fn face_triangle_positions(&self, face_index: usize) -> Vec<Vec3> {
        let polyhedron = &self.model.polyhedron;
        // The positions of each
        let positions = polyhedron.cycles[face_index]
            .iter()
            .map(|v| polyhedron.positions[v])
            .collect::<Vec<_>>();

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
                let centroid = polyhedron.face_centroid(face_index);
                let n = positions.len();
                (0..n).fold(vec![], |acc, i| {
                    [acc, vec![positions[i], centroid, positions[(i + 1) % n]]].concat()
                })
            }
        }
    }

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
        let mut vertices = Vec::new();
        let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];

        for i in 0..self.model.polyhedron.cycles.len() {
            let sides = self.face_sides_buffer(i);
            let positions = self.face_triangle_positions(i);

            for j in 0..positions.len() {
                let p = positions[j].normalized();
                let b = barycentric[j % barycentric.len()];
                vertices.push(ShapeVertex {
                    normal: Vec4::new(p.x, p.y, p.z, 0.0),
                    sides: Vec4::new(sides[j].x, sides[j].y, sides[j].z, 0.0),
                    barycentric: Vec4::new(b.x, b.y, b.z, 0.0),
                });
            }
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
        let vertex_count = self.model.polyhedron.vertex_count();
        if !storage.has::<Pipeline>() {
            storage.store(Pipeline::new(device, format, target_size, vertex_count));
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
        pipeline.update(device, queue, target_size, vertex_count, &uniforms, self);
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
