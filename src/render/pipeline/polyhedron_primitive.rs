use std::collections::HashMap;

use crate::render::{
    message::ColorMethodMessage,
    pipeline::{MomentVertex, ShapeVertex},
    state::{ModelState, RenderState},
};
use ultraviolet::{Vec3, Vec4};

#[derive(Debug)]
pub struct PolyhedronPrimitive {
    pub model: ModelState,
    pub render: RenderState,
}

impl PolyhedronPrimitive {
    pub fn new(model: ModelState, render: RenderState) -> Self {
        Self { model, render }
    }

    /// All the vertices that will change moment to moment
    pub fn moment_vertices(&self) -> Vec<MomentVertex> {
        let polyhedron = &self.model.polyhedron;
        let colors = &self.render.picker.palette.colors;
        polyhedron.moment_vertices(colors)
    }
}
