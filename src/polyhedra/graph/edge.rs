use super::Vertex;

#[derive(Debug, Clone, Eq, Copy)]
pub struct Edge<V: Vertex> {
    pub a: V,
    pub b: V,
}

impl<V: Vertex> Edge<V> {
    pub fn other(&self, v: V) -> V {
        if self.a == v {
            self.b.clone()
        } else {
            self.a.clone()
        }
    }
}
impl<V: Vertex> From<&Edge<V>> for Edge<V> {
    fn from(value: &Edge<V>) -> Self {
        (value.a.clone(), value.b.clone()).into()
    }
}
impl<V: Vertex> From<(V, V)> for Edge<V> {
    fn from(value: (V, V)) -> Self {
        Self {
            a: value.0,
            b: value.1,
        }
    }
}

impl<V: Vertex> PartialEq for Edge<V> {
    fn eq(&self, other: &Self) -> bool {
        (self.a == other.a && self.b == other.b) || (self.a == other.b && self.b == other.a)
    }
}
