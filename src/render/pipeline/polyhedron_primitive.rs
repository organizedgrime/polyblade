use std::collections::HashMap;

use crate::bones::Face;
use crate::render::message::ColorMethodMessage;
use crate::render::state::{ModelState, RenderState};
use iced::widget::canvas::path::lyon_path::geom::euclid::vec3;
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
    pub fn vertices(&self) -> Vec<Vertex> {
        let polyhedron = &self.model.polyhedron;

        // hashmap of polygon length to color
        let colors = &self.render.picker.palette.colors;
        let mut color_map: HashMap<usize, Vec4> = HashMap::new();
        for cycle in &polyhedron.cycles {
            if !color_map.contains_key(&cycle.len()) {
                color_map.insert(cycle.len(), colors[color_map.len() % colors.len()].into());
            }
        }

        let mut vertices = Vec::new();
        let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
        for cycle in &polyhedron.cycles {
            let color = *color_map.get(&cycle.len()).unwrap();

            let sides: Vec4 = match cycle.len() {
                3 => Vec3::new(1.0, 1.0, 1.0),
                4 => Vec3::new(1.0, 0.0, 1.0),
                _ => Vec3::new(0.0, 1.0, 0.0),
            }
            .into();

            let positions: Vec<(usize, Vec3)> = cycle
                .iter()
                .map(|&c| (c, polyhedron.positions[&c]))
                .collect();

            let color_maaap: HashMap<usize, Vec4> = cycle
                .iter()
                .map(|&i| {
                    (
                        i,
                        match self.render.method {
                            ColorMethodMessage::Vertex => colors[i % colors.len()].into(),
                            _ => color,
                        },
                    )
                })
                .collect();

            let triangles: Vec<Vertex> = match cycle.len() {
                3 => positions
                    .iter()
                    .enumerate()
                    .map(|(i, (c, p))| Vertex {
                        position: *p,
                        color: color_maaap[c],
                        barycentric: barycentric[i].into(),
                        sides,
                    })
                    .collect(),
                4 => vec![0usize, 1, 2, 2, 3, 0]
                    .iter()
                    .enumerate()
                    .map(|(i, &j)| Vertex {
                        position: positions[j].1,
                        color: color_maaap[&positions[j].0],
                        barycentric: barycentric[i % barycentric.len()].into(),
                        sides,
                    })
                    .collect(),
                _ => {
                    let centroid: Vec3 = positions.iter().fold(Vec3::zero(), |a, &b| a + b.1)
                        / positions.len() as f32;
                    let centroid_color: Vec4 =
                        cycle.iter().fold(Vec4::zero(), |a, &b| a + color_maaap[&b])
                            / cycle.len() as f32;
                    let mut triangles = vec![];
                    for i in 0..cycle.len() {
                        triangles.extend(vec![
                            Vertex {
                                position: positions[i].1,
                                color: color_maaap[&positions[i].0],
                                barycentric: barycentric[0].into(),
                                sides,
                            },
                            Vertex {
                                position: centroid,
                                color: centroid_color,
                                barycentric: barycentric[1].into(),
                                sides,
                            },
                            Vertex {
                                position: positions[(i + 1) % cycle.len()].1,
                                color: color_maaap[&positions[(i + 1) % cycle.len()].0],
                                barycentric: barycentric[2].into(),
                                sides,
                            },
                        ]);
                    }
                    triangles
                }
            };
            vertices.extend(triangles);
        }

        println!("vertices: {:?}", vertices);

        vertices
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
                line_mode: self.render.method.clone().into(),
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
