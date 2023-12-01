
// const INPUT: &'static str = include_str!("./input.txt");
const INPUT2: &'static str = include_str!("./input.txt");

fn line_to_digit(line: &str) -> i32 {
    let line = replace_num_in_line(line);
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

const LOOKUP_ARRAY: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];

fn get_str_number(num_str: &str) -> i32 {
    LOOKUP_ARRAY
    .iter()
    .position(|&s| s == num_str)
    .map(|i| (i as i32) + 1)
    .unwrap()
}

fn replace_num_in_line(line: &str) -> String {
    let mut new_line = line.to_string();
    for num_str in LOOKUP_ARRAY {
        let mut c = num_str.chars();
        let first_letter = c.next().unwrap();
        let last_letter = c.next_back().unwrap();
        new_line = new_line.replace(
            num_str,
            &format!("{}{}{}", first_letter, get_str_number(num_str), last_letter)
        );
    }
    new_line
}

fn main() {
    // let answer = INPUT.lines().map(line_to_digit).sum::<i32>();
    // println!("Part 1: {:?}.\n", answer);
    let answer = INPUT2.lines().map(line_to_digit).sum::<i32>();
    println!("Part 2: {:?}.\n", answer);
}

#[cfg(test)]
mod tests {
    use crate::line_to_digit;

    #[test]
    fn line_digits_correct() {
        let input = "1abc2
        pqr3stu8vwx
        a1b2c3d4e5f
        treb7uchet";
        let correct = [12, 38, 15, 77];
        let all_pass = input
        .lines()
        .map(line_to_digit)
        .zip(correct.iter())
        .all(|(answer, correct)| answer == *correct);
        assert!(all_pass);
    }

    #[test]
    fn line_digits_correct2() {
        let input = "two1nine
        eightwothree
        abcone2threexyz
        xtwone3four
        4nineeightseven2
        zoneight234
        7pqrstsixteen";
        let correct = [29, 83, 13, 24, 42, 14, 76];
        let all_pass = input
        .lines()
        .map(line_to_digit)
        .zip(correct.iter())
        .all(|(answer, correct)| answer == *correct);
        assert!(all_pass);
    }
}