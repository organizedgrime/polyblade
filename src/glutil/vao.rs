use crate::verify;
use gl::types::GLuint;

pub struct Vao {
    id: GLuint,
}

impl Vao {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe { gl::GenVertexArrays(1, &mut id) }
        let vao = Self { id };
        vao.bind();
        vao
    }

    pub fn bind(&self) {
        unsafe { verify!(gl::BindVertexArray(self.id)) }
    }

    pub fn unbind(&self) {
        unsafe { verify!(gl::BindVertexArray(0)) }
    }
}

impl Drop for Vao {
    fn drop(&mut self) {
        unsafe { verify!(gl::DeleteVertexArrays(1, &self.id)) }
    }
}
