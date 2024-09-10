use polyblade_derive::Meow;

pub trait SimpleGraph {
    fn insert(&mut self) -> usize;
}

#[derive(Meow)]
pub struct Complex {
    #[internal]
    pub graph: Simple,
    pub dogs: f32,
}

pub struct Simple {}
impl SimpleGraph for Simple {
    fn insert(&mut self) -> usize {
        353
    }
}

fn main() {
    let mut c = Complex {
        graph: Simple {},
        dogs: 2.0,
    };

    println!("ci: {:?}", c.insert());

    assert_eq!(c.insert(), 353);
}
