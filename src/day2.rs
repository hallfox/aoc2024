use itertools::Itertools;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

/// The levels are either all increasing or all decreasing.
/// Any two adjacent levels differ by at least one and at most three.
///
/// 7 6 4 2 1: Safe because the levels are all decreasing by 1 or 2.
/// 1 2 7 8 9: Unsafe because 2 7 is an increase of 5.
/// 9 7 6 2 1: Unsafe because 6 2 is a decrease of 4.
/// 1 3 2 4 5: Unsafe because 1 3 is increasing but 3 2 is decreasing.
/// 8 6 4 4 1: Unsafe because 4 4 is neither an increase or a decrease.
/// 1 3 6 7 9: Safe because the levels are all increasing by 1, 2, or 3.
fn star1(input: &str) {
    let safe = input
        .lines()
        .map(|levs| {
            levs.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .tuple_windows()
                .map(|(x, y)| x - y)
                .collect()
        })
        .filter(|diffs: &Vec<_>| {
            (diffs.iter().all(|x| *x < 0) || diffs.iter().all(|x| *x > 0))
                && diffs.iter().all(|x| x.abs() >= 1 && x.abs() <= 3)
        })
        .count();

    println!("{safe}");
}

fn star2(input: &str) {
    let levels: Vec<Vec<_>> = input
        .lines()
        .map(|levs| {
            levs.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect()
        })
        .collect();

    let safe = levels
        .iter()
        .filter(|case: &&Vec<i32>| case.iter().combinations(case.len() - 1).any(is_safe))
        .count();

    println!("{safe}");
}

fn is_safe(levels: Vec<&i32>) -> bool {
    let diffs: Vec<_> = levels.iter().tuple_windows().map(|(x, y)| *x - *y).collect();
    (diffs.iter().all(|x| *x < 0) || diffs.iter().all(|x| *x > 0))
        && diffs.iter().all(|x| x.abs() >= 1 && x.abs() <= 3)
}
