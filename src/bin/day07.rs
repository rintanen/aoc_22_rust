use std::collections::{HashMap, HashSet};
use std::path::PathBuf;

fn dir_size() -> u32 {
    0
}


fn main()  {
    let task_input = include_str!("../../inputs/day07.in");
    let mut cwd = PathBuf::new();
    let mut file_tree: HashMap<PathBuf, u32> = HashMap::new();

    for chunk in task_input.split("$").skip(1) {
        let command = chunk.lines().next().unwrap().trim();
        // println!("l = ({})", command);
        match command {
            "cd .." => {
                cwd.pop();
            },
            "ls" => {
                // rekursiivisesti laske dir size ja laita value othon
                let size = dir_size();
                file_tree.insert(cwd.clone(), size);
            },
            // this matches any pattern, so catches the "cd <dir_name>" case
            cd => {
                cwd.push(cd.split_once(" ").unwrap().1);
            }
        }
    }

    println!("{:?}", cwd);
    
}