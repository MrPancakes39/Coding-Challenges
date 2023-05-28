const INPUT: &str = include_str!("input.txt");

#[allow(dead_code)]
fn part1(matrix: &Vec<Vec<i32>>) {
    let mut count = 2 * matrix.len() + 2 * matrix[0].len() - 4;
    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            let visible_h_left = matrix[i].iter().take(j).all(|&elt| matrix[i][j] > elt);
            let visible_h_right = matrix[i].iter().skip(j + 1).all(|&elt| matrix[i][j] > elt);
            let visible_v_top = matrix.iter().take(i).all(|elt| matrix[i][j] > elt[j]);
            let visible_v_bottom = matrix.iter().skip(i + 1).all(|elt| matrix[i][j] > elt[j]);

            if visible_h_left || visible_h_right || visible_v_top || visible_v_bottom {
                count += 1;
            }
        }
    }
    println!("Number of trees visibile: {}", count);
}

#[allow(dead_code)]
fn part2(matrix: &Vec<Vec<i32>>) {
    let mut best_dist = -1;
    let mut best = (0, 0);

    for i in 1..matrix.len() - 1 {
        for j in 1..matrix[i].len() - 1 {
            let mut dist_h_left = 0;
            for elt in matrix[i].iter().take(j).rev() {
                dist_h_left += 1;
                if elt >= &matrix[i][j] {
                    break;
                }
            }

            let mut dist_h_right = 0;
            for elt in matrix[i].iter().skip(j + 1) {
                dist_h_right += 1;
                if elt >= &matrix[i][j] {
                    break;
                }
            }

            let mut dist_v_up = 0;
            for elt in matrix.iter().take(i).rev() {
                dist_v_up += 1;
                if elt[j] >= matrix[i][j] {
                    break;
                }
            }

            let mut dist_v_down = 0;
            for elt in matrix.iter().skip(i + 1) {
                dist_v_down += 1;
                if elt[j] >= matrix[i][j] {
                    break;
                }
            }

            let dist = dist_v_up * dist_h_right * dist_v_down * dist_h_left;
            if dist > best_dist {
                best_dist = dist;
                best = (i, j);
            }
        }
    }

    println!(
        "Best spot ({},{}): {} with dist {}",
        best.0, best.1, matrix[best.0][best.1], best_dist
    );
}

fn main() {
    let matrix = INPUT
        .lines()
        .into_iter()
        .map(|line| {
            line.split("")
                .filter_map(|c| {
                    if c.is_empty() {
                        None
                    } else {
                        c.parse::<i32>().ok()
                    }
                })
                .collect::<Vec<i32>>()
        })
        .collect::<Vec<Vec<i32>>>();

    // part1(&matrix);
    part2(&matrix);
}
