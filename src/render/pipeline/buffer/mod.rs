mod types;
use iced::widget::shader::wgpu::{self, RenderPass};
pub use types::*;

pub enum BufferKind {
    Uniform,
    Index,
    Vertex,
}

impl Into<wgpu::BufferUsages> for BufferKind {
    fn into(self) -> wgpu::BufferUsages {
        match self {
            BufferKind::Uniform => wgpu::BufferUsages::UNIFORM | wgpu::BufferUsages::COPY_DST,
            BufferKind::Index => wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
            BufferKind::Vertex => wgpu::BufferUsages::VERTEX | wgpu::BufferUsages::COPY_DST,
        }
    }
}

// A custom buffer container for dynamic resizing.
pub struct Buffer {
    pub raw: wgpu::Buffer,
    label: &'static str,
    usage: wgpu::BufferUsages,
    size_of_type: u64,
    pub count: u32,
}
/* // A custom buffer container for dynamic resizing.
pub struct IndexBuffer {
    pub data_raw: wgpu::Buffer,
    pub index_raw: wgpu::Buffer,
    label: &'static str,
    usage: wgpu::BufferUsages,
    size_of_type: u64,
    pub data_count: u32,
    pub index_count: u32,
} */

impl Buffer {
    pub fn new<T>(device: &wgpu::Device, label: &'static str, kind: BufferKind) -> Self {
        let size_of_type = std::mem::size_of::<T>() as u64;
        let usage = kind.into();
        Self {
            raw: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: size_of_type * 1,
                usage,
                mapped_at_creation: false,
            }),
            label,
            usage,
            size_of_type,
            count: 1,
        }
    }

    pub fn resize(&mut self, device: &wgpu::Device, new_count: u32) {
        self.raw = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(self.label),
            size: self.size_of_type * new_count as u64,
            usage: self.usage,
            mapped_at_creation: false,
        });
        self.count = new_count;
    }

    pub fn write_data<T: bytemuck::Pod>(&mut self, queue: &wgpu::Queue, data: &T) {
        queue.write_buffer(&self.raw, 0, bytemuck::bytes_of(data));
    }

    pub fn write_vec<T: bytemuck::Pod>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        data: Vec<T>,
    ) {
        let count = data.len() as u32;
        // Resize the index buffer if necessary
        if self.count != count {
            self.resize(device, count);
        }
        // Write to the buffers
        queue.write_buffer(&self.raw, 0, bytemuck::cast_slice(&data));
    }
}

/* impl IndexBuffer {
    pub fn new<T>(device: &wgpu::Device, label: &'static str, usage: wgpu::BufferUsages) -> Self {
        let size_of_type = std::mem::size_of::<T>() as u64;
        Self {
            data_raw: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(label),
                size: size_of_type * 1,
                usage,
                mapped_at_creation: false,
            }),
            index_raw: device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{}_index", label)),
                size: std::mem::size_of::<u32>() as u64 * 1,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            }),
            label,
            usage,
            size_of_type,
            data_count: 1,
            index_count: 1,
        }
    }

    pub fn write<T: bytemuck::Pod>(
        &mut self,
        device: &wgpu::Device,
        queue: &wgpu::Queue,
        buf: (Vec<T>, Vec<u32>),
    ) {
        let (data, indices) = buf;
        // Resize the index buffer if necessary
        if indices.len() as u32 != self.index_count {
            self.index_count = indices.len() as u32;
            self.index_raw = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{}_index", self.label)),
                size: std::mem::size_of::<u32>() as u64 * self.index_count as u64,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
        // Resize the data buffer if necessary
        if data.len() as u32 != self.data_count {
            self.data_count = data.len() as u32;
            self.data_raw = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(self.label),
                size: self.size_of_type * self.data_count as u64,
                usage: self.usage,
                mapped_at_creation: false,
            });
        }
        // Write to the buffers
        queue.write_buffer(&self.data_raw, 0, bytemuck::cast_slice(&data));
        queue.write_buffer(&self.index_raw, 0, bytemuck::cast_slice(&indices));
    }

    // pub fn draw<'pass>(&self, pass: &mut RenderPass<'pass>) {
    //     RenderPass::set_vertex_buffer(pass, 0, self.data_raw.slice(..));
    //     RenderPass::set_index_buffer(pass, self.index_raw.slice(..), wgpu::IndexFormat::Uint16);
    //     RenderPass::draw_indexed(pass, 0..self.index_count as u32, 0, 0..1);
    // }

    // pub fn resize_data(&mut self, device: &wgpu::Device, new_count: u64) {
    //     self.data_raw = device.create_buffer(&wgpu::BufferDescriptor {
    //         label: Some(self.label),
    //         size: self.size_of_type * new_count,
    //         usage: self.usage,
    //         mapped_at_creation: false,
    //     });
    //     self.data_count = new_count;
    // }
    //
    // pub fn resize_index(&mut self, device: &wgpu::Device, new_count: u64) {
    //     self.index_raw = device.create_buffer(&wgpu::BufferDescriptor {
    //         label: Some(&format!("{}_index", self.label)),
    //         size: std::mem::size_of::<u16>() as u64 * new_count,
    //         usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
    //         mapped_at_creation: false,
    //     });
    //     self.index_count = new_count;
    // }
} */
