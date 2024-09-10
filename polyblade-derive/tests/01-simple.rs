use polyblade_derive::Simple;

pub trait SimpleGraph {
    fn insert(&mut self) -> usize;
}

#[derive(Simple)]
pub struct Complex {
    #[internal]
    pub graph: Nya,
    pub dogs: f32,
}

pub struct Nya {}
impl SimpleGraph for Nya {
    fn insert(&mut self) -> usize {
        353
    }
}

fn main() {
    let mut c = Complex {
        graph: Nya {},
        dogs: 2.0,
    };

    println!("ci: {:?}", c.insert());

    assert_eq!(c.insert(), 353);
}
