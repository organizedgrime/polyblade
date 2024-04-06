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

fn compile_shader(src: &str, ty: GLenum) -> GLuint {
    let shader = unsafe { gl::CreateShader(ty) };

    let c_str = CString::new(src.as_bytes()).unwrap();
    unsafe {
        gl::ShaderSource(shader, 1, &c_str.as_ptr(), core::ptr::null());
        gl::CompileShader(shader);
    }

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status);
    }

    if status != (gl::TRUE as GLint) {
        let mut len = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = vec![0; len as usize];

        unsafe {
            gl::GetShaderInfoLog(
                shader,
                len,
                core::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            core::str::from_utf8(&buf).expect("ShaderInfoLog not valid utf8")
        );
    }

    shader
}

fn link_program(vs: GLuint, fs: GLuint) -> GLuint {
    let program = unsafe { gl::CreateProgram() };

    unsafe {
        gl::AttachShader(program, vs);
        gl::AttachShader(program, fs);
        gl::LinkProgram(program);
    }

    let mut status = gl::FALSE as GLint;
    unsafe {
        gl::GetProgramiv(program, gl::LINK_STATUS, &mut status);
    }

    if status != (gl::TRUE as GLint) {
        let mut len: GLint = 0;
        unsafe {
            gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len);
        }

        let mut buf = vec![0; len as usize];

        unsafe {
            gl::GetProgramInfoLog(
                program,
                len,
                core::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar,
            );
        }

        panic!(
            "{}",
            core::str::from_utf8(&buf).expect("ProgramInfoLog not valid utf8")
        );
    }

    program
}

const VS_SRC: &str = "
#version 150
in vec3 xyz;
in vec3 rgb;

void main() {
    gl_Position = vec4(xyz, 1.0);
}";

const FS_SRC: &str = "
#version 150
out vec4 out_color;

void main() {
    out_color = vec4(1.0, 1.0, 1.0, 1.0);
}";

//static VERTEX_DATA: [GLfloat; 9] = [0.0, 0.2, 0.0, 0.5, -0.5, 0.0, -0.5, -0.5, 0.0];

pub struct Poly {
    // Graph / Data
    pub pg: PolyGraph,
    // Shaders
    pub vs: GLuint,
    pub fs: GLuint,

    // Program
    pub program: GLuint,

    pub vao: Vao,
    pub xyz_vbo: Vbo,
    pub rgb_vbo: Vbo,
}

impl Poly {
    pub fn new() -> Self {
        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        Poly {
            pg: PolyGraph::cube(),
            vs,
            fs,
            program,
            vao: Vao::new(),
            xyz_vbo: Vbo::new(),
            rgb_vbo: Vbo::new(),
        }
    }

    pub fn draw(&self) {
        let (xyz, rgb, _) = self.pg.triangle_buffers();
        let draw_len: i32 = xyz.len() as i32;

        //let size = std::mem::size_of::<f32>();
        //let rgb_ptr = mem::transmute(&rgb[0]);
        //let rgb_size = (rgb.len() * size) as GLsizeiptr;

        self.vao.bind();
        self.xyz_vbo.bind_with_data(&xyz);

        unsafe {
            gl::UseProgram(self.program);
        }

        let c_out_color = cstr!("out_color");
        unsafe {
            gl::BindFragDataLocation(self.program, 0, c_out_color.as_ptr());
        }

        let c_xyz = cstr!("xyz");
        //let c_color = cstr!("color");
        let pos_attr = unsafe { gl::GetAttribLocation(self.program, c_xyz.as_ptr()) };
        //let color_attr = unsafe { gl::GetAttribLocation(self.program, c_color.as_ptr()) };

        unsafe {
            gl::EnableVertexAttribArray(pos_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            /*
            gl::EnableVertexAttribArray(color_attr as GLuint);
            gl::VertexAttribPointer(
                color_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            */
        }

        unsafe {
            gl::DrawArrays(gl::TRIANGLES, 0, draw_len);
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
