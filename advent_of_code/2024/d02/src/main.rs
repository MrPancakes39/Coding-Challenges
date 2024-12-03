const INPUT: &'static str = include_str!("./input.txt");

const EX: &'static str = r#"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9"#;

fn is_safe(v: &[i32]) -> bool {
    let diffs: Vec<i32> = v
        .iter()
        .zip(v.iter().skip(1))
        .map(|(&a, &b)| b - a)
        .collect();

    let is_monotonic = diffs
        .iter()
        .map(|&n| n.signum())
        .all(|n| n == diffs[0].signum());

    if !is_monotonic {
        return false;
    }

    let diff_safe = diffs.iter().map(|n| n.abs()).all(|n| n >= 1 && n <= 3);

    diff_safe
}

fn part1(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let line: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        match is_safe(&line) {
            true => acc + 1,
            false => acc,
        }
    })
}

fn part2(input: &str) -> i32 {
    input.lines().fold(0, |acc, line| {
        let line: Vec<i32> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if is_safe(&line) {
            return acc + 1;
        }

        let is_safe = line.iter().enumerate().any(|(i, _)| {
            let tmp: Vec<i32> = line
                .iter()
                .enumerate()
                .filter_map(|(j, &v)| match j == i {
                    true => None,
                    false => Some(v),
                })
                .collect();

            is_safe(&tmp)
        });

        match is_safe {
            true => acc + 1,
            false => acc,
        }
    })
}

fn main() {
    let answer = part2(&INPUT);
    println!("Answer: {answer}");
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(2, part1(EX));
    }

    #[test]
    fn test_part2() {
        assert_eq!(4, part2(EX));
    }
}
