use itertools::Itertools;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

fn star1(input: &str) {
    let (mut xs, mut ys): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    xs.sort();
    ys.sort();

    let sum: i32 = xs.iter().zip(ys).map(|(x, y)| (x - y).abs()).sum();
    println!("{}", sum);
}

fn star2(input: &str) {
    let (xs, ys): (Vec<_>, Vec<_>) = input
        .lines()
        .map(|l| {
            l.split_whitespace()
                .map(|i| i.parse::<i32>().unwrap())
                .collect_tuple()
                .unwrap()
        })
        .unzip();

    let ys_counts = ys.iter().counts();
    let mut score = 0;
    for x in &xs {
        if ys_counts.contains_key(x) {
            score += x * ys_counts[x] as i32;
        }
    }

    println!("{}", score);
}
