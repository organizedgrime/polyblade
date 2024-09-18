use std::collections::HashMap;

use crate::bones::Face;
use crate::render::message::ColorMethodMessage;
use crate::render::state::{ModelState, RenderState};
use iced::widget::canvas::path::lyon_path::geom::euclid::vec3;
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

    pub fn surface_area(&self, face_index: usize) -> f32 {
        let positions: Vec<Vec3> = self.model.polyhedron.cycles[face_index]
            .iter()
            .map(|i| self.model.polyhedron.positions[i])
            .collect();
        let mut area = 0.0;
        for i in 0..positions.len() / 3 {
            let j = i * 3;
            let a = (positions[j] - positions[j + 1]).mag();
            let b = (positions[j + 1] - positions[j + 2]).mag();
            let c = (positions[j + 2] - positions[j]).mag();
            let s = (a + b + c) / 2.0;
            area += (s * (s - a) * (s - b) * (s - c)).sqrt();
        }
        println!("area for face {face_index} is {area}");
        area
    }

    /// All the vertices that will change moment to moment
    pub fn moment_vertices(&self) -> Vec<MomentVertex> {
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
        for cycle in &polyhedron.cycles {
            let color = *color_map.get(&cycle.len()).unwrap();

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

            let triangles: Vec<MomentVertex> = match cycle.len() {
                3 => positions
                    .iter()
                    .map(|(c, p)| MomentVertex {
                        position: *p,
                        color: color_maaap[c],
                    })
                    .collect(),
                4 => vec![0usize, 1, 2, 2, 3, 0]
                    .iter()
                    .map(|&j| MomentVertex {
                        position: positions[j].1,
                        color: color_maaap[&positions[j].0],
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
                            MomentVertex {
                                position: positions[i].1,
                                color: color_maaap[&positions[i].0],
                            },
                            MomentVertex {
                                position: centroid,
                                color: centroid_color,
                            },
                            MomentVertex {
                                position: positions[(i + 1) % cycle.len()].1,
                                color: color_maaap[&positions[(i + 1) % cycle.len()].0],
                            },
                        ]);
                    }
                    triangles
                }
            };

            /* println!(
                "this cycle had len {} and now has {} moment vertices",
                cycle.len(),
                triangles.len()
            ); */
            vertices.extend(triangles);
        }

        vertices
    }

    pub fn shape_vertices(&self) -> Vec<ShapeVertex> {
        let polyhedron = &self.model.polyhedron;
        let mut vertices = Vec::new();
        let barycentric = [Vec3::unit_x(), Vec3::unit_y(), Vec3::unit_z()];
        for cycle in &polyhedron.cycles {
            let sides: Vec4 = match cycle.len() {
                3 => Vec3::new(1.0, 1.0, 1.0),
                4 => Vec3::new(1.0, 1.0, 1.0),
                _ => Vec3::new(1.0, 1.0, 1.0),
            }
            .into();

            let b_shapes: Vec<ShapeVertex> = barycentric
                .iter()
                .map(|&b| ShapeVertex {
                    barycentric: b.into(),
                    sides,
                })
                .collect();

            let cycle_shapes = match cycle.len() {
                3 => b_shapes.clone(),
                4 => (0..6)
                    .into_iter()
                    .map(|i| ShapeVertex {
                        barycentric: barycentric[i % 3].into(),
                        sides,
                    })
                    .collect(),
                _ => vec![b_shapes; cycle.len()].concat(),
            };

            println!(
                "this cycle had len {} and now has {} shape vertices",
                cycle.len(),
                cycle_shapes.len()
            );

            vertices.extend(cycle_shapes);
        }
        println!("{vertices:?}");

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
