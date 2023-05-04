use std::fs;

struct Grid {
    array: Vec<u8>,
    shape: (usize, usize),
}

impl Grid {
    fn new(arr: String) -> Grid {
        let cols = arr.as_bytes().iter().position(|&x| x == 10).unwrap();
        let rows = arr.len() / cols;
        Grid {
            array: arr.replace("\n", "").chars().map(|c| c as u8).collect(),
            shape: (rows as usize, cols),
        }
    }

    fn loc(&self, row: usize, col: usize) -> Option<u8> {
        let i = col + self.shape.1 * row;
        if row <= self.shape.0 && col <= self.shape.1 {
            Some(self.array[i])
        } else {
            None
        }
    }

    fn find(&self, search_entry: u8) -> Option<(usize, usize)> {
        /*
        returns the first location where search_entry is found.
        */
        let pos = self.array.iter().position(|&x| x == search_entry);
        match pos {
            Some(i) => Some((i / self.shape.1, i % self.shape.1)),
            _ => None,
        }
    }
}

fn find_path_breadth_first(grid: &Grid, starting_loc: (usize, usize), destination: u8) -> Vec<u8> {
    let explored = vec![];
    let queue = vec![vec![starting_loc]];
    

    vec![1,2,3]
}

fn main() {
    let raw_input: String = fs::read_to_string("../../inputs/day12.in").unwrap();
    let grid = Grid::new(raw_input);
    let start = grid.find('S' as u8).unwrap();
    let end = grid.find('E' as u8).unwrap();

    let path = find_path_breadth_first(&grid, start, 'E' as u8);

}
