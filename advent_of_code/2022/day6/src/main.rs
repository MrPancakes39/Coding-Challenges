use std::collections::HashMap;

const INPUT: &str = include_str!("input.txt");

fn unique_seq(sequence: Option<&[char]>) -> bool {
    match sequence {
        None => false,
        Some(sequence) => {
            let mut dict: HashMap<char, i32> = HashMap::new();
            for letter in sequence {
                let count = dict.entry(*letter).or_insert(0);
                *count += 1;
                if *count > 1 {
                    return false;
                }
            }
            true
        }
    }
}

fn get_info(datastream: &str, n: usize) -> Option<usize> {
    let len = datastream.len();
    if len < n {
        return None;
    }

    let data: Vec<char> = datastream.chars().collect();

    for i in 0..=len - n {
        if unique_seq(data.get(i..i + n)) {
            return Some(i + n);
        }
    }

    None
}

fn get_marker(datastream: &str) -> Option<usize> {
    get_info(datastream, 4)
}

fn get_message(datastream: &str) -> Option<usize> {
    get_info(datastream, 14)
}

fn main() {
    println!("Marker: {}", get_marker(INPUT).unwrap());
    println!("Message: {}", get_message(INPUT).unwrap());
}
