use crate::wgpu;
use glam::Vec3;

use super::polyhedron;

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub normal: Vec3,
    pub barycentric: Vec3,
    pub sides: Vec3,
    pub color: Vec3,
}

impl Vertex {
    const ATTRIBS: [wgpu::VertexAttribute; 4] = wgpu::vertex_attr_array![
        // normal
        1 => Float32x3,
        // barycentric
        2 => Float32x3,
        // sides
        3 => Float32x3,
        // color
        4 => Float32x3,
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
    pub raw: polyhedron::Raw,
}
