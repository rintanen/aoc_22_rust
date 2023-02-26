fn instruction_routine(
    n: usize, // how many cpu cycles instruction takes
    cycle: &mut i32, // pointer to current cycle number
    x: &mut i32, // pointer to register value
    register_values: &mut Vec<i32>,
    display_rows: &mut Vec<String>,
    display_row: &mut String
) 
{
    for _ in 0..n {
        // start of cycle -> draw pixel
        display_row.push(
            if [*x - 1, *x, *x + 1].iter().any(|&val| val == display_row.len() as i32) {
                '#'
            }
            else {
                '.'
            }
        );
        
        // store signal strenght during these cycles
        if [20, 60, 100, 140, 180, 220].iter().any(|r| r == cycle) {
            let signal_strenght = *cycle * *x;
            register_values.push(signal_strenght);
        }
        // new cycle begins here
        *cycle += 1;

        if (*cycle - 1) % 40 == 0 {
            // display row is complete store it and start new one
            display_rows.push(display_row.clone());
            display_row.clear();
        }
    }
}


fn main() {
    let instructions = include_str!("../../inputs/day10.in");
    let mut register_values = vec![];
    let mut cycle = 1;
    let mut x = 1;
    let mut display_rows = vec![];
    let mut display_row = String::new();

    for instruction in instructions.lines() {
        if let Some((_, incr)) = instruction.split_once(' ') {
            // addx
            instruction_routine(2, &mut cycle, &mut x, &mut register_values, &mut display_rows, &mut display_row);
            x += incr.parse::<i32>().unwrap();
        } else {
            // noop
            instruction_routine(1, &mut cycle, &mut x, &mut register_values, &mut display_rows, &mut display_row);
        }
    }
    println!("P1: {}", register_values.iter().sum::<i32>());

    display_rows.iter().for_each(|l| println!("{:?}", l));
}
