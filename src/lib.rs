//#![feature(associated_type_defaults)]
#![feature(let_chains)]

mod color;
mod polyhedra;
mod render;
mod scene;

pub mod prelude {
    pub use crate::color::*;
    pub use crate::polyhedra::*;
    pub use crate::render::*;
    pub use crate::scene::*;

    #[cfg(test)]
    pub fn ids<V: Vertex>(vertices: Vec<V>) -> Vec<VertexId> {
        let mut ids: Vec<VertexId> = vertices.iter().map(|v| v.id()).collect();
        ids.sort();
        ids
    }
}
