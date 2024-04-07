use crate::verify;
use gl::types::{GLsizeiptr, GLuint};

pub struct Vbo {
    id: GLuint,
}

impl Default for Vbo {
    fn default() -> Self {
        Self::new()
    }
}

impl Vbo {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe { verify!(gl::GenBuffers(1, &mut id)) }
        Self { id }
    }

    pub fn array_bind(&self) {
        unsafe { verify!(gl::BindBuffer(gl::ARRAY_BUFFER, self.id)) }
    }

    pub fn array_data<T>(&self, data: &[T]) {
        self.array_bind();
        let buf_size = std::mem::size_of_val(data) as GLsizeiptr;
        if !data.is_empty() {
            unsafe {
                let data_ptr = std::mem::transmute(&data[0]);
                verify!(gl::BufferData(
                    gl::ARRAY_BUFFER,
                    buf_size,
                    data_ptr,
                    gl::STATIC_DRAW,
                ));
            }
        }
    }

    pub fn array_sub<T>(&self, data: &[T]) {
        self.array_bind();
        let buf_size = std::mem::size_of_val(data) as GLsizeiptr;
        unsafe {
            let data_ptr = std::mem::transmute(&data[0]);
            verify!(gl::BufferSubData(gl::ARRAY_BUFFER, 0, buf_size, data_ptr));
        }
    }
}

impl Drop for Vbo {
    fn drop(&mut self) {
        unsafe { verify!(gl::DeleteBuffers(1, &self.id)) }
    }
}
