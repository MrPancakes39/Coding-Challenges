const INPUT: &'static str = include_str!("./input.txt");

const EX: &'static str = r#"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX
"#;

fn part1(input: &str) -> usize {
    let mut count: usize = 0;
    let matrix: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();

    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.iter().enumerate() {
            if cell != &'X' {
                continue;
            }

            // Cases: SAMX and XMAS
            let level = (j.checked_sub(1).and_then(|k| row.get(k)) == Some(&'M')
                && j.checked_sub(2).and_then(|k| row.get(k)) == Some(&'A')
                && j.checked_sub(3).and_then(|k| row.get(k)) == Some(&'S'))
                as usize
                + (j.checked_add(1).and_then(|k| row.get(k)) == Some(&'M')
                    && j.checked_add(2).and_then(|k| row.get(k)) == Some(&'A')
                    && j.checked_add(3).and_then(|k| row.get(k)) == Some(&'S'))
                    as usize;

            let upper = {
                let s_char = i
                    .checked_sub(3)
                    .and_then(|k| matrix.get(k))
                    .map(|row| {
                        (
                            (j.checked_sub(3).and_then(|k| row.get(k)) == Some(&'S')) as usize,
                            (row.get(j) == Some(&'S')) as usize,
                            (j.checked_add(3).and_then(|k| row.get(k)) == Some(&'S')) as usize,
                        )
                    })
                    .unwrap_or_default();

                let a_char = i
                    .checked_sub(2)
                    .and_then(|k| matrix.get(k))
                    .map(|row| {
                        (
                            (j.checked_sub(2).and_then(|k| row.get(k)) == Some(&'A')) as usize,
                            (row.get(j) == Some(&'A')) as usize,
                            (j.checked_add(2).and_then(|k| row.get(k)) == Some(&'A')) as usize,
                        )
                    })
                    .unwrap_or_default();

                let m_char = i
                    .checked_sub(1)
                    .and_then(|k| matrix.get(k))
                    .map(|row| {
                        (
                            (j.checked_sub(1).and_then(|k| row.get(k)) == Some(&'M')) as usize,
                            (row.get(j) == Some(&'M')) as usize,
                            (j.checked_add(1).and_then(|k| row.get(k)) == Some(&'M')) as usize,
                        )
                    })
                    .unwrap_or_default();

                ((s_char.0 + a_char.0 + m_char.0) == 3) as usize
                    + ((s_char.1 + a_char.1 + m_char.1) == 3) as usize
                    + ((s_char.2 + a_char.2 + m_char.2) == 3) as usize
            };

            let lower = {
                let s_char = i
                    .checked_add(3)
                    .and_then(|k| matrix.get(k))
                    .map(|row| {
                        (
                            (j.checked_sub(3).and_then(|k| row.get(k)) == Some(&'S')) as usize,
                            (row.get(j) == Some(&'S')) as usize,
                            (j.checked_add(3).and_then(|k| row.get(k)) == Some(&'S')) as usize,
                        )
                    })
                    .unwrap_or_default();

                let a_char = i
                    .checked_add(2)
                    .and_then(|k| matrix.get(k))
                    .map(|row| {
                        (
                            (j.checked_sub(2).and_then(|k| row.get(k)) == Some(&'A')) as usize,
                            (row.get(j) == Some(&'A')) as usize,
                            (j.checked_add(2).and_then(|k| row.get(k)) == Some(&'A')) as usize,
                        )
                    })
                    .unwrap_or_default();

                let m_char = i
                    .checked_add(1)
                    .and_then(|k| matrix.get(k))
                    .map(|row| {
                        (
                            (j.checked_sub(1).and_then(|k| row.get(k)) == Some(&'M')) as usize,
                            (row.get(j) == Some(&'M')) as usize,
                            (j.checked_add(1).and_then(|k| row.get(k)) == Some(&'M')) as usize,
                        )
                    })
                    .unwrap_or_default();

                ((s_char.0 + a_char.0 + m_char.0) == 3) as usize
                    + ((s_char.1 + a_char.1 + m_char.1) == 3) as usize
                    + ((s_char.2 + a_char.2 + m_char.2) == 3) as usize
            };

            count += level + upper + lower;
        }
    }

    count
}

fn main() {
    let answer = part1(INPUT);
    println!("Answer: {}", answer);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(18, part1(EX));
    }
}
