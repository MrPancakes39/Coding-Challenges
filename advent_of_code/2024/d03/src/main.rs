use std::cell::LazyCell;

use regex::Regex;

const INPUT: &str = include_str!("./input.txt");

const EX: &'static str =
    r#"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))"#;

const EX2: &'static str =
    r#"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))"#;

const RE: LazyCell<Regex> = LazyCell::new(|| Regex::new(r"mul\((\d{1,3},\d{1,3})\)").unwrap());

#[derive(Debug, Eq, PartialEq, PartialOrd)]
enum OP {
    Mul(usize, u32),
    DO(usize),
    DONT(usize),
}

impl Ord for OP {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let a = match self {
            OP::Mul(n, _) => *n,
            OP::DO(n) => *n,
            OP::DONT(n) => *n,
        };
        let b = match other {
            OP::Mul(n, _) => *n,
            OP::DO(n) => *n,
            OP::DONT(n) => *n,
        };

        a.cmp(&b)
    }
}

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

fn part2(input: &str) -> u32 {
    let mult_re = RE;
    let mult_iter = mult_re.captures_iter(input).map(|c| {
        let n = c.get(0).unwrap().start();
        let (_, c) = c.extract::<1>();
        let c = c[0];

        let Some((left, right)) = c.split_once(',') else {
            return OP::Mul(n, 0);
        };
        let Ok(left) = left.parse::<u32>() else {
            return OP::Mul(n, 0);
        };
        let Ok(right) = right.parse::<u32>() else {
            return OP::Mul(n, 0);
        };

        OP::Mul(n, left * right)
    });

    let re_do = Regex::new(r"do\(\)").unwrap();
    let re_dont = Regex::new(r"don't\(\)").unwrap();

    let do_iter = re_do.find_iter(input).map(|m| OP::DO(m.start()));
    let dont_iter = re_dont.find_iter(input).map(|m| OP::DONT(m.start()));

    let mut ops: Vec<OP> = do_iter.chain(dont_iter).chain(mult_iter).collect();
    ops.sort_by(|a, b| a.cmp(b));

    let mut flag = true;
    ops.iter().fold(0, |acc, op| match op {
        OP::DO(_) => {
            flag = true;
            acc
        }
        OP::DONT(_) => {
            flag = false;
            acc
        }
        OP::Mul(_, m) => match flag {
            true => acc + m,
            false => acc,
        },
    })
}

fn main() {
    let answer = part2(INPUT);
    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(161, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(48, part2(EX2));
    }
}
