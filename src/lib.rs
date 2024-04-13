//#![feature(test)]

mod color;
mod polyhedra;

pub mod prelude {
    pub type V3f = Vector3<f32>;
    use cgmath::Vector3;

    pub use crate::color::*;
    pub use crate::polyhedra::*;
}

//extern crate test;
