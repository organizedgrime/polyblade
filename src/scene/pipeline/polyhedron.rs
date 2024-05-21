use std::f32::consts::PI;

use crate::wgpu;
use crate::{polyhedra::PolyGraph, scene::pipeline::Vertex};

use glam::{vec2, vec3, Mat3, Mat4, Quat, Vec3};
use rand::{thread_rng, Rng};

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
