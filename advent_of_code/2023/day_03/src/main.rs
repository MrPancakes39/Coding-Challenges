const INPUT: &str = include_str!("./input.txt");

fn is_symbol(c: char) -> bool {
    c.is_ascii_punctuation() && c != '.'
}

fn get_part_nums_sum(input: &str) -> usize {
    let mut sum: usize = 0;
    let mut input_lines: Vec<Vec<char>> =
        input.lines().map(|line| line.chars().collect()).collect();

    // pad the input text
    let empty_line = std::iter::repeat('.')
        .take(input_lines[0].len())
        .collect::<Vec<char>>();
    input_lines.push(empty_line.clone());
    input_lines.insert(0, empty_line);

    assert!(input_lines.len() >= 3);

    input_lines.windows(3).for_each(|window| {
        assert_eq!(window.len(), 3);
        let prev = &window[0];
        let current = &window[1];
        let next = &window[2];

        // println!("prev:\t\t{:?}", prev);
        // println!("current:\t{:?}", current);
        // println!("next:\t\t{:?}", next);
        // println!("-----------------------\n");

        let mut iter = current.iter().enumerate();
        while let Some((i, ch)) = iter.next() {
            if !ch.is_numeric() {
                continue;
            }
            let start_index = i;
            let mut end_index: Option<usize> = None;
            while let Some((j, next_ch)) = iter.next() {
                if next_ch.is_numeric() {
                    continue;
                }
                end_index = Some(j - 1);
                break;
            }
            let end_index = match end_index {
                Some(num) => num,
                None => current.len() - 1,
            };
            let num = current[start_index..=end_index]
                .iter()
                .collect::<String>()
                .parse::<usize>()
                .unwrap();
            // println!("({start_index}, {end_index:?}): {:?}", possible_num);

            // check line before
            let has_symbol_up = prev[start_index..=end_index]
                .iter()
                .map(|&c| is_symbol(c))
                .any(|x| x);
            if has_symbol_up {
                // println!("{num} is part num.");
                sum += num;
                continue;
            }
            // check line after
            let has_symbol_down = next[start_index..=end_index]
                .iter()
                .map(|&c| is_symbol(c))
                .any(|x| x);
            if has_symbol_down {
                // println!("{num} is part num.");
                sum += num;
                continue;
            }
            // check sides
            if start_index != 0 {
                // not start of line
                let has_symbol_left = is_symbol(prev[start_index - 1])
                    || is_symbol(current[start_index - 1])
                    || is_symbol(next[start_index - 1]);
                if has_symbol_left {
                    // println!("{num} is part num.");
                    sum += num;
                    continue;
                }
            }
            if end_index != current.len() - 1 {
                // not end of line
                let has_symbol_right = is_symbol(prev[end_index + 1])
                    || is_symbol(current[end_index + 1])
                    || is_symbol(next[end_index + 1]);
                if has_symbol_right {
                    // println!("{num} is part num.");
                    sum += num;
                    continue;
                }
            }
            // println!("{num} is NOT.");
        }

        // println!("-----------------------\n");
    });

    sum
}

fn part1() {
    let part1 = get_part_nums_sum(INPUT);
    println!("Part 1: {:?}", part1);
}

fn main() {
    part1();
}

#[cfg(test)]
mod tests {
    use crate::get_part_nums_sum;

    #[test]
    fn check_output() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        let output = get_part_nums_sum(input);
        assert_eq!(output, 4361)
    }
}
