use crate::util::ForceI32;
use itertools::Itertools;

pub fn solve(input: &str) {
    {
        let before = std::time::Instant::now();
        star1(input);
        println!("Elapsed: {:.2?}", before.elapsed());
    }

    {
        let before = std::time::Instant::now();
        star2(input);
        println!("Elapsed: {:.2?}", before.elapsed());
    }
}

fn star1(input: &str) {
    let valid: i64 = input
        .lines()
        .map(|l| {
            let (test, vals) = l.split(": ").collect_tuple().unwrap();
            let test = test.to_i64();
            let vals: Vec<_> = vals.split_whitespace().map(|i| i.to_i64()).collect();

            (test, vals)
        })
        .filter(|(test, vals)| {
            vals.iter()
                .skip(1)
                .fold(vec![vals[0]], |paths, &x| {
                    // Sometimes we can pretend to be Haskell as a treat
                    paths
                        .into_iter()
                        .map(|y| [y + x, y * x])
                        .flatten()
                        .collect_vec()
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
    let valid: i64 = input
        .lines()
        .map(|l| {
            let (test, vals) = l.split(": ").collect_tuple().unwrap();
            let test = test.to_i64();
            let vals: Vec<_> = vals.split_whitespace().map(|i| i.to_i64()).collect();

            (test, vals)
        })
        .filter(|(test, vals)| {
            vals.iter()
                .skip(1)
                .fold(vec![vals[0]], |paths, &x| {
                    paths
                        .into_iter()
                        .map(|y| [y + x, y * x, concat(y, x)])
                        .flatten()
                        .collect_vec()
                })
                .iter()
                .find(|&x| x == test)
                .is_some()
        })
        .map(|(test, _)| test)
        .sum();

    println!("{valid}");
}
