
fn simulate_rope(instructions: &Vec<(&str, u8)>, n: usize) {
    let mut nodes = vec![(0, 0); n];
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
        println!("{:?}", delta);

        for _ in 0..*amount {
            nodes[0].0 += delta.0;
            nodes[1].1 += delta.1;

            for i in 1..nodes.len() {

            }
        }
    }
}



fn main() {
    let instructions = include_str!("../../inputs/day09.in")
        .lines()
        .map(|l| {
            let (direction, amount) = l.split_once(' ').unwrap();
            (direction, amount.parse::<u8>().unwrap())
        })
        .collect::<Vec<(&str, u8)>>();

    simulate_rope(&instructions, 2);
}