use crate::verify;
use gl::types::{GLsizeiptr, GLuint};

pub struct Vbo {
    id: GLuint,
}

impl Vbo {
    pub fn new() -> Self {
        let mut id = 0;
        unsafe { verify!(gl::GenBuffers(1, &mut id)) }
        Self { id }
    }

    fn bind(&self) {
        unsafe { verify!(gl::BindBuffer(gl::ARRAY_BUFFER, self.id)) }
    }

    pub fn bind_with_data<T>(&self, data: &[T]) {
        self.bind();
        let size = std::mem::size_of::<T>();
        let buf_size = (data.len() * size) as GLsizeiptr;
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
    pub fn drop(&mut self) {
        unsafe { verify!(gl::DeleteBuffers(1, &self.id)) }
    }
}
