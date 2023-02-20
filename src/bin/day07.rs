use std::collections::HashMap;
use std::path::PathBuf;

fn calc_dir_size(file_tree: &HashMap<PathBuf, Vec<(u32, &str)>>, dir_sizes: &mut HashMap<PathBuf, u32>, dir_name: &PathBuf) {
    let size = file_tree[dir_name].iter()
        .map(|&(file_size, file_name)| {
            match file_size {
                // size 0 means directory, calculate size of that dir recursively
                0 => {
                    let dir_name = dir_name.join(file_name);
                    calc_dir_size(file_tree, dir_sizes, &dir_name);
                    dir_sizes[&dir_name]
                },
                // matches other cases, return file_size
                s => s
            }
        })
        .sum();
    dir_sizes.insert(dir_name.clone(), size);
}


fn main()  {
    let task_input = include_str!("../../inputs/day07.in");
    let mut cwd = PathBuf::new();
    let mut file_tree: HashMap<PathBuf, Vec<(u32, &str)>> = HashMap::new();

    for chunk in task_input.split('$').skip(1) {
        let command = chunk.lines().next().unwrap().trim();
        match command {
            "cd .." => {
                cwd.pop();
            },
            "ls" => { 
                let dir_contents = chunk.lines().skip(1)
                    .map(|l| {
                        let (size, file_name) = l.split_once(' ').unwrap();
                        (size.parse::<u32>().unwrap_or(0), file_name)
                    })
                    .collect::<Vec<(u32, &str)>>();
                file_tree.insert(cwd.clone(), dir_contents);
            },
            // this matches any pattern, so catches the "cd <dir_name>" case
            cd => {
                cwd.push(cd.split_once(" ").unwrap().1);
            }
        }
    }

    let mut dir_sizes: HashMap<PathBuf, u32> = HashMap::new();
    
    for key in file_tree.keys() {
        calc_dir_size(&file_tree, &mut dir_sizes, key);
    }

    let pt1 = dir_sizes.values().filter(|&&size| size <= 100000).sum::<u32>();
    println!("PT1: {}", pt1);
    let space_left = 70000000 - dir_sizes[&PathBuf::from("/")];
    let pt2 = dir_sizes.values().filter(|&&size| (space_left  + size) >= 30000000).min().unwrap();
    println!("PT2: {}", pt2);
}