use regex::Regex;

pub fn solve(input: &str) {
    star1(input);
    star2(input);
}

fn star1(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();

    let val: i32 = re
        .captures_iter(input)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            x.to_i32() * y.to_i32()
        })
        .sum();

    println!("{val}");
}

fn star2(input: &str) {
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let re2 = Regex::new(r"(?s)don't\(\).*?do\(\)").unwrap();
    let re3 = Regex::new(r"(?s)don't\(\).*$").unwrap();

    let cleaned = re2.replace_all(input, "");
    let cleaned = re3.replace(&cleaned, "");
    let val: i32 = re
        .captures_iter(&cleaned)
        .map(|caps| {
            let (_, [x, y]) = caps.extract();
            x.to_i32() * y.to_i32()
        })
        .sum();

    println!("{val}");
}

trait ForceI32 {
    fn to_i32(&self) -> i32;
}

impl ForceI32 for &str {
    fn to_i32(&self) -> i32 {
        self.parse::<i32>().unwrap()
    }
}
