use std::fs;

fn main()  {
    let task_input = fs::read_to_string("inputs/day07.in").expect("fars");
    task_input.split("$");
}