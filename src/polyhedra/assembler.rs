use std::collections::HashSet;

use super::*;

// Keep track of many states while an operation is being performed
#[derive(Default)]
pub struct Assembler {
    pub flags: Vec<Vec<f64>>,
    pub vertices: Vec<Vec<f64>>,
    pub vertex_names: HashSet<String>,
    pub face_names: HashSet<String>,
}

impl Assembler {
    pub fn flag(&mut self, face: i32, v1: f64, v2: f64) {
        let face = face as usize;
        // Make sure there is a flag array for this face
        while self.flags.len() - 1 < face {
            self.flags.push(vec![]);
        }
        //self.flags[face][v1] = v2;
    }

    pub fn commit(&self) -> Polyhedron {
        Polyhedron {
            name: String::new(),
            faces: vec![],
            vertices: vec![],
        }
    }
}
