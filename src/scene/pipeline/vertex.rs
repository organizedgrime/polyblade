use ultraviolet::{Mat4, Vec3, Vec4};

#[derive(Debug, Copy, Clone, bytemuck::Pod, bytemuck::Zeroable)]
#[repr(C)]
pub struct Vertex {
    pub normal: Vec4,
    pub barycentric: Vec4,
    pub sides: Vec4,
    pub color: Vec4,
}

#[derive(Debug)]
pub struct PolyData {
    pub positions: Vec<Vec3>,
    pub vertices: Vec<Vertex>,
    pub transform: Mat4,
}
