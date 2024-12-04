use std::collections::HashSet;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

fn star1(input: &str) {
    let grid: Grid<char> = Grid(input.lines().map(|x| x.chars().collect()).collect());

    use Dir::*;
    let dirs = [N, NE, E, SE, S, SW, W, NW];

    let mut count = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            for dir in &dirs {
                let mut curr = (r, c);
                let mut word = String::new();
                for i in 0..4 {
                    match grid.at(curr) {
                        Some(c) => word.push(*c),
                        None => break,
                    }
                    match grid.neighbor(*dir, &curr) {
                        Some(c) => curr = c,
                        _ => break,
                    }
                }
                if word == "XMAS" {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

fn star2(input: &str) {
    let grid: Grid<char> = Grid(input.lines().map(|x| x.chars().collect()).collect());

    use Dir::*;

    let line1 = [NE, SW];
    let line2 = [NW, SE];
    let mut count = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            let curr = (r, c);
            if *grid.at(curr).unwrap() == 'A' {
                if in_line(&grid, &line1, &['M', 'S'], curr)
                && in_line(&grid, &line2, &['M', 'S'], curr) {
                    count += 1;
                }
            }
        }
    }

    println!("{count}");
}

fn in_line(grid: &Grid<char>, line: &[Dir], chars: &[char], curr: Loc) -> bool {
    let locs: Option<HashSet<_>> = line.iter().map(|d| {
        grid.neighbor(*d, &curr).and_then(|l| grid.at(l))
    })
    .collect();
    if let Some(locs) = locs {
        chars.iter().all(|c| locs.contains(c))
    }
    else {
        false
    }
}

#[derive(Copy, Clone)]
enum Dir {
    N,
    NE,
    E,
    SE,
    S,
    SW,
    W,
    NW,
}

struct Grid<T>(Vec<Vec<T>>);
type Loc = (usize, usize);

impl<T> Grid<T> {
    fn neighbor(&self, dir: Dir, idx: &Loc) -> Option<Loc> {
        use Dir::*;
        let next = match dir {
            N => (-1, 0),
            NE => (- 1, 1),
            E => (0, 1),
            SE=> (1, 1),
            S=> (1,0),
            SW=> (1, -1),
            W=> (0, -1),
            NW=> (-1, -1),
        };

        let next = check_add(*idx, next);

        if let Some(next) = next {
            if self.in_bounds(&next) {
                return Some(next);
            }
        }

        None
    }

    fn in_bounds(&self, idx: &Loc) -> bool {
        idx.0 < self.0.len() && idx.1 < self.0[0].len()
    }

    fn rows(&self) -> usize {
        self.0.len()
    }

    fn cols(&self) -> usize {
        self.0[0].len()
    }

    fn at(&self, loc: Loc) -> Option<&T> {
        if self.in_bounds(&loc) {
            let (r, c) = loc;
            Some(&self.0[r][c])
        } else {
            None
        }
    }


}

fn check_add(x: Loc, y: (i32, i32)) -> Option<Loc> {
    let (x1, x2) = x;
    let (y1, y2) = y;

    if y1 < 0 && x1 < y1.abs() as usize {
        return None;
    }
    if y2 < 0 && x2 < y2.abs() as usize {
        return None;
    }

    let z1 = if y1 < 0 {
        x1 - y1.abs() as usize
    } else {
        x1 + y1 as usize
    };
    let z2 = if y2 < 0 {
        x2 - y2.abs() as usize
    } else {
        x2 + y2 as usize
    };

    Some((z1, z2))
}
