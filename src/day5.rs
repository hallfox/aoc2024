use crate::util::ForceI32;
use itertools::Itertools;
use std::collections::{HashMap, HashSet};
use std::cmp::Ordering;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

fn star1(input: &str) {
    let (ordering, prints) = input.split("\n\n").collect_tuple().unwrap();

    let rules: HashMap<_, _> = ordering
        .lines()
        .map(|rule| {
            let (x, y) = rule.split("|").collect_tuple().unwrap();
            (x.to_i32(), y.to_i32())
        })
        .into_group_map();

    let total: i32 = prints
        .lines()
        .map(|line| line.split(",").map(|x| x.to_i32()).collect::<Vec<_>>())
        .filter(|case| is_ordered(&rules, case))
        .map(|case| case[case.len() / 2])
        .sum();

    println!("{total}");
}

fn is_ordered(rules: &HashMap<i32, Vec<i32>>, case: &Vec<i32>) -> bool {
    let mut seen = HashSet::new();
    for x in case {
        if let Some(orders) = rules.get(x) {
            let orders = orders.iter().collect();
            let union = seen.intersection(&orders).collect::<Vec<_>>();
            if !union.is_empty() {
                return false;
            }
        }
        seen.insert(x);
    }
    true
}

fn star2(input: &str) {
    let (ordering, prints) = input.split("\n\n").collect_tuple().unwrap();

    let rules: HashMap<_, _> = ordering
        .lines()
        .map(|rule| {
            let (x, y) = rule.split("|").collect_tuple().unwrap();
            (x.to_i32(), y.to_i32())
        })
        .into_group_map();

    let total: i32 = prints
        .lines()
        .map(|line| line.split(",").map(|x| x.to_i32()).collect::<Vec<_>>())
        .filter(|case| !is_ordered(&rules, case))
        .map(|mut case| {
            case.as_mut_slice().sort_by(|x, y| {
                if let Some(orders) = rules.get(x) {
                    if orders.iter().contains(y) {
                        return Ordering::Less;
                    }
                }
                if let Some(orders) = rules.get(y) {
                    if orders.iter().contains(x) {
                        return Ordering::Greater;
                    }
                }
                Ordering::Equal
            });
            case
        })
        .map(|case| case[case.len() / 2])
        .sum();

    println!("{total}");
}
