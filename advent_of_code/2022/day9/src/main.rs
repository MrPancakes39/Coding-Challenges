use std::collections::HashSet;

const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone, Copy, Eq, Hash, PartialEq)]
struct Point {
    x: i32,
    y: i32,
}

impl Point {
    fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    fn dist(p1: &Point, p2: &Point) -> f32 {
        (((p2.x - p1.x).pow(2) + (p2.y - p1.y).pow(2)) as f32).sqrt()
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    fn new(letter: &str) -> Self {
        match letter {
            "U" => Direction::Up,
            "D" => Direction::Down,
            "L" => Direction::Left,
            "R" => Direction::Right,
            _ => panic!("Can't create Direction from invalid letter '{letter}'."),
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Instruction {
    dir: Direction,
    step: i32,
}

impl Instruction {
    fn normalize(&self) -> Self {
        Instruction {
            dir: self.dir,
            step: 1,
        }
    }
}

fn update_tail(head: &Point, tail: &mut Point) {
    // update tail
    if Point::dist(&head, &tail) >= 2.0 {
        // if head is right or left of tail directly.
        if head.y == tail.y {
            if head.x > tail.x {
                tail.x += 1;
            } else if head.x < tail.x {
                tail.x -= 1;
            }
        }
        // if head is up or down of tail directly.
        if head.x == tail.x {
            if head.y == tail.y + 2 {
                tail.y += 1;
            } else if head.y == tail.y - 2 {
                tail.y -= 1;
            }
        }
        // if head is diagonally to the right of tail.
        if head.x == tail.x + 2 {
            if head.y > tail.y {
                tail.y += 1;
            } else if head.y < tail.y {
                tail.y -= 1;
            }
            tail.x += 1;
        }
        // if head is diagonally to the left of tail.
        if head.x == tail.x - 2 {
            if head.y > tail.y {
                tail.y += 1;
            } else if head.y < tail.y {
                tail.y -= 1;
            }
            tail.x -= 1;
        }
        // if head is diagonally to the up of tail.
        if head.y == tail.y + 2 {
            if head.x > tail.x {
                tail.x += 1;
            } else if head.x < tail.x {
                tail.x -= 1;
            }
            tail.y += 1;
        }
        // if head is diagonally to the down of tail.
        if head.y == tail.y - 2 {
            if head.x > tail.x {
                tail.x += 1;
            } else if head.x < tail.x {
                tail.x -= 1;
            }
            tail.y -= 1;
        }
    }
}

fn move_step(instruction: Instruction, knots: &mut [Point], ihead: usize, itail: usize) {
    let mut head = knots[ihead];
    let mut tail = knots[itail];
    // move head
    match instruction.dir {
        Direction::Up => head.y += 1,
        Direction::Down => head.y -= 1,
        Direction::Left => head.x -= 1,
        Direction::Right => head.x += 1,
    }

    update_tail(&head, &mut tail);

    knots[ihead] = head;
    knots[itail] = tail;
}

fn move_rope(instruction: Instruction, knots: &mut [Point], places: &mut HashSet<Point>) {
    let n = knots.len();
    for _ in 1..=instruction.step {
        move_step(instruction.normalize(), knots, 0, 1);
        for i in 1..n - 1 {
            update_tail(&knots[i].clone(), &mut knots[i + 1]);
        }
        places.insert(knots[n - 1]);
    }
}

fn main() {
    let instructions: Vec<Instruction> = INPUT
        .lines()
        .map(|line| {
            let (letter, step) = line.split_once(' ').unwrap();
            Instruction {
                dir: Direction::new(letter),
                step: step.parse().unwrap(),
            }
        })
        .collect();

    let mut knots = [Point::new(0, 0); 10];

    let mut places: HashSet<Point> = HashSet::new();

    for inst in instructions {
        move_rope(inst, &mut knots, &mut places);
    }

    // print list
    // for (i, k) in knots.iter().enumerate() {
    //     if i == 0 {
    //         print!("H: ");
    //     } else {
    //         print!("{i}: ");
    //     }
    //     println!("{:?}", k);
    // }

    println!("Places visited: {}", places.len());
}
