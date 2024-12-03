use std::io;

mod day1;
mod day2;
mod day3;

pub fn solve(day: i32) {
    let days = [day1::solve, day2::solve, day3::solve];

    let input = read_input();
    let i = (day - 1) as usize;
    days[i](&input);
}

fn read_input() -> String {
    io::read_to_string(io::stdin()).expect("Problem reading input")
}
