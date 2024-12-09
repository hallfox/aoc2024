use itertools::Itertools;
use std::iter;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

fn star1(input: &str) {
    let disk = input
        .chars()
        .filter(|&x| x.is_digit(10))
        .map(|i| i.to_digit(10).unwrap() as usize)
        .collect_vec();

    let mut free = false;
    let mut id = 0;
    let mut layout: Vec<i32> = vec![];
    for i in &disk {
        if free {
            layout.extend(iter::repeat_n(-1, *i));
        } else {
            layout.extend(iter::repeat_n(id, *i));
            id += 1;
        }
        free = !free;
    }

    let mut x = layout.len() - 1;
    let mut space = layout
        .iter()
        .enumerate()
        .find(|p| *p.1 == -1)
        .map(|p| p.0)
        .unwrap_or(layout.len());
    while space < x {
        if layout[x] != -1 {
            layout.swap(x, space);
            space = layout
                .iter()
                .enumerate()
                .find(|p| *p.1 == -1)
                .map(|p| p.0)
                .unwrap_or(layout.len());
        }
        x -= 1;
    }

    let sum: usize = layout
        .iter()
        .filter_map(|&i| if i >= 0 { Some(i as usize) } else { None })
        .enumerate()
        .map(|(i, j)| i * j)
        .sum();
    println!("{sum}")
}

fn star2(input: &str) {
    let disk = input
        .chars()
        .filter(|&x| x.is_digit(10))
        .map(|i| i.to_digit(10).unwrap() as usize)
        .collect_vec();

    let mut free = false;
    let mut id = 0;
    let mut layout: Vec<(i32, usize)> = vec![];
    for i in &disk {
        if free {
            layout.push((-1, *i));
        } else {
            layout.push((id, *i));
            id += 1;
        }
        free = !free;
    }

    let mut x = layout.len() - 1;
    let mut space = layout
        .iter()
        .enumerate()
        .find(|(_, p)| p.0 == -1)
        .map(|p| p.0)
        .unwrap_or(layout.len());
    while space < x {
        if layout[x].0 != -1 {
            let mut next = space;
            loop {
                if next >= x {
                    break;
                } else if layout[x].1 <= layout[next].1 {
                    let block = layout[next];
                    layout[next] = layout[x];
                    if block.1 > layout[x].1 {
                        layout.insert(next + 1, (-1, block.1 - layout[x].1));
                        x += 1;
                    }
                    layout[x] = (-1, layout[x].1);
                    break;
                }
                next = layout
                    .iter()
                    .enumerate()
                    .skip(next + 1)
                    .find(|(_, p)| p.0 == -1)
                    .map(|p| p.0)
                    .unwrap_or(layout.len());
            }
            space = layout
                .iter()
                .enumerate()
                .find(|(_, p)| p.0 == -1)
                .map(|p| p.0)
                .unwrap_or(layout.len());
        }
        x -= 1;
    }

    let sum: usize = layout
        .iter()
        .flat_map(|(i, x)| iter::repeat_n(*i, *x))
        .enumerate()
        .filter(|(_, x)| *x >= 0)
        .map(|(i, j)| i * j as usize)
        .sum();
    println!("{sum}")
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {
        let case = "2333133121414131402";
        star1(case);
        star2(case);
    }
}
