use ndarray::Array2;

#[derive(Debug, Clone)]
struct Grid2<T>(pub Array2<T>);

impl<T> Grid2<T> {
    fn new(grid: Array2<T>) -> Self {
        Self(grid)
    }
}

// Grid utilities
#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

impl Dir {
    pub fn rotate(&self, times: usize) -> Self {
        use Dir::*;
        const CARDINALS: [Dir; 8] = [
            N, NE, E, SE, S, SW, W, NW
        ];

        let (i, _) = CARDINALS.iter().enumerate().find(|(_, d)| *d == self).unwrap();
        CARDINALS[(i + times) % 8]
    }
}

#[derive(Debug, Clone)]
pub struct Grid<T>(pub Vec<Vec<T>>);
pub type Loc = (usize, usize);

impl<T> Grid<T> {
    pub fn neighbor(&self, dir: Dir, idx: Loc) -> Option<Loc> {
        use Dir::*;
        let next = match dir {
            N => (-1, 0),
            NE => (-1, 1),
            E => (0, 1),
            SE => (1, 1),
            S => (1, 0),
            SW => (1, -1),
            W => (0, -1),
            NW => (-1, -1),
        };

        let next = checked_add(idx, next);

        if let Some(next) = next {
            if self.in_bounds(next) {
                return Some(next);
            }
        }

        None
    }

    pub fn path(&self, loc: Loc, dir: Dir) -> DirPath<'_, T> {
        DirPath::new(self, loc, dir)
    }

    pub fn in_bounds(&self, idx: Loc) -> bool {
        idx.0 < self.0.len() && idx.1 < self.0[0].len()
    }

    pub fn rows(&self) -> usize {
        self.0.len()
    }

    pub fn cols(&self) -> usize {
        self.0[0].len()
    }

    pub fn at(&self, loc: Loc) -> Option<&T> {
        if self.in_bounds(loc) {
            let (r, c) = loc;
            Some(&self.0[r][c])
        } else {
            None
        }
    }
}

fn checked_add(x: Loc, y: (isize, isize)) -> Option<Loc> {
    let z0 = x.0.checked_add_signed(y.0);
    let z1 = x.1.checked_add_signed(y.1);

    match (z0, z1) {
        (Some(z0), Some(z1)) => Some((z0, z1)),
        _ => None,
    }
}

#[derive(Debug, Clone)]
pub struct DirPath<'a, T> {
    grid: &'a Grid<T>,
    loc: Loc,
    dir: Dir,
}

impl<'a, T> DirPath<'a, T> {
    fn new(grid: &'a Grid<T>, loc: Loc, dir: Dir) -> Self {
        DirPath { grid, loc, dir }
    }
}

impl<'a, T> Iterator for DirPath<'a, T> {
    type Item = &'a T;

    fn next(&mut self) -> Option<Self::Item> {
        let Some(next) = self.grid.neighbor(self.dir, self.loc) else {
            return None;
        };

        self.loc = next;
        Some(self.grid.at(next).unwrap())
    }
}

pub trait ForceI32 {
    fn to_i32(&self) -> i32;
}

impl ForceI32 for &str {
    fn to_i32(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
}
