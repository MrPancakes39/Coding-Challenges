const INPUT: &str = include_str!("./input.txt");

use std::str::FromStr;

#[derive(Debug)]
enum Balls {
    Red(u32),
    Green(u32),
    Blue(u32),
    Unknown,
}

impl FromStr for Balls {
    type Err = Balls;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (num, color) = s.split_once(' ').ok_or(Balls::Unknown)?;
        let num = num.parse::<u32>().map_err(|_| Balls::Unknown)?;
        let ball = match color {
            "blue" => Balls::Blue(num),
            "red" => Balls::Red(num),
            "green" => Balls::Green(num),
            _ => Balls::Unknown,
        };
        Ok(ball)
    }
}

#[derive(Debug, Default)]
struct BallsCounter {
    red: u32,
    green: u32,
    blue: u32,
}

const BALL_COUNT: BallsCounter = BallsCounter {
    red: 12,
    green: 13,
    blue: 14,
};

fn is_game_possible(game: &str) -> (u32, bool) {
    let (game_id_str, game_info) = game.split_once(": ").unwrap();
    let game_id = game_id_str
        .chars()
        .filter(|c| c.is_numeric())
        .collect::<String>()
        .parse::<u32>()
        .unwrap();
    let has_over = game_info
        .split("; ")
        .map(|pick| {
            pick.split(", ")
                .map(|ball_str| ball_str.parse::<Balls>().unwrap())
                .map(|ball| match ball {
                    Balls::Red(count) => count > BALL_COUNT.red,
                    Balls::Green(count) => count > BALL_COUNT.green,
                    Balls::Blue(count) => count > BALL_COUNT.blue,
                    Balls::Unknown => panic!("SHouldn't get here"),
                })
                .any(|over| over)
        })
        .any(|over| over);
    (game_id, !has_over)
}

fn part1() {
    let part1 = INPUT
        .lines()
        .filter_map(|game| {
            let (game_id, is_possible) = is_game_possible(game);
            match is_possible {
                true => Some(game_id),
                _ => None,
            }
        })
        .sum::<u32>();
    println!("Part 1: {:?}", part1);
}

fn game_min_balls(game: &str) -> BallsCounter {
    let (_, game_info) = game.split_once(": ").unwrap();
    game_info
        .split("; ")
        .map(|pick| {
            let mut ball_count = BallsCounter::default();
            pick.split(", ")
                .map(|ball_str| ball_str.parse::<Balls>().unwrap())
                .for_each(|ball| match ball {
                    Balls::Red(count) => ball_count.red = count,
                    Balls::Green(count) => ball_count.green = count,
                    Balls::Blue(count) => ball_count.blue = count,
                    Balls::Unknown => panic!("Shouldn't get here"),
                });
            ball_count
        })
        .reduce(|acc, e| BallsCounter {
            red: acc.red.max(e.red),
            green: acc.green.max(e.green),
            blue: acc.blue.max(e.blue),
        })
        .unwrap()
}

fn part2() {
    let part2 = INPUT
        .lines()
        .map(game_min_balls)
        .map(|counter| counter.red * counter.green * counter.blue)
        .sum::<u32>();
    println!("Part 2: {:?}", part2);
}

fn main() {
    part2();
}
