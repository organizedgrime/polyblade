//#![feature(test)]

mod color;
mod gl_utils;
mod glutil;
mod polyhedra;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::gl_utils::*;
    pub use crate::glutil::*;
    pub use crate::polyhedra::*;
}

//extern crate test;
