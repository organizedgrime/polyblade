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

    pub fn prepare(&self, shader: &Shader) {
        self.vao.bind();
        shader.activate();
        self.vbo.bind();
        let stride = std::mem::size_of::<PolyVertex>() as i32;
        shader.enable(
            "xyz",
            3,
            stride,
            std::mem::offset_of!(PolyVertex, xyz) as usize,
        );
        shader.enable(
            "rgb",
            3,
            stride,
            std::mem::offset_of!(PolyVertex, rgb) as usize,
        );
        shader.enable(
            "bsc",
            3,
            stride,
            std::mem::offset_of!(PolyVertex, bsc) as usize,
        );
        shader.enable(
            "tri",
            3,
            stride,
            std::mem::offset_of!(PolyVertex, tri) as usize,
        );
        self.vao.unbind();
    }

    pub fn draw(&self, shape: &PolyGraph) {
        let buffer = shape.triangle_buffers();
        let draw_len = buffer.len() as i32;
        self.vao.bind();
        self.vbo.bind_with_data(&buffer);
        if draw_len > 0 {
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, draw_len);
            }
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
