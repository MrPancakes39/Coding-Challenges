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

fn slice_to_num(slc: &[char]) -> usize {
    slc.iter().collect::<String>().parse::<usize>().unwrap()
}

fn vec_start_num_left(vec: &Vec<char>, i: usize) -> Option<(usize, &char)> {
    vec.iter()
        .enumerate()
        .take(i)
        .rev()
        .take_while(|(_, c)| c.is_numeric())
        .last()
}

fn vec_end_num_left(vec: &Vec<char>, i: usize) -> Option<(usize, &char)> {
    vec.iter()
        .enumerate()
        .skip(i)
        .take_while(|(_, c)| c.is_numeric())
        .last()
}

fn vec_start_num_right(vec: &Vec<char>, i: usize) -> Option<(usize, &char)> {
    vec.iter()
        .enumerate()
        .skip(i + 1)
        .rev()
        .take_while(|(_, c)| c.is_numeric())
        .last()
}

fn vec_end_num_right(vec: &Vec<char>, i: usize) -> Option<(usize, &char)> {
    vec.iter()
        .enumerate()
        .skip(i + 1)
        .take_while(|(_, c)| c.is_numeric())
        .last()
}

fn get_gear_part_sum(input: &str) -> usize {
    let mut sum = 0;
    let input_lines: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

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
        while let Some((i, &ch)) = iter.next() {
            if ch != '*' {
                continue;
            }

            let is_gear_part = {
                let mut counter = 0;
                counter += prev[i].is_numeric() as i32;
                counter += next[i].is_numeric() as i32;
                if i != 0 {
                    // not start of line
                    counter += prev[i - 1].is_numeric() as i32;
                    counter += current[i - 1].is_numeric() as i32;
                    counter += next[i - 1].is_numeric() as i32;
                }
                if i != current.len() - 1 {
                    // not end of line
                    counter += prev[i + 1].is_numeric() as i32;
                    counter += current[i + 1].is_numeric() as i32;
                    counter += next[i + 1].is_numeric() as i32;
                }
                counter > 1
            };

            if is_gear_part {
                // println!("row_i={row_i} i={i}, ch={ch}, is_gear_part={is_gear_part}.");
                let mut gear_ratio = 1;

                // check left
                if i != 0 {
                    // top left
                    if prev[i] == '.' && prev[i - 1].is_numeric() {
                        let start = vec_start_num_left(prev, i).map(|(i, _)| i).unwrap();
                        let end = i - 1;
                        let num = slice_to_num(&prev[start..=end]);
                        // println!("{:?}", num)
                        gear_ratio *= num;
                    }
                    // middle left
                    if current[i - 1].is_numeric() {
                        let start = vec_start_num_left(current, i).map(|(i, _)| i).unwrap();
                        let end = i - 1;
                        let num = slice_to_num(&current[start..=end]);
                        // println!("{:?}", num)
                        gear_ratio *= num;
                    }
                    // bottom left
                    if next[i] == '.' && next[i - 1].is_numeric() {
                        let start = vec_start_num_left(next, i).map(|(i, _)| i).unwrap();
                        let end = i - 1;
                        let num = slice_to_num(&next[start..=end]);
                        // println!("{:?}", num)
                        gear_ratio *= num;
                    }
                }
                // check right
                if i != current.len() - 1 {
                    // top right
                    if prev[i] == '.' && prev[i + 1].is_numeric() {
                        // println!("p = {:?}, i = {i:?}", vec_start_num_right(prev, i));
                        let start = i + 1;
                        let end = vec_end_num_right(prev, i).map(|(i, _)| i).unwrap();
                        // println!("start = {:?}, end = {:?}", start, end);
                        let num = slice_to_num(&prev[start..=end]);
                        // println!("{:?}", num)
                        gear_ratio *= num;
                    }
                    // middle right
                    if current[i + 1].is_numeric() {
                        let start = i + 1;
                        let end = vec_end_num_right(current, i).map(|(i, _)| i).unwrap();
                        let num = slice_to_num(&current[start..=end]);
                        // println!("{:?}", num)
                        gear_ratio *= num;
                    }
                    // bottom right
                    if next[i] == '.' && next[i + 1].is_numeric() {
                        // println!("q = {:?}, i = {i:?}", vec_start_num_right(prev, i));
                        let start = i + 1;
                        let end = vec_end_num_right(next, i).map(|(i, _)| i).unwrap();
                        // println!("start={start:?}, end={end:?}");
                        let num = slice_to_num(&next[start..=end]);
                        // println!("{:?}", num)
                        gear_ratio *= num;
                    }
                }
                // check center
                if prev[i].is_numeric() {
                    let start = match prev
                        .iter()
                        .enumerate()
                        .take(i)
                        .rev()
                        .take_while(|(_, c)| c.is_numeric())
                        .last()
                    {
                        Some((j, _)) => j, // if starts before *
                        None => i,         // if it starts above *
                    };
                    let end = prev
                        .iter()
                        .enumerate()
                        .skip(i)
                        .take_while(|(_, c)| c.is_numeric())
                        .last()
                        .map(|(i, _)| i)
                        .unwrap();
                    // dbg!(start);
                    // dbg!(end);
                    let num = slice_to_num(&prev[start..=end]);
                    // println!("{:?}", num)
                    gear_ratio *= num;
                }
                if next[i].is_numeric() {
                    let start = match next
                        .iter()
                        .enumerate()
                        .take(i)
                        .rev()
                        .take_while(|(_, c)| c.is_numeric())
                        .last()
                    {
                        Some((j, _)) => j, // if starts before *
                        None => i,         // if it starts above *
                    };
                    let end = next
                        .iter()
                        .enumerate()
                        .skip(i)
                        .take_while(|(_, c)| c.is_numeric())
                        .last()
                        .map(|(i, _)| i)
                        .unwrap();
                    // dbg!(start);
                    // dbg!(end);
                    let num = slice_to_num(&next[start..=end]);
                    // println!("{:?}", num)
                    gear_ratio *= num;
                }

                // dbg!(gear_ratio);
                sum += gear_ratio;
            }
            // println!("counter: {counter}");
        }
        // println!("-----------------------\n");
    });

    sum
}

fn part1() {
    let part1 = get_part_nums_sum(INPUT);
    println!("Part 1: {:?}", part1);
}

fn part2() {
    let part2 = get_gear_part_sum(INPUT);
    println!("Part 2: {:?}", part2);
}

fn main() {
    // part1();
    part2();
}

#[cfg(test)]
mod tests {
    use crate::{get_gear_part_sum, get_part_nums_sum};

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

    #[test]
    fn check_output_part2() {
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
        let output = get_gear_part_sum(input);
        assert_eq!(output, 467835)
    }
}
