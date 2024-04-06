// Draws a simple white triangle
// based on the example from:
// https://github.com/brendanzab/gl-rs/blob/master/gl/examples/triangle.rs

use egui_gl_glfw::gl;
use egui_gl_glfw::gl::types::*;
use std::{mem, ptr, str};

use std::ffi::CString;

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
    pub pg: PolyGraph,
    pub vs: GLuint,
    pub fs: GLuint,
    pub program: GLuint,
    pub vao: GLuint,
    pub xyz_vbo: GLuint,
    pub rgb_vbo: GLuint,
}

impl Poly {
    pub fn new() -> Self {
        let vs = compile_shader(VS_SRC, gl::VERTEX_SHADER);
        let fs = compile_shader(FS_SRC, gl::FRAGMENT_SHADER);
        let program = link_program(vs, fs);

        let mut vao = 0;
        let mut xyz_vbo = 0;
        let mut rgb_vbo = 0;
        //let mut bcs_vbo = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut xyz_vbo);
            gl::GenBuffers(1, &mut rgb_vbo);
        }

        println!("va: {vao}, xyz: {xyz_vbo}, rgb: {rgb_vbo}");

        Poly {
            pg: PolyGraph::cube(),
            vs,
            fs,
            program,
            vao,
            xyz_vbo,
            rgb_vbo,
        }
    }

    pub fn draw(&self) {
        let (xyz, rgb, _) = self.pg.triangle_buffers();
        let draw_len: i32 = xyz.len() as i32;

        unsafe {
            let size = std::mem::size_of::<f32>();
            let rgb_ptr = mem::transmute(&rgb[0]);
            let rgb_size = (rgb.len() * size) as GLsizeiptr;

            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.rgb_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, rgb_size, rgb_ptr, gl::STATIC_DRAW);

            let xyz_ptr = mem::transmute(&xyz[0]);
            let xyz_size = (xyz.len() * size) as GLsizeiptr;
            gl::BindVertexArray(self.vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.xyz_vbo);
            gl::BufferData(gl::ARRAY_BUFFER, xyz_size, xyz_ptr, gl::STATIC_DRAW);
        }

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
            //gl::EnableVertexAttribArray(color_attr as GLuint);
            gl::VertexAttribPointer(
                pos_attr as GLuint,
                3,
                gl::FLOAT,
                gl::FALSE as GLboolean,
                0,
                ptr::null(),
            );
            /*
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

impl Drop for Poly {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
            gl::DeleteShader(self.fs);
            gl::DeleteShader(self.vs);
            gl::DeleteBuffers(1, &self.xyz_vbo);
            gl::DeleteVertexArrays(1, &self.vao);
        }
    }
}
