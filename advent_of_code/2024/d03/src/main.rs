use std::cell::LazyCell;

use regex::Regex;

const INPUT: &str = include_str!("./input.txt");

const EX: &'static str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

const RE: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap());

fn part1(input: &str) -> u32 {
    RE.captures_iter(input)
        .map(|c| {
            let (_, c) = c.extract::<1>();
            let c = c[0];

            let Some((left, right)) = c.split_once(',') else {
                return 0;
            };
            let Ok(left) = left.parse::<u32>() else {
                return 0;
            };
            let Ok(right) = right.parse::<u32>() else {
                return 0;
            };

            left * right
        })
        .sum()
}

fn main() {
    let answer = part1(INPUT);
    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(EX));
    }
}
