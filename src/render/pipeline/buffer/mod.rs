mod types;
use iced::widget::shader::wgpu;
pub use types::*;

// A custom buffer container for dynamic resizing.
pub struct Buffer {
    pub raw: wgpu::Buffer,
    label: &'static str,
    usage: wgpu::BufferUsages,
    size_of_type: u64,
    pub count: u64,
}
// A custom buffer container for dynamic resizing.
pub struct IndexBuffer {
    pub data_raw: wgpu::Buffer,
    pub index_raw: wgpu::Buffer,
    label: &'static str,
    usage: wgpu::BufferUsages,
    size_of_type: u64,
    pub data_count: u64,
    pub index_count: u64,
}

impl Buffer {
    pub fn new<T>(device: &wgpu::Device, label: &'static str, usage: wgpu::BufferUsages) -> Self {
        let size_of_type = std::mem::size_of::<T>() as u64;
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

    pub fn resize(&mut self, device: &wgpu::Device, new_count: u64) {
        self.raw = device.create_buffer(&wgpu::BufferDescriptor {
            label: Some(self.label),
            size: self.size_of_type * new_count,
            usage: self.usage,
            mapped_at_creation: false,
        });
        self.count = new_count;
    }
}

impl IndexBuffer {
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
                size: std::mem::size_of::<u16>() as u64 * 1,
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

    pub fn resize(&mut self, device: &wgpu::Device, data_count: usize, index_count: usize) {
        if index_count as u64 != self.index_count {
            self.data_count = data_count as u64;
            self.data_raw = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(self.label),
                size: self.size_of_type * self.data_count,
                usage: self.usage,
                mapped_at_creation: false,
            });
            self.index_count = index_count as u64;
            self.index_raw = device.create_buffer(&wgpu::BufferDescriptor {
                label: Some(&format!("{}_index", self.label)),
                size: std::mem::size_of::<u16>() as u64 * self.index_count,
                usage: wgpu::BufferUsages::INDEX | wgpu::BufferUsages::COPY_DST,
                mapped_at_creation: false,
            });
        }
    }

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
}
