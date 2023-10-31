use std::fs;
use std::fmt;
use std::collections::{VecDeque, HashSet};
use std::hash::{Hash, Hasher};


fn is_positive_index(loc: &Loc, delta: (isize, isize)) -> bool {
    loc.row as isize - delta.0 >= 0 && loc.col as isize - delta.1 >= 0
}


// fn walkable(neighbour: Vec<Loc>, grid: &Grid) -> Vec<Loc> {
//     let mut walkable_nbs = vec![];
//     for neighbour in neighbours {
//         let nb_height = grid.loc(&neighbour);
//         let node_height = grid.loc(&node);
//         if grid.loc(&neighbour) <= grid.loc(&node) {
//             walkable_nbs.push(neighbour.clone());
//         }
//     }
//     walkable_nbs
// }

#[derive(Eq, PartialEq, Clone)]
struct Loc {
    row: usize,
    col: usize
}

impl Loc {
    fn neighbours(&self) -> Vec<Loc> {
        let mut nbs = vec![];
        let deltas: [(isize, isize); 4] = [(0, 1), (0, -1), (1, 0), (-1, 0)];
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

impl Hash for Loc {
    fn hash<H: Hasher>(&self, state: &mut H) {
        // Implement a custom hashing logic for Loc
        // You should hash the fields that uniquely identify a Loc instance.
        // For example, if Loc has an (x, y) coordinate, you can hash them like this:
        self.col.hash(state);
        self.row.hash(state);
    }
}

impl fmt::Display for Loc {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
      write!(f, "({} {})", self.row, self.col)
    }
}



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


fn breadth_first_search(grid: Grid, start: Loc, dest:Loc) -> Option<Vec<Loc>>{
    let mut explored: HashSet<Loc> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(vec![start]);

    while let Some(path) = queue.pop_front() {
        let node = path.last().cloned().unwrap();

        if !explored.contains(&node) {
            let neighbours = node.neighbours().iter()
                    .filter(|nb| grid.loc(nb).is_some())
                    .cloned()
                    .collect::<Vec<Loc>>();
            for neighbour in neighbours {
                println!("{}", neighbour);
                if grid.loc(&neighbour) <= grid.loc(&node) {
                    let mut new_path = path.clone();
                    new_path.push(neighbour.clone());
                    queue.push_back(new_path.clone());
                    if neighbour == dest {
                        return Some(new_path);
                    }
                }
            }
            explored.insert(node);
        }
    }
    None
}



fn main() {
    let raw_input: String = fs::read_to_string("inputs/day12.in").unwrap();
    println!("{:?}", raw_input);
    let grid = Grid::new(raw_input.as_bytes());
    let start_loc = grid.find(b'S').unwrap();
    let end_loc = grid.find(b'E').unwrap();

    if let Some(path) = breadth_first_search(grid, start_loc, end_loc) {
        println!("found path");
        path.iter().for_each(|loc| println!("{}", loc));
    }
    else {
        println!("no path found");
    }

}