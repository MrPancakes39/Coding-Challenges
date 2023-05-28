const INPUT: &str = include_str!("input.txt");

type Stack<T> = Vec<T>;

fn make_boxes(stacks: &str) -> Vec<Stack<char>> {
    // reverse order of lines then turn each line to array of chars
    let text: Vec<Vec<&str>> = stacks
        .lines()
        .rev()
        .map(|line| line.split("").collect::<Vec<&str>>())
        .collect();
    // get the positions of each stack
    let indicies: Vec<usize> = text[0]
        .iter()
        .enumerate()
        .filter_map(|(index, ch)| match ch.parse::<i32>() {
            Ok(_) => Some(index),
            Err(_) => None,
        })
        .collect();
    // init the stacks
    let n = indicies.len();
    let mut boxes: Vec<Stack<char>> = Vec::with_capacity(n);
    for _ in 0..n {
        boxes.push(Stack::new())
    }
    // loop through the lines and push the correct letter
    for line in text.iter().skip(1) {
        let mut s = 0;
        for (i, ch) in line.iter().enumerate() {
            if indicies.contains(&i) {
                let letter = ch.chars().next().unwrap();
                if letter != ' ' {
                    boxes[s].push(letter);
                }
                s = s + 1;
            }
        }
    }
    boxes
}

fn follow_instructions(boxes: &mut Vec<Stack<char>>, instructions: &str) {
    let instruct_iter = instructions.lines().map(|line| {
        line.split(" ")
            .enumerate()
            .filter_map(|(i, item)| match i {
                1 | 3 | 5 => Some(item.parse::<i32>().unwrap()),
                _ => None,
            })
            .collect::<Vec<i32>>()
    });
    for i in instruct_iter {
        let (count, src, dest) = (i[0], (i[1] - 1) as usize, (i[2] - 1) as usize);
        // for _ in 0..count {
        //     let item = boxes[src].pop().unwrap();
        //     boxes[dest].push(item);
        // }
        let start = boxes[src].len() - count as usize;
        let mut items_to_move: Stack<char> = boxes[src].drain(start..).collect();
        boxes[dest].append(&mut items_to_move);
    }
}

fn main() {
    let mut divide = INPUT.split("\n\n");
    let stacks: &str = divide.next().unwrap();
    let instructions: &str = divide.next().unwrap();

    let mut boxes: Vec<Stack<char>> = make_boxes(stacks);
    follow_instructions(&mut boxes, instructions);

    let mut message = String::new();
    for box_item in boxes {
        message.push(*box_item.last().unwrap());
    }
    println!("Message: {}", message);
}
