use std::fs;
use std::fmt;
use std::collections::{VecDeque, HashSet};
use std::hash::{Hash, Hasher};


fn in_bounds(loc: &Loc, delta: (i8, i8)) -> bool {
    loc.row as i8 - delta.0 >= 0 && loc.col as i8 - delta.1 >= 0 
}


#[derive(Eq, PartialEq, Clone)]
struct Loc {
    row: usize,
    col: usize
}

impl Loc {
    fn neighbours(&self) -> Vec<Loc> {
        let mut nbs = vec![];
        for delta in [(0, 1), (0, -1), (1, 0), (-1, 0)] {
            if in_bounds(&self, delta) {
                let row = self.row as i8 - delta.0;
                let col = self.col as i8 - delta.1;
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
        Grid { array: arr, shape: (rows as usize, cols )}
    }

    fn loc(&self, location: &Loc) -> Option<&'a u8> {
        let index_1d = location.col + self.shape.1 * location.row;
        // add row's index to the 2d->1d grid mapping index 
        // because there is newline character between each row
        // NOTE: don't need to check that row and col are >= 0, because usize is always positive
        if location.row <= self.shape.0 && location.col <= self.shape.1 {
            let res = Some(&self.array[index_1d + location.row]);
            // exception for start and end location
            if res == Some(&83) {
                return Some(&97);
            } else if res == Some(&69) {
                return Some(&122);
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


fn breadth_first(grid: Grid, start: Loc, dest:Loc) -> Option<Vec<Loc>>{
    let mut explored: HashSet<Loc> = HashSet::new();
    let mut queue = VecDeque::new();
    queue.push_back(vec![start]);

    while let Some(path) = queue.pop_front() {
        let node = path.last().cloned().unwrap();
        
        if node == dest {
            return Some(path)
        }

        if !explored.contains(&node) {
            // let neighbours = node.neighbours();
            for neighbour in node.neighbours() {
                println!("{}", neighbour);
                if grid.loc(&neighbour) <= grid.loc(&node) {
                    let mut new_path = path.clone();
                    new_path.push(neighbour.clone());
                    queue.push_back(new_path.clone());
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
    let start_loc = grid.find('S' as u8).unwrap();
    let end_loc = grid.find('E' as u8).unwrap();

    let path = breadth_first(grid, start_loc, end_loc).unwrap();

    path.iter().for_each(|loc| println!("{}", loc));
}
