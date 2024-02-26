// Representation of an undirected graph
// Uses adjacency lists
pub struct Poly {
    // Adjacency list
    pub edges: Vec<Vec<i32>>,

    // We can also directly represent the edges to make their computation trivial
    pub faces: Vec<Vec<i32>>,
}

impl Poly {}
/*
 *
 * Euler's formula:
 * V - E + F = 2
 *
 *
 * How do we create pretty Schlegel diagrams from our
 * known edge sets like octahedrons, dodecahedrons, etc?
 * just make the vertices repel each other and let the physics solve it somehow.
 * can perform a greedy algorithm for initial layout:
 * start at a vertex, (or actually, a face, if we want it centered), and draw all the adjacent
 * vertices next, at an increased radius from the origin. do this again and again until all
 * vertices have been placed, then allow the physics simulation to act on them and bring them to a
 * state of rest. this should be a cute and simple way to solve for these diagrams no matter the
 * polyhedra we're solving for.
 */
