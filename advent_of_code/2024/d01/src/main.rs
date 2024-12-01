// https://adventofcode.com/2024/day/1

const INPUT: &'static str = include_str!("./input.txt");

const EX: &'static str = r#"3   4
4   3
2   5
1   3
3   9
3   3"#;

#[cfg(feature = "part1")]
fn part1(txt: &str) -> u32 {
    let (mut left, mut right): (Vec<u32>, Vec<u32>) = txt
        .lines()
        .filter_map(|line| {
            let tmp: Vec<u32> = line
                .split(" ")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            match tmp.len() {
                2 => Some((tmp[0], tmp[1])),
                _ => None,
            }
        })
        .unzip();

    left.sort();
    right.sort();

    left.iter()
        .zip(right.iter())
        .map(|(&a, &b)| a.abs_diff(b))
        .sum::<u32>()
}

fn part2(txt: &str) -> u32 {
    let (left, right): (Vec<u32>, Vec<u32>) = txt
        .lines()
        .filter_map(|line| {
            let tmp: Vec<u32> = line
                .split(" ")
                .filter_map(|s| s.parse::<u32>().ok())
                .collect();
            match tmp.len() {
                2 => Some((tmp[0], tmp[1])),
                _ => None,
            }
        })
        .unzip();

    left.iter()
        .map(|&n| {
            n * right.iter().fold(0u32, |acc, &m| match n == m {
                true => acc + 1,
                false => acc,
            })
        })
        .sum()
}

fn main() {
    // let answer = part1(INPUT);

    let answer = part2(INPUT);

    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let answer = part1(EX);
        assert_eq!(answer, 11);
    }

    #[test]
    fn test_part2() {
        let answer = part2(EX);
        assert_eq!(answer, 31);
    }
}
