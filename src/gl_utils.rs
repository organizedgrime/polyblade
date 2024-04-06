// Draws a simple white triangle
// based on the example from:
// https://github.com/brendanzab/gl-rs/blob/master/gl/examples/triangle.rs

use egui_gl_glfw::gl;
use egui_gl_glfw::gl::types::*;
use std::{mem, ptr, str};

use std::ffi::CString;

use crate::glutil::*;
use crate::prelude::PolyGraph;

#[allow(unconditional_panic)]
const fn illegal_null_in_string() {
    [][0]
}

#[doc(hidden)]
pub const fn validate_cstr_contents(bytes: &[u8]) {
    let mut i = 0;
    while i < bytes.len() {
        if bytes[i] == b'\0' {
            illegal_null_in_string();
        }
        i += 1;
    }
}

macro_rules! cstr {
    ( $s:literal ) => {{
        validate_cstr_contents($s.as_bytes());
        unsafe { std::mem::transmute::<_, &std::ffi::CStr>(concat!($s, "\0")) }
    }};
}

//static VERTEX_DATA: [GLfloat; 9] = [0.0, 0.2, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];

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
            pg: PolyGraph::cube(),
            vao: Vao::new(),
            xyz_vbo: Vbo::new(),
            rgb_vbo: Vbo::new(),
            draw_len: 0,
        }
    }

    pub fn prepare(&mut self, shader: &Shader) {
        let (xyz, rgb, _) = self.pg.triangle_buffers();
        self.draw_len = xyz.len() as i32;

        //let size = std::mem::size_of::<f32>();
        //let rgb_ptr = mem::transmute(&rgb[0]);
        //let rgb_size = (rgb.len() * size) as GLsizeiptr;

        self.vao.bind();
        shader.activate();
        self.xyz_vbo.bind_with_data(&xyz);
        shader.enable("xyz", 3);

        self.rgb_vbo.bind_with_data(&rgb);
        shader.enable("rgb", 3);
        //shader.
    }

    pub fn draw(&self) {
        if self.draw_len > 0 {
            unsafe {
                gl::DrawArrays(gl::TRIANGLES, 0, self.draw_len);
            }
        }
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
