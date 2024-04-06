use crate::glutil::*;
use crate::prelude::PolyGraph;
use egui_gl_glfw::gl;

pub struct Poly {
    // Graph / Data
    pub pg: PolyGraph,
    pub vao: Vao,
    pub xyz_vbo: Vbo,
    pub rgb_vbo: Vbo,
    draw_len: i32,
}

impl Poly {
    pub fn new() -> Self {
        Poly {
            pg: PolyGraph::tetrahedron(),
            vao: Vao::new(),
            xyz_vbo: Vbo::new(),
            rgb_vbo: Vbo::new(),
            draw_len: 0,
        }
    }

    pub fn prepare(&mut self, shader: &Shader) {
        let (xyz, rgb, _) = self.pg.triangle_buffers();
        self.draw_len = xyz.len() as i32;
        self.vao.bind();
        shader.activate();
        self.xyz_vbo.bind_with_data(&xyz);
        shader.enable("xyz", 3);

        self.rgb_vbo.bind_with_data(&rgb);
        shader.enable("rgb", 3);
    }

    pub fn draw(&self) {
        if self.draw_len > 0 {
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, self.draw_len);
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
