use crate::{polyhedra::PolyGraph, wgpu};

use ultraviolet::{Mat3, Mat4, Vec3};

use super::Vertex;

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
#[repr(C)]
pub struct Raw {
    pub(crate) transformation: Mat4,
    normal: Mat3,
    _padding: [f32; 3],
}

impl Raw {
    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        //cube transformation matrix
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        8 => Float32x4,
        //normal rotation matrix
        9 => Float32x3,
        10 => Float32x3,
        11 => Float32x3,
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl From<&Mat4> for Raw {
    fn from(value: &Mat4) -> Self {
        Self {
            transformation: *value,
            normal: Mat3::identity(),
            _padding: [0.0; 3],
        }
    }
}

#[derive(Debug)]
pub struct Descriptor {
    /// Size of the buffer containing only position data
    pub position_buffer_size: u64,
    /// Size of the buffer containing remaining vertex data
    pub vertex_buffer_size: u64,
    /// Number of vertices when we represent the polyhedron as triangles
    pub vertex_triangle_count: u64,
}

impl From<&PolyGraph> for Descriptor {
    fn from(value: &PolyGraph) -> Self {
        let mut vertex_triangle_count = 0;
        for face in value.cycles.iter() {
            match face.len() {
                3 => {
                    vertex_triangle_count += 3;
                }
                4 => {
                    vertex_triangle_count += 6;
                }
                _ => {
                    vertex_triangle_count += 3 * face.len() as u64;
                }
            }
        }

        Self {
            position_buffer_size: std::mem::size_of::<Vec3>() as u64 * vertex_triangle_count,
            vertex_buffer_size: std::mem::size_of::<Vertex>() as u64 * vertex_triangle_count,
            vertex_triangle_count,
        }
    }
}
