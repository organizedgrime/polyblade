use crate::bones::Polyhedron;

impl Polyhedron {
    pub fn truncate(&mut self) {
        for v in self.shape.distance.vertices() {
            self.render.extend(
                self.shape.distance.connections(v).len() - 1,
                self.render.positions[v],
            );
        }
        self.shape.truncate(None);
        self.shape.recompute();
    }
}
