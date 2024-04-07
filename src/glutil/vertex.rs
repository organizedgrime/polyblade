use cgmath::Vector3;

pub type V3f = Vector3<f32>;

pub struct PolyVertex {
    pub rgb: Vector3<f32>,
    pub bsc: Vector3<f32>,
    pub tri: Vector3<f32>,
}
