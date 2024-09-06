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
