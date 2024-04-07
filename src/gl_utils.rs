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
        let (rgb, bsc, tri) = shape.static_buffers();

        self.vao.bind();
        shader.activate();
        self.vbo.array_bind();

        shader.enable("xyz", gl::FLOAT, 3, 0, 0);

        let stride = std::mem::size_of::<PolyVertex>() as i32;
        shader.enable(
            "rgb",
            gl::FLOAT,
            3,
            stride,
            std::mem::offset_of!(PolyVertex, rgb) as usize,
        );
        shader.enable(
            "bsc",
            gl::FLOAT,
            3,
            stride,
            std::mem::offset_of!(PolyVertex, bsc) as usize,
        );
        shader.enable(
            "tri",
            gl::FLOAT,
            3,
            stride,
            std::mem::offset_of!(PolyVertex, tri) as usize,
        );

        self.vbo.array_data(&[xyz, rgb, bsc, tri].concat());

        self.vao.unbind();
    }

    pub fn draw(&self, shape: &PolyGraph) {
        let xyz = shape.xyz_buffer();
        self.vao.bind();
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
