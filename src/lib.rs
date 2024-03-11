//#![feature(associated_type_defaults)]

mod color;
mod polyhedra;
mod render;
mod scene;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::polyhedra::*;
    pub use crate::render::*;
    pub use crate::scene::*;
}
