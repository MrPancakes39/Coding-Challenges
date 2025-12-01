// https://adventofcode.com/2025/day/1

use std::str::FromStr;

const MOD: u8 = 100;

const INPUT: &'static str = include_str!("./input.txt");

const EX: &'static str = r#"L68
L30
R48
L5
R60
L55
L1
L99
R14
L82
"#;

// ====================================================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rotation {
    Left(u16),
    Right(u16),
}

#[derive(Debug, PartialEq, Eq)]
struct ParseRotationError;

impl FromStr for Rotation {
    type Err = ParseRotationError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (dir, num) = s.split_at(1);
        let num: u16 = num.parse().map_err(|_| ParseRotationError)?;

        match dir {
            "R" => Ok(Rotation::Right(num)),
            "L" => Ok(Rotation::Left(num)),
            _ => Err(ParseRotationError),
        }
    }
}

// ====================================================================================================

fn part1(initial_count: u16, input: &str) -> u16 {
    let mut count: i32 = initial_count as i32;
    let mut num_zero = 0;

    for line in input.lines() {
        let rotation = line.parse::<Rotation>().unwrap();

        match rotation {
            Rotation::Left(n) => {
                count -= n as i32;
            }
            Rotation::Right(n) => {
                count += n as i32;
            }
        }

        // Keep it between 0 and MOD
        while count < 0 {
            count += MOD as i32;
        }
        while count >= MOD as i32 {
            count -= MOD as i32;
        }

        if count == 0 {
            num_zero += 1;
        }
    }

    num_zero
}

fn main() {
    println!("Part 1: {}", part1(50, INPUT));
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_parse() {
        let i1 = "L30".parse::<Rotation>().unwrap();
        let i2 = "R48".parse::<Rotation>().unwrap();

        assert_eq!(i1, Rotation::Left(30));
        assert_eq!(i2, Rotation::Right(48))
    }

    #[test]
    fn can_solve_part1() {
        let result = part1(50, EX);
        assert_eq!(result, 3);
    }
}
