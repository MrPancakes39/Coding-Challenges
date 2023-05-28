fn letter_to_val(letter: char) -> i32 {
    if letter >= 'a' && letter <= 'z' {
        letter as i32 - 'a' as i32 + 1
    } else if letter >= 'A' && letter <= 'Z' {
        letter as i32 - 'A' as i32 + 27
    } else {
        panic!("letter '{letter}' is not an alpha char.");
    }
}

fn main() {
    let input = include_str!("input.txt");
    // let letters: Vec<_> = input
    //     .lines()
    //     .map(|line| {
    //         let length = line.len();
    //         for i in (&line[0..length / 2]).chars() {
    //             for j in (&line[length / 2..]).chars() {
    //                 if i == j {
    //                     return Some(i);
    //                 }
    //             }
    //         }
    //         None
    //     })
    //     .map(|x| x.unwrap())
    //     .collect();
    // let priorities: i32 = letters.iter().map(|letter| letter_to_val(*letter)).sum();
    // println!("Sum of priorities: {}", priorities);

    let mut current_array = -1;
    let mut group3: Vec<Vec<&str>> = Vec::new();
    for (i, line) in input.lines().enumerate() {
        if i as i32 / 3 == current_array {
            group3[current_array as usize].push(line);
        } else {
            current_array += 1;
            group3.push(vec![line]);
        }
    }
    let letters: Vec<char> = group3
        .into_iter()
        .map(|elves| {
            for i in (&elves[0]).chars() {
                for j in (&elves[1]).chars() {
                    for k in (&elves[2]).chars() {
                        if i == j && j == k {
                            return Some(i);
                        }
                    }
                }
            }
            None
        })
        .map(|val| val.unwrap())
        .collect();
    // println!("{:?}", letters);

    let sum: i32 = letters.iter().map(|letter| letter_to_val(*letter)).sum();
    println!("Sum of priorities: {}", sum);
}
