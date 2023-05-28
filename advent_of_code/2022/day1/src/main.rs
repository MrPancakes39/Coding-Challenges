use regex::Regex;

fn main() {
    let input = include_str!("input.txt");
    let seperator = Regex::new(r"\n\n").unwrap();
    let mut calories_vec: Vec<i32> = seperator
        .split(input)
        .into_iter()
        .map(|calories_str| {
            calories_str
                .lines()
                .into_iter()
                .map(|calorie_str| calorie_str.parse::<i32>().unwrap())
                .sum()
        })
        .collect();
    // let max_calories: i32 = calories_vec.into_iter().fold(0, |acc, val| acc.max(val));
    // println!("Max Calories: {}", max_calories);
    calories_vec.sort();
    let max_calories: i32 = *calories_vec.iter().last().unwrap();
    println!("Max Calories: {}", max_calories);
    let top3_calories: i32 = calories_vec.iter().rev().take(3).sum();
    println!("The top three Elves are carrying: {}", top3_calories);
}
