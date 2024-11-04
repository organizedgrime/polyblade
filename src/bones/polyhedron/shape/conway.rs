
use crate::bones::{Shape, VertexId};

impl Shape {
    pub fn truncate(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
        let edges = self.distance.truncate(degree);
        self.recompute();
        edges
    }

    pub fn ambo(&mut self) -> Vec<[VertexId; 2]> {
        let edges = self.distance.ambo();
        self.recompute();
        edges
    }

    pub fn kis(&mut self, degree: Option<usize>) -> Vec<[VertexId; 2]> {
        let edges = self.distance.edges().collect();
        // let mut cycles = self.cycles.clone();
        if let Some(degree) = degree {
            self.cycles
                .iter()
                .collect::<Vec<_>>()
                .retain(|c| c.len() == degree);
        }
        for cycle in self.cycles.iter() {
            let v = self.distance.insert();
            // let mut vpos = Vec3::zero();

            for &u in cycle.iter() {
                self.distance.connect([v, u]);
                //vpos += self.positions[&u];
            }

            //self.positions.insert(v, vpos / cycle.len() as f32);
        }

        self.recompute();
        edges
    }
}
