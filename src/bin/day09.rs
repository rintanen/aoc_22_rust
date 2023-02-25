use std::collections::HashSet;

fn simulate_rope(instructions: &Vec<(&str, u8)>, n: usize) -> usize {
    let mut nodes = vec![(0, 0); n];
    let mut visited = HashSet::new();

    for (direction, amount) in instructions {
        let delta = {
            match *direction {
                "U" => (0, 1),
                "D" => (0, -1),
                "L" => (-1, 0),
                "R" => (1, 0),
                _ => (0, 0)
            }
        };

        for _ in 0..*amount {
            nodes[0].0 += delta.0;
            nodes[0].1 += delta.1;

            for i in 1..n {
                let dx: i32 = nodes[i-1].0 - nodes[i].0;
                let dy: i32 = nodes[i-1].1 - nodes[i].1;

                if dx.abs() > 1 || dy.abs() > 1 {
                    if dx.abs() >= 1 {
                        nodes[i].0 += (dx / dx.abs()) as i32 
                    }
                    if dy.abs() >= 1 {
                        nodes[i].1 += (dy / dy.abs()) as i32
                    }
                }

            }
            visited.insert(nodes.last().cloned());
        }
    }
    visited.len()
}



fn main() {
    let instructions = include_str!("../../inputs/day09.in")
        .lines()
        .map(|l| {
            let (direction, amount) = l.split_once(' ').unwrap();
            (direction, amount.parse::<u8>().unwrap())
        })
        .collect::<Vec<(&str, u8)>>();

    println!("PT1: {}", simulate_rope(&instructions, 2));
    println!("PT2: {}", simulate_rope(&instructions, 10));
}