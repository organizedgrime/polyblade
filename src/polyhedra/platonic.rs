use super::PolyGraph;

/*
    // TODO: once conway ops are efficient enough- use this.
    T = Y3
    O = aT (ambo tetrahedron)
    C = jT (join tetrahedron)
    I = sT (snub tetrahedron)
    D = gT (gyro tetrahedron)
*/

// Platonic Solids
impl PolyGraph {
    pub fn tetrahedron() -> PolyGraph {
        PolyGraph::new_platonic(
            "T",
            vec![vec![1, 2, 3], vec![0, 2, 3], vec![0, 1, 3], vec![0, 1, 2]],
        )
    }

    pub fn cube() -> PolyGraph {
        PolyGraph::new_platonic(
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
        )
    }
    pub fn octahedron() -> PolyGraph {
        PolyGraph::new_platonic(
            "O",
            vec![
                vec![1, 2, 3, 4],
                vec![0, 2, 4, 5],
                vec![0, 1, 3, 5],
                vec![0, 2, 4, 5],
                vec![0, 1, 3, 5],
                vec![1, 2, 3, 4],
            ],
        )
    }
    pub fn dodecahedron() -> PolyGraph {
        PolyGraph::new_platonic(
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
        )
    }
    pub fn icosahedron() -> PolyGraph {
        PolyGraph::new_platonic(
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
        )
    }
}
