use cgmath::Vector3;

pub struct PolyVertex {
    pub xyz: Vector3<f32>,
    pub rgb: Vector3<f32>,
    pub bsc: Vector3<f32>,
    pub tri: Vector3<f32>,
}
