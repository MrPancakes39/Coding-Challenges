
const INPUT: &'static str = include_str!("./input.txt");

fn line_to_digit(line: &str) -> i32 {
    let mut iter = line
    .chars()
    .filter(|f| f.is_numeric());

    let first_digit = iter.next().unwrap_or_default();
    if let Some(last_digit) = iter.next_back() {
        format!("{first_digit}{last_digit}").parse().unwrap_or_default()
    } else {
        format!("{first_digit}{first_digit}").parse().unwrap_or_default()
    }
}

fn main() {
    let answer = INPUT.lines().map(line_to_digit).sum::<i32>();
    println!("Part 1: {:?}.\n", answer);
}

#[cfg(test)]
mod tests {
    use crate::line_to_digit;

    #[test]
    fn line_digits_correct() {
        let input = "1abc2\npqr3stu8vwx\na1b2c3d4e5f\ntreb7uchet";
        let correct = [12, 38, 15, 77];
        let all_pass = input
        .lines()
        .map(line_to_digit)
        .zip(correct.iter())
        .all(|(answer, correct)| answer == *correct);
        assert!(all_pass);
    }
}