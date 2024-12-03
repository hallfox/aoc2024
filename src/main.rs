use aoc_2024::solve;
use std::env::args;

fn main() {
    let day = args()
        .skip(1)
        .next()
        .expect("./aoc2024 [day]")
        .parse::<i32>()
        .expect("Invalid day");
    solve(day);
}
