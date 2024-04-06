use crate::glutil::*;
use crate::prelude::PolyGraph;
use egui_gl_glfw::gl;

pub struct Poly {
    // Graph / Data
    pub vao: Vao,
    pub xyz_vbo: Vbo,
    pub rgb_vbo: Vbo,
    pub bsc_vbo: Vbo,
    pub tri_vbo: Vbo,
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
            xyz_vbo: Vbo::new(),
            rgb_vbo: Vbo::new(),
            bsc_vbo: Vbo::new(),
            tri_vbo: Vbo::new(),
        }
    }

    pub fn prepare(&self, shader: &Shader) {
        self.vao.bind();
        shader.activate();
        self.xyz_vbo.bind();
        shader.enable("xyz", 3);
        self.rgb_vbo.bind();
        shader.enable("rgb", 3);
        self.bsc_vbo.bind();
        shader.enable("bsc", 3);
        self.tri_vbo.bind();
        shader.enable("tri", 3);
        self.vao.unbind();
    }

    pub fn draw(&self, shape: &PolyGraph) {
        let (xyz, rgb, bsc, tri) = shape.triangle_buffers();
        let draw_len = xyz.len() as i32;
        self.vao.bind();
        self.xyz_vbo.bind_with_data(&xyz);
        self.rgb_vbo.bind_with_data(&rgb);
        self.bsc_vbo.bind_with_data(&bsc);
        self.tri_vbo.bind_with_data(&tri);
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
