// https://adventofcode.com/2025/day/1

use std::{fmt::Display, str::FromStr};

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

impl Display for Rotation {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Rotation::Left(n) => write!(f, "L{n}"),
            Rotation::Right(n) => write!(f, "R{n}"),
        }
    }
}

// ====================================================================================================

fn solution(initial_count: u16, input: &str) -> (u16, u16) {
    let mut count: i32 = initial_count as i32;
    let mut num_zero = 0;
    let mut num_point = 0;

    // println!("The dial starts by pointing at {count}.");

    for line in input.lines() {
        let current_count = count;
        let rotation = line.parse::<Rotation>().unwrap();

        let rot_count = match rotation {
            Rotation::Left(n) => {
                count -= n as i32;
                if count < 0 && current_count != 0 {
                    i32::abs(count) / MOD as i32 + 1
                } else {
                    0
                }
            }
            Rotation::Right(n) => {
                count += n as i32;
                if count > MOD as i32 {
                    i32::abs(count) / MOD as i32
                } else {
                    0
                }
            }
        };

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

        // print!("The dial is rotated {rotation} to point at {count}");
        // if rot_count > 0 {
        //     println!(" during this rotation, it points at zero {rot_count} time(s).");
        // } else {
        //     println!(".");
        // }

        num_point += rot_count as u16;
    }

    (num_zero, num_point)
}

fn main() {
    let result = solution(50, INPUT);
    println!("Part 1: {}", result.0);
    println!("Part 2: {}", result.1);
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
        let (solution, _) = solution(50, EX);
        assert_eq!(solution, 3);
    }

    #[test]
    fn point_to_zero() {
        let count = 50;
        let rot = -68;
        let new_count = count + rot;
        let rot_count = i32::abs(new_count) / MOD as i32 + 1;
        assert_eq!(rot_count, 1);

        let count = 95;
        let rot = 60;
        let new_count = count + rot;
        let rot_count = i32::abs(new_count) / MOD as i32;
        assert_eq!(rot_count, 1);

        let count = 14;
        let rot = -82;
        let new_count = count + rot;
        let rot_count = i32::abs(new_count) / MOD as i32 + 1;
        assert_eq!(rot_count, 1);

        let count = 50;
        let rot = 1000;
        let new_count = count + rot;
        let rot_count = i32::abs(new_count) / MOD as i32;
        assert_eq!(rot_count, 10);
    }

    #[test]
    fn can_solve_part2() {
        let solution = solution(50, EX);
        let solution = solution.0 + solution.1;
        assert_eq!(solution, 6);
    }
}
