use std::{str::FromStr, string::ParseError};

#[derive(Debug, PartialEq, Clone)]
enum Hand {
    Rock,
    Paper,
    Scissors,
}

impl FromStr for Hand {
    type Err = ParseError;
    fn from_str(letter: &str) -> Result<Self, Self::Err> {
        match letter {
            "A" | "X" => Ok(Hand::Rock),
            "B" | "Y" => Ok(Hand::Paper),
            "C" | "Z" => Ok(Hand::Scissors),
            _ => panic!("Couldn't convert to Hand because of invalid letter '{letter}'."),
        }
    }
}

fn score(opp: &Hand, me: &Hand) -> i32 {
    let selected = match me {
        Hand::Rock => 1,
        Hand::Paper => 2,
        Hand::Scissors => 3,
    };
    let outcome = if me == opp {
        // tie
        3
    } else if me == &Hand::Rock && opp == &Hand::Scissors
        || me == &Hand::Paper && opp == &Hand::Rock
        || me == &Hand::Scissors && opp == &Hand::Paper
    {
        // win
        6
    } else {
        // lose
        0
    };
    selected + outcome
}

fn to_lose(hand: Hand) -> Hand {
    match hand {
        Hand::Rock => Hand::Scissors,
        Hand::Paper => Hand::Rock,
        Hand::Scissors => Hand::Paper,
    }
}

fn to_win(hand: Hand) -> Hand {
    match hand {
        Hand::Rock => Hand::Paper,
        Hand::Paper => Hand::Scissors,
        Hand::Scissors => Hand::Rock,
    }
}

fn strategy(opp: &str, me: &str) -> Vec<Hand> {
    let opp_hand = opp.parse::<Hand>().unwrap();
    match me {
        // draw
        "Y" => vec![opp_hand.clone(), opp_hand],
        // lose
        "X" => vec![opp_hand.clone(), to_lose(opp_hand)],
        // win
        "Z" => vec![opp_hand.clone(), to_win(opp_hand)],
        _ => panic!("Shouldn't reach here."),
    }
}

fn main() {
    let input = include_str!("input.txt");
    let rounds: Vec<_> = input
        .lines()
        .map(|line| {
            // let letters: Vec<Hand> = line
            //     .split(" ")
            //     .map(|letter| letter.parse::<Hand>().unwrap())
            //     .collect();
            // assert_eq!(letters.len(), 2);
            // letters
            let letters: Vec<&str> = line.split(" ").collect();
            assert_eq!(letters.len(), 2);
            strategy(letters[0], letters[1])
        })
        .collect();
    // println!("{:?}", rounds);
    let total_score: i32 = rounds
        .into_iter()
        .map(|round| score(&round[0], &round[1]))
        .sum();
    println!("Total score: {:?}", total_score);
}
