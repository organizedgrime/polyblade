use cgmath::{Matrix, Matrix4};
use gl::types::{GLchar, GLenum, GLfloat, GLint, GLuint};
use glfw::with_c_str;

use crate::verify;

pub struct Shader {
    id: GLuint,
}

impl Shader {
    pub fn new(vs: &str, fs: &str) -> Self {
        let vertex_shader = compile(vs, gl::VERTEX_SHADER);
        let fragment_shader = compile(fs, gl::FRAGMENT_SHADER);
        let program = link(vertex_shader, fragment_shader);
        // mark shaders for deletion after program deletion
        unsafe {
            verify!(gl::DeleteShader(fragment_shader));
            verify!(gl::DeleteShader(vertex_shader));
        }
        Shader { id: program }
    }

    pub fn activate(&self) {
        unsafe { verify!(gl::UseProgram(self.id)) }
    }

    pub fn enable(&self, name: &str, count: GLint) {
        let mut id = 0;
        with_c_str(name, |n| unsafe {
            id = verify!(gl::GetAttribLocation(self.id, n));
        });
        unsafe { verify!(gl::EnableVertexAttribArray(id as GLuint)) }
        let normalized = gl::FALSE;
        let stride = 0;
        unsafe {
            verify!(gl::VertexAttribPointer(
                id as GLuint,
                count,
                gl::FLOAT,
                normalized,
                stride,
                std::ptr::null()
            ))
        }
    }

    pub fn get_uniform(&self, name: &str) -> GLuint {
        with_c_str(name, |name| unsafe {
            verify!(gl::GetUniformLocation(self.id, name) as GLuint)
        })
    }

    pub fn uniform_mat4(&self, id: GLuint, mat: &Matrix4<GLfloat>) {
        unsafe { verify!(gl::UniformMatrix4fv(id as i32, 1, gl::FALSE, mat.as_ptr())) }
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            verify!(gl::DeleteProgram(self.id));
        }
    }
}

fn link(vs: GLuint, fs: GLuint) -> GLuint {
    unsafe {
        let program = verify!(gl::CreateProgram());
        verify!(gl::AttachShader(program, vs));
        verify!(gl::AttachShader(program, fs));
        verify!(gl::LinkProgram(program));

        let mut status = gl::FALSE as GLint;
        verify!(gl::GetProgramiv(program, gl::LINK_STATUS, &mut status));
        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            verify!(gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut len));
            let mut buf = std::vec::from_elem(len as u8 - 1, 0_usize);
            verify!(gl::GetProgramInfoLog(
                program,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar
            ));
            panic!("link_program: {}", String::from_utf8(buf).unwrap());
        }

        program
    }
}

fn compile(src: &str, shader_type: GLenum) -> GLuint {
    unsafe {
        let shader = verify!(gl::CreateShader(shader_type));
        with_c_str(src, |s| {
            verify!(gl::ShaderSource(shader, 1, &s, std::ptr::null()))
        });
        verify!(gl::CompileShader(shader));
        let mut status = gl::FALSE as GLint;
        verify!(gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut status));

        if status != (gl::TRUE as GLint) {
            let mut len = 0;
            verify!(gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut len));
            let mut buf = vec![0_u8; (len) as usize];
            verify!(gl::GetShaderInfoLog(
                shader,
                len,
                std::ptr::null_mut(),
                buf.as_mut_ptr() as *mut GLchar
            ));
            panic!("compile_shader: {}", String::from_utf8(buf).unwrap());
        }
        shader
    }
}
