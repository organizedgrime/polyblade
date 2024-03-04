use std::collections::HashSet;

use super::{Point, Polyhedron};

// Platonic Solids
impl Polyhedron {
    pub fn new(name: &str, points: Vec<Vec<usize>>, faces: Vec<Vec<usize>>) -> Polyhedron {
        Polyhedron {
            name: String::from(name),
            points: points.into_iter().map(Point::new).collect(),
            faces,
            enemies: HashSet::new(),
        }
    }

    pub fn tetrahedron() -> Polyhedron {
        Polyhedron::new(
            "T",
            vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]],
            vec![vec![0, 1, 2], vec![1, 0, 3], vec![2, 1, 3], vec![0, 2, 3]],
        )
    }

    pub fn cube() -> Polyhedron {
        Polyhedron::new(
            "C",
            vec![
                vec![1, 2, 7],
                vec![0, 3, 6],
                vec![0, 3, 5],
                vec![1, 2, 4],
                vec![3, 5, 6],
                vec![2, 4, 7],
                vec![1, 4, 7],
                vec![0, 5, 6],
            ],
            vec![
                vec![0, 1, 6, 7],
                vec![1, 3, 4, 6],
                vec![3, 2, 5, 4],
                vec![2, 0, 7, 5],
                vec![2, 3, 1, 0],
                vec![6, 7, 5, 4],
            ],
        )
    }
    pub fn octahedron() -> Polyhedron {
        Polyhedron::new(
            "O",
            vec![
                vec![1, 2, 3, 4],
                vec![0, 2, 4, 5],
                vec![0, 1, 3, 5],
                vec![0, 2, 4, 5],
                vec![0, 1, 3, 5],
                vec![1, 2, 3, 4],
            ],
            vec![
                vec![2, 0, 1],
                vec![1, 0, 4],
                vec![4, 0, 3],
                vec![3, 0, 2],
                vec![3, 5, 2],
                vec![3, 5, 4],
                vec![4, 5, 1],
                vec![1, 5, 2],
            ],
        )
    }
    pub fn dodecahedron() -> Polyhedron {
        Polyhedron::new(
            "D",
            vec![
                vec![1, 4, 7],
                vec![0, 2, 9],
                vec![1, 3, 11],
                vec![4, 2, 13],
                vec![0, 3, 5],
                vec![4, 6, 14],
                vec![5, 7, 16],
                vec![0, 6, 8],
                vec![7, 9, 17],
                vec![1, 8, 10],
                vec![9, 11, 18],
                vec![2, 10, 12],
                vec![11, 13, 19],
                vec![3, 14, 12],
                vec![5, 13, 15],
                vec![14, 16, 19],
                vec![15, 6, 17],
                vec![8, 16, 18],
                vec![10, 17, 19],
                vec![12, 18, 15],
            ],
            vec![
                vec![0, 1, 2, 3, 4],
                vec![0, 4, 5, 6, 7],
                vec![0, 7, 8, 9, 1],
                vec![1, 9, 10, 11, 2],
                vec![2, 11, 12, 13, 3],
                vec![3, 13, 14, 5, 4],
                vec![16, 6, 7, 8, 17],
                vec![17, 8, 9, 10, 18],
                vec![18, 10, 11, 12, 19],
                vec![19, 12, 13, 14, 15],
                vec![15, 14, 5, 6, 16],
                vec![15, 16, 17, 18, 19],
            ],
        )
    }
    pub fn icosahedron() -> Polyhedron {
        Polyhedron::new(
            "I",
            vec![
                vec![1, 2, 5, 3, 4],
                vec![2, 0, 7, 6, 5],
                vec![0, 1, 7, 8, 3],
                vec![4, 0, 2, 8, 9],
                vec![0, 5, 3, 9, 10],
                vec![0, 1, 4, 6, 10],
                vec![1, 7, 5, 11, 10],
                vec![6, 1, 2, 8, 11],
                vec![2, 3, 7, 11, 9],
                vec![11, 8, 3, 4, 10],
                vec![11, 9, 4, 5, 6],
                vec![9, 8, 7, 6, 10],
            ],
            vec![
                vec![0, 1, 2],
                vec![0, 3, 4],
                vec![0, 4, 5],
                vec![0, 5, 1],
                vec![0, 3, 2],
                vec![1, 5, 6],
                vec![1, 6, 7],
                vec![1, 7, 2],
                vec![2, 7, 8],
                vec![2, 8, 3],
                vec![4, 10, 5],
                vec![5, 10, 6],
                vec![6, 11, 7],
                vec![7, 11, 8],
                vec![8, 9, 3],
                vec![3, 9, 4],
                vec![4, 9, 10],
                vec![6, 10, 11],
                vec![11, 8, 9],
                vec![11, 9, 10],
            ],
        )
    }
}
