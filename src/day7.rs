use itertools::Itertools;
use crate::util::ForceI32;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

fn star1(input: &str) {
    let valid: i64 = input.lines().map(|l| {
        let (test, vals) = l.split(": ").collect_tuple().unwrap();
        let test = test.to_i64();
        let vals: Vec<_> = vals.split_whitespace().map(|i| i.to_i64()).collect();

        (test, vals)
    })
    .filter(|(test, vals)| {
        vals.iter().fold(vec![], |mut paths, &x| {
            if paths.is_empty() {
                paths.push(x);
                paths
            } else {
                // Sometimes we can pretend to be Haskell as a treat
                paths.into_iter().map(|y| [y + x, y * x]).flatten().collect_vec()
            }
        })
        .iter()
        .find(|&x| x == test)
        .is_some()
    })
    .map(|(test, _)| test)
    .sum();

    println!("{valid}");
}

fn concat(x: i64, y: i64) -> i64 {
    let f = format!("{}{}", x, y);
    f.as_str().to_i64()
}

fn star2(input: &str) {
    let valid: i64 = input.lines().map(|l| {
        let (test, vals) = l.split(": ").collect_tuple().unwrap();
        let test = test.to_i64();
        let vals: Vec<_> = vals.split_whitespace().map(|i| i.to_i64()).collect();

        (test, vals)
    })
    .filter(|(test, vals)| {
        vals.iter().fold(vec![], |mut paths, &x| {
            if paths.is_empty() {
                paths.push(x);
                paths
            } else {
                paths.into_iter().map(|y| [y + x, y * x, concat(y, x)]).flatten().collect_vec()
            }
        })
        .iter()
        .find(|&x| x == test)
        .is_some()
    })
    .map(|(test, _)| test)
    .sum();

    println!("{valid}");
}
