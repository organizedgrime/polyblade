use crate::glutil::*;
use crate::prelude::PolyGraph;
use cgmath::Vector3;
use egui_gl_glfw::gl;

pub struct Poly {
    // Graph / Data
    pub vao: Vao,
    pub vbo: Vbo,
}

impl Default for Poly {
    fn default() -> Self {
        Self::new()
    }
}

impl Poly {
    pub fn new() -> Self {
        Poly {
            vao: Vao::new(),
            vbo: Vbo::new(),
        }
    }

    pub fn prepare(&self, shape: &PolyGraph, shader: &Shader) {
        let xyz = shape.xyz_buffer();
        let static_buffer = shape.static_buffer();
        shader.activate();
        self.vao.bind();
        self.vbo.array_bind();
        let s = std::mem::size_of::<V3f>() as usize;
        let start = s * xyz.len() as usize;
        let stride = (s * 3) as i32;
        shader.enable("xyz", 0, 0);
        shader.enable("rgb", stride, start + 0);
        shader.enable("bsc", stride, start + s);
        shader.enable("tri", stride, start + s * 2);
        self.vbo.array_data(&[xyz, static_buffer].concat());
        self.vao.unbind();
    }

    pub fn draw(&self, shape: &PolyGraph) {
        let xyz = shape.xyz_buffer();
        self.vao.bind();
        self.vbo.array_bind();
        self.vbo.array_sub(&xyz);
        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, xyz.len() as i32);
        }
        self.vao.unbind();
    }
}

/*
impl Drop for Poly {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
            self.xyz_vbo.drop();
            self.rgb_vbo.drop();
            self.vao.drop();
        }
    }
}
*/
