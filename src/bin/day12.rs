use std::fs;
use std::fmt;
use std::collections::{VecDeque, HashSet};
use std::hash::Hash;


fn is_positive_index(loc: &Loc, delta: (isize, isize)) -> bool {
    loc.row as isize - delta.0 >= 0 && loc.col as isize - delta.1 >= 0
}


#[derive(Eq, PartialEq, Clone, Hash)]
struct Loc {
    row: usize,
    col: usize
}


impl Loc {
    fn adjacent(&self) -> Vec<Loc> {
        let mut nbs = vec![];
        let deltas: [(isize, isize); 4] = [(1, 0), (0, -1), (-1, 0), (0, 1)];
        for delta in deltas {
            if is_positive_index(&self, delta) {
                let row = self.row as isize - delta.0;
                let col = self.col as isize - delta.1;
                nbs.push(Loc{row: row as usize, col: col as usize})
            }
        }
        nbs
    }
}


impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({} {})", self.row, self.col)
    }
}


#[derive(Clone)]
struct Grid<'a> {
    array: &'a [u8],
    shape: (usize, usize)
}


impl<'a> Grid<'a> {
    fn new(arr: &'a [u8]) -> Grid {
        let cols = arr.iter().position(|&b| b == 10).unwrap();
        let rows = arr.len() / cols;
        Grid { array: arr, shape: (rows, cols )}
    }

    fn loc(&self, location: &Loc) -> Option<&'a u8> {
        let index_1d = location.col + self.shape.1 * location.row + location.row;
        // add row's index to the 2d->1d grid mapping index 
        // because there is newline character between each row
        if index_1d < self.array.len() {
            let res = Some(&self.array[index_1d]);
            // exception for start and end location
            if res == Some(&b'S') {
                return Some(&b'a');
            } else if res == Some(&b'E') {
                return Some(&b'z');
            }
            res
        } else {
            None
        }
    }

    fn find(&self, target: u8) ->  Option<Loc> {
        let index_1d = self.array.iter().position(|c| c == &target); 
        if let Some(i) = index_1d {
            let row = i / self.shape.1;
            let col = i % self.shape.1 - row;
            Some(Loc{ row, col })
        } else {
            None
        }
    }
}


fn get_neighbours(grid: &Grid, node: &Loc, reverse: bool) -> Vec<Loc> {
    let node_height = grid.loc(&node).unwrap();
    let neighbours = node.adjacent().iter()
        .filter(|nb| {
            let Some(nb_height) = grid.loc(nb) else { return false };
            if reverse == false {
                nb_height.wrapping_sub(*node_height) <= 1
            } else {
                node_height.wrapping_sub(*nb_height) <= 1
            }
        })
        .cloned()
        .collect::<Vec<Loc>>();
    neighbours
}

fn breadth_first_search(grid: Grid, start: Loc, dest:u8, reverse: bool) -> Option<Vec<Loc>>{
    let mut explored: HashSet<Loc> = HashSet::new();
    let mut queue: VecDeque<Vec<Loc>> = VecDeque::new();
    queue.push_back(vec![start]);
    while let Some(path) = queue.pop_front() {
        let node = path.last().cloned().unwrap();
        if !explored.contains(&node) {
            let neighbours = get_neighbours(&grid, &node, reverse);
            for neighbour in neighbours {
                let mut new_path = path.clone();
                new_path.push(neighbour.clone());
                queue.push_back(new_path.clone());
                if *grid.loc(&neighbour).unwrap() == dest {
                    return Some(new_path);
                }
            }
            explored.insert(node);
        }
    }
    None
}

fn check_result(path: Option<Vec<Loc>>) {
    match path {
        Some(p) => {
            println!("Found path");
            println!("Steps: {}", p.len() - 1);
        },
        None => println!("No path found")
    }
}

fn main() {
    let raw_input: String = fs::read_to_string("inputs/day12_asd.in").unwrap();
    let grid = Grid::new(raw_input.as_bytes());

    println!("Part 1");
    let start_loc = grid.find(b'S').unwrap();
    let res = breadth_first_search(grid.clone(), start_loc, b'E', false);
    check_result(res);

    println!("Part 2");
    let start_loc = grid.find(b'E').unwrap();
    let res = breadth_first_search(grid.clone(), start_loc, b'a', true);
    check_result(res);
}
