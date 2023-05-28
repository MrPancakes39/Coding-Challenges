use std::{str::FromStr, string::ParseError};

#[derive(Debug)]
struct IDRange {
    start: i32,
    end: i32,
}

impl FromStr for IDRange {
    type Err = ParseError;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let nums: Vec<&str> = s.split("-").collect();
        if nums.len() != 2 {
            panic!("Wrong format.")
        } else {
            let s = nums[0].parse().unwrap();
            let e = nums[1].parse().unwrap();
            if s > e {
                panic!("Start is bigger than the end.")
            } else {
                Ok(IDRange { start: s, end: e })
            }
        }
    }
}

impl IDRange {
    fn contains(&self, other: &IDRange) -> bool {
        self.start <= other.start && self.end >= other.end
    }

    fn overlap(&self, other: &IDRange) -> bool {
        (self.end >= other.start && self.start <= other.start)
            || (self.start <= other.end && self.end >= other.end)
    }
}

fn main() {
    let input = include_str!("input.txt");
    let overlaps: Vec<bool> = input
        .lines()
        .map(|section| {
            let pairs: Vec<_> = section
                .split(",")
                .map(|range: &str| range.parse::<IDRange>().unwrap())
                .collect();
            // println!("{:?}", pairs);
            // pairs[0].contains(&pairs[1]) || pairs[1].contains(&pairs[0])
            pairs[0].overlap(&pairs[1]) || pairs[1].overlap(&pairs[0])
        })
        .collect();
    // println!("{:?}", overlaps);
    let num: i32 = overlaps.iter().map(|n| if *n { 1 } else { 0 }).sum();
    println!("Number of overlaps: {}", num);
}
