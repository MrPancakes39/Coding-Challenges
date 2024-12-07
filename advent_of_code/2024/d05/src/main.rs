use std::collections::HashMap;

const INPUT: &'static str = include_str!("./input.txt");

const EX: &'static str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

fn parse_input(input: &str) -> (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>) {
    let rules: Vec<(u32, u32)> = input
        .lines()
        .filter_map(|line| {
            line.split_once('|').and_then(|(a, b)| {
                let Some(a) = a.parse::<u32>().ok() else {
                    return None;
                };
                let Some(b) = b.parse::<u32>().ok() else {
                    return None;
                };
                Some((a, b))
            })
        })
        .collect();

    let updates: Vec<Vec<u32>> = input
        .lines()
        .filter_map(|line| {
            (!line.contains('|'))
                .then_some(
                    line.split(',')
                        .map(|s| s.parse::<u32>().ok())
                        .collect::<Option<Vec<u32>>>(),
                )
                .flatten()
        })
        .collect();

    let mut rules_map: HashMap<u32, Vec<u32>> = HashMap::new();
    for (a, b) in rules {
        rules_map.entry(a).or_default().push(b);
    }

    (rules_map, updates)
}

fn rules_apply(rules: &HashMap<u32, Vec<u32>>, update: &Vec<u32>) -> bool {
    rules.into_iter().all(|(a, bs)| {
        bs.iter().all(|b| {
            let mut it = update.iter();

            match it.find(|n| n == &a) {
                None => return true,
                Some(_) => {}
            };

            match it.find(|n| n == &b) {
                Some(_) => return true,
                None => !update.contains(b),
            }
        })
    })
}

fn part1(input: &str) -> u32 {
    let (rules, updates) = parse_input(input);

    updates
        .iter()
        .filter_map(|update| rules_apply(&rules, update).then_some(update[update.len() / 2]))
        .sum()
}

fn main() {
    let answer = part1(INPUT);
    println!("Answer: {answer}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        assert_eq!(143, part1(EX));
    }
}
