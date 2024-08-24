use crate::wgpu;
use ultraviolet::{Vec3, Vec4};

use super::polyhedron::{self, Transforms};

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub normal: Vec4,
    pub barycentric: Vec4,
    pub sides: Vec4,
    pub color: Vec4,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        // normal
        1 => Float32x4,
        // barycentric
        2 => Float32x4,
        // sides
        3 => Float32x4,
        // color
        4 => Float32x4,
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &Self::ATTRIBS,
        }
    }
}

#[derive(Debug)]
pub struct PolyData {
    pub positions: Vec<Vec3>,
    pub vertices: Vec<Vertex>,
    pub raw: Transforms,
}
