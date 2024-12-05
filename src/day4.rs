use std::collections::HashSet;
use crate::util::{Grid, Dir, Loc};

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
                let curr = (r, c);
                let word: String = grid.path(curr, *dir).take(4).collect();
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
        grid.neighbor(*d, curr).and_then(|l| grid.at(l))
    })
    .collect();
    if let Some(locs) = locs {
        chars.iter().all(|c| locs.contains(c))
    }
    else {
        false
    }
}

