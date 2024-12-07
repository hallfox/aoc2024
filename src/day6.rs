use crate::util::{Dir, Grid, Loc};
use std::collections::HashSet;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

/// If there is something directly in front of you, turn right 90 degrees.
/// Otherwise, take a step forward.
fn star1(input: &str) {
    let mut grid: Grid<char> = Grid(input.lines().map(|l| l.chars().collect()).collect());

    let start = find_start(&grid).unwrap();
    let mut pos = start;
    let mut dir = Dir::N;
    loop {
        let (r, c) = pos;
        grid.0[r][c] = 'X';

        let mut next = match grid.neighbor(dir, pos) {
            Some(next) => next,
            None => break,
        };
        while grid.at(next) == Some(&'#') {
            dir = dir.rotate(2);
            next = match grid.neighbor(dir, pos) {
                Some(next) => next,
                None => break,
            };
        }

        pos = next;
    }

    let spaces = grid.0.iter().flatten().filter(|&x| *x == 'X').count();
    println!("{spaces}");
}

fn find_start(grid: &Grid<char>) -> Option<Loc> {
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if grid.at((r, c)) == Some(&'^') {
                return Some((r, c));
            }
        }
    }
    None
}

// This one takes a few seconds, I wouldn't accept it under a time limit
fn star2(input: &str) {
    let mut grid: Grid<char> = Grid(input.lines().map(|l| l.chars().collect()).collect());

    let mut spots = 0;
    for r in 0..grid.rows() {
        for c in 0..grid.cols() {
            if grid.at((r, c)) == Some(&'.') {
                grid.0[r][c] = '#';
                if is_looping(&grid) {
                    spots += 1;
                }
                grid.0[r][c] = '.';
            }
        }
    }

    println!("{spots}");
}

fn is_looping(grid: &Grid<char>) -> bool {
    let start = find_start(&grid).unwrap();
    let mut pos = start;
    let mut dir = Dir::N;
    let mut visited = HashSet::new();
    loop {
        if !visited.insert((pos, dir)) {
            return true;
        }

        let mut next = match grid.neighbor(dir, pos) {
            Some(next) => next,
            None => return false,
        };
        while grid.at(next) == Some(&'#') {
            dir = dir.rotate(2);
            next = match grid.neighbor(dir, pos) {
                Some(next) => next,
                None => return false,
            };
        }

        pos = next;
    }
}
