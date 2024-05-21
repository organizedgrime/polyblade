use std::f32::consts::PI;

use crate::wgpu;
use crate::{polyhedra::PolyGraph, scene::pipeline::Vertex};

use glam::{vec2, vec3, Mat3, Mat4, Quat, Vec3};
use rand::{thread_rng, Rng};

/// A single instance of a cube.
#[derive(Debug, Clone)]
pub struct Hedron {
    pub pg: PolyGraph,
    pub rotation: Mat4,
    pub size: f32,
}

impl Default for Hedron {
    fn default() -> Self {
        Self {
            pg: PolyGraph::dodecahedron(),
            rotation: glam::Mat4::ZERO,
            size: 0.1,
        }
    }
}

impl Hedron {
    pub fn new(pg: PolyGraph, size: f32) -> Self {
        Self {
            pg,
            rotation: glam::Mat4::IDENTITY,
            size,
        }
    }

    pub fn update(&mut self, size: f32, time: f32) {
        self.pg.update();
        self.rotation = Mat4::from_rotation_x(time / PI) * Mat4::from_rotation_y(time / PI * 1.1);
        self.size = size;
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
#[repr(C)]
pub struct Raw {
    // todo fix
    pub(crate) transformation: glam::Mat4,
    normal: glam::Mat3,
    _padding: [f32; 3],
}

impl Raw {
    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        //cube transformation matrix
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        //normal rotation matrix
        8 => Float32x3,
        9 => Float32x3,
        10 => Float32x3,
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl Raw {
    pub fn from_pg(rotation: &Mat4) -> Self {
        Self {
            transformation: rotation.clone(),
            normal: Mat3::from_quat(Quat::IDENTITY),
            _padding: [0.0; 3],
        }
    }

    pub fn from_cube(cube: &Hedron) -> Raw {
        Raw {
            transformation: cube.rotation,
            normal: glam::Mat3::from_quat(Quat::IDENTITY),
            _padding: [0.0; 3],
        }
    }
}

impl PolyGraph {
    pub fn vertices2(&self) -> Vec<Vertex> {
        let (rgb, bsc, tri) = self.static_buffer();
        let ver = self.xyz_buffer();
        let mut x = Vec::new();

        for i in 0..ver.len() {
            x.push(Vertex {
                position: ver[i],
                normal: bsc[i],
                color: rgb[i],
            });
        }
        x
    }
}
