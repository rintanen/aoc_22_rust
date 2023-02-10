use itertools::Itertools;

fn main() {
    let raw_input = include_str!("../../inputs/day05.in");

    let (boxes, moves) = raw_input.split_once("\n\n").unwrap();

    let num_stacks = boxes.lines()
        .rev()
        .take(1)
        .next()
        .unwrap()
        .chars()
        .filter(|c| c.is_numeric())
        .count();

    let mut stacks: Vec<Vec<char>>  = vec![vec![]; num_stacks];
    
    boxes.lines()
        .rev()
        .skip(1)
        .map(str::as_bytes)
        .for_each(|l| {
            for i in 0..stacks.len() {
                let val = l[1 + i * 4]; 
                if val.is_ascii_alphabetic() {
                    stacks[i].push(val as char);
                }
            }
        });
    
    let moves: Vec<(usize, usize, usize)> = moves.lines()
        .map(|l| l.split_ascii_whitespace()
            .filter_map(|s| s.parse::<usize>().ok())
            .collect_tuple()
            .unwrap()
        )
        .collect::<Vec<_>>();
    
    // PT1
    let mut stacks_clone = stacks.clone();

    for (this_many, from, to) in moves.clone() {
        for _ in 0..this_many {
            let move_this = stacks_clone[from - 1].pop().unwrap();
            stacks_clone[to - 1].push(move_this);
        }
    }

    println!("PT1: {}", 
        stacks_clone.iter()
            .map(|stack| stack.last().unwrap())
            .join(""));
    

    // PT2
    for (this_many, from, to) in moves {
        let mut temp = vec![];
        for _ in 0..this_many {
            let move_this = stacks[from - 1].pop().unwrap();
            temp.push(move_this);
        }
        while let Some(item) = temp.pop() {
            stacks[to - 1].push(item);
        }
    }

    println!("PT2: {}", 
        stacks.iter()
            .map(|stack| stack.last().unwrap())
            .join(""));

}


