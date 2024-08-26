use super::Vertex;
use crate::polyhedra::PolyGraph;
use ultraviolet::Vec3;

#[derive(Debug)]
pub struct Descriptor {
    /// Size of the buffer containing only position data
    //pub position_buffer_size: u64,
    /// Size of the buffer containing remaining vertex data
    //pub vertex_buffer_size: u64,
    /// Number of vertices when we represent the polyhedron as triangles
    pub vertex_triangle_count: u64,
}

impl From<&PolyGraph> for Descriptor {
    fn from(value: &PolyGraph) -> Self {
        let mut vertex_triangle_count = 0;
        for face in value.cycles.iter() {
            match face.len() {
                3 => {
                    vertex_triangle_count += 3;
                }
                4 => {
                    vertex_triangle_count += 6;
                }
                _ => {
                    vertex_triangle_count += 3 * face.len() as u64;
                }
            }
        }

        Self {
            //position_buffer_size: std::mem::size_of::<Vec3>() as u64 * vertex_triangle_count,
            //vertex_buffer_size: std::mem::size_of::<Vertex>() as u64 * vertex_triangle_count,
            vertex_triangle_count,
        }
    }
}
