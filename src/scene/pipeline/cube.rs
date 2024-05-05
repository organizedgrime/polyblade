use std::f32::consts::PI;

use crate::scene::pipeline::Vertex;
use crate::wgpu;

use glam::{vec2, vec3, Mat4, Quat, Vec3};
use rand::{thread_rng, Rng};

/// A single instance of a cube.
#[derive(Debug, Clone)]
pub struct Cube {
    pub rotation: Mat4,
    pub position: Vec3,
    pub size: f32,
}

impl Default for Cube {
    fn default() -> Self {
        Self {
            rotation: glam::Mat4::ZERO,
            position: glam::Vec3::ZERO,
            size: 0.1,
        }
    }
}

impl Cube {
    pub fn new(size: f32, origin: Vec3) -> Self {
        let rnd = thread_rng().gen_range(0.0..=1.0f32);

        Self {
            rotation: glam::Mat4::IDENTITY,
            position: origin + Vec3::new(0.1, 0.1, 0.1),
            size,
        }
    }

    pub fn update(&mut self, size: f32, time: f32) {
        self.rotation = Mat4::from_rotation_x(time / PI) * Mat4::from_rotation_y(time / PI * 1.1);
        self.size = size;
    }
}

#[derive(Clone, Copy, bytemuck::Pod, bytemuck::Zeroable, Debug)]
#[repr(C)]
pub struct Raw {
    // todo fix
    pub(crate) transformation: glam::Mat4,
    normal: glam::Mat3,
    _padding: [f32; 3],
}

impl Raw {
    const ATTRIBS: [wgpu::VertexAttribute; 7] = wgpu::vertex_attr_array![
        //cube transformation matrix
        4 => Float32x4,
        5 => Float32x4,
        6 => Float32x4,
        7 => Float32x4,
        //normal rotation matrix
        8 => Float32x3,
        9 => Float32x3,
        10 => Float32x3,
    ];

    pub fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Self>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}

impl Raw {
    pub fn from_cube(cube: &Cube) -> Raw {
        Raw {
            transformation: cube.rotation,
            normal: glam::Mat3::from_quat(Quat::IDENTITY),
            _padding: [0.0; 3],
        }
    }

    pub fn vertices() -> [Vertex; 36] {
        [
            //face 1
            Vertex {
                position: vec3(-0.5, -0.5, -0.5),
                normal: vec3(0.0, 0.0, -1.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, -0.5),
                normal: vec3(0.0, 0.0, -1.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, -0.5),
                normal: vec3(0.0, 0.0, -1.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, -0.5),
                normal: vec3(0.0, 0.0, -1.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(-0.5, 0.5, -0.5),
                normal: vec3(0.0, 0.0, -1.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(-0.5, -0.5, -0.5),
                normal: vec3(0.0, 0.0, -1.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            //face 2
            Vertex {
                position: vec3(-0.5, -0.5, 0.5),
                normal: vec3(0.0, 0.0, 1.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, 0.5),
                normal: vec3(0.0, 0.0, 1.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, 0.5),
                normal: vec3(0.0, 0.0, 1.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, 0.5),
                normal: vec3(0.0, 0.0, 1.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, 0.5, 0.5),
                normal: vec3(0.0, 0.0, 1.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, -0.5, 0.5),
                normal: vec3(0.0, 0.0, 1.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            //face 3
            Vertex {
                position: vec3(-0.5, 0.5, 0.5),
                normal: vec3(-1.0, 0.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, 0.5, -0.5),
                normal: vec3(-1.0, 0.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, -0.5, -0.5),
                normal: vec3(-1.0, 0.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, -0.5, -0.5),
                normal: vec3(-1.0, 0.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, -0.5, 0.5),
                normal: vec3(-1.0, 0.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, 0.5, 0.5),
                normal: vec3(-1.0, 0.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            //face 4
            Vertex {
                position: vec3(0.5, 0.5, 0.5),
                normal: vec3(1.0, 0.0, 0.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, -0.5),
                normal: vec3(1.0, 0.0, 0.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, -0.5),
                normal: vec3(1.0, 0.0, 0.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, -0.5),
                normal: vec3(1.0, 0.0, 0.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, 0.5),
                normal: vec3(1.0, 0.0, 0.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, 0.5),
                normal: vec3(1.0, 0.0, 0.0),
                color: vec3(0.0, 0.0, 1.0),
            },
            //face 5
            Vertex {
                position: vec3(-0.5, -0.5, -0.5),
                normal: vec3(0.0, -1.0, 0.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, -0.5),
                normal: vec3(0.0, -1.0, 0.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, 0.5),
                normal: vec3(0.0, -1.0, 0.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, -0.5, 0.5),
                normal: vec3(0.0, -1.0, 0.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, -0.5, 0.5),
                normal: vec3(0.0, -1.0, 0.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, -0.5, -0.5),
                normal: vec3(0.0, -1.0, 0.0),
                color: vec3(0.0, 1.0, 0.0),
            },
            //face 6
            Vertex {
                position: vec3(-0.5, 0.5, -0.5),
                normal: vec3(0.0, 1.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, -0.5),
                normal: vec3(0.0, 1.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, 0.5),
                normal: vec3(0.0, 1.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(0.5, 0.5, 0.5),
                normal: vec3(0.0, 1.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, 0.5, 0.5),
                normal: vec3(0.0, 1.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
            Vertex {
                position: vec3(-0.5, 0.5, -0.5),
                normal: vec3(0.0, 1.0, 0.0),
                color: vec3(1.0, 0.0, 0.0),
            },
        ]
    }
}
