//! Helper functions and const

use std::collections::HashSet;

// GRIDS

pub type Coord = (isize, isize);

pub const VECS4: [Coord; 4] = [(1, 0), (0, 1), (-1, 0), (0, -1)];
pub const VECS8: [Coord; 8] = [(1, 0), (0, 1), (-1, 0), (0, -1), (-1, -1), (1, 1), (-1, 1), (1, -1)];

/// Get HashSet of cells representing a boundary around a nxn grid
pub fn get_boundary(n: isize) -> HashSet<Coord> {
    let mut boundary = HashSet::new();
    for i in -1..=n {
        boundary.insert((-1, i));
        boundary.insert((n, i));
        boundary.insert((i, -1));
        boundary.insert((i, n));
    }
    boundary
}
