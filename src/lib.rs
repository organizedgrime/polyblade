//#![feature(test)]

mod color;
mod gl_utils;
mod polyhedra;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::gl_utils::*;
    pub use crate::polyhedra::*;
}

//extern crate test;
