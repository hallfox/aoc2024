use itertools::Itertools;

use crate::util::{Dir, Grid, Loc};
use std::collections::HashSet;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

/// If there is something directly in front of you, turn right 90 degrees.
/// Otherwise, take a step forward.
fn star1(input: &str) {
    let grid: Grid<char> = Grid(input.lines().map(|l| l.chars().collect()).collect());
    let mut path = get_guard_path(&grid);
    path.sort();
    let spaces = path.iter().dedup().count();

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

fn get_guard_path(grid: &Grid<char>) -> Vec<Loc> {
    let start = find_start(&grid).unwrap();
    let mut pos = start;
    let mut dir = Dir::N;
    let mut path = Vec::new();
    'stop: loop {
        path.push(pos);

        loop {
            let next = match grid.neighbor(dir, pos) {
                Some(next) => next,
                None => break 'stop,
            };

            if grid.at(next) == Some(&'#') {
                dir = dir.rotate(2);
            } else {
                pos = next;
                break;
            }
        }
    }
    path
}

// This one takes a few seconds, I wouldn't accept it under a time limit
fn star2(input: &str) {
    let mut grid = Grid::from_str(input);
    let mut path = get_guard_path(&grid);
    path.sort();
    let spots = path
        .iter()
        .dedup()
        .filter(|(r, c)| {
            if grid.at((*r, *c)) == Some(&'.') {
                grid.0[*r][*c] = '#';
                let loops = is_looping(&grid);
                grid.0[*r][*c] = '.';
                loops
            } else {
                false
            }
        })
        .count();

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
