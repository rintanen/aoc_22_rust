
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

    fn loc(&self, row: usize, col: usize) -> Option<&'a u8> {
        let i = col + self.shape.1 * row;
        // add row's index to the 2d->1d grid mapping index 
        // because there is newline character between each row
        // NOTE: don't need to check that row and col are >= 0, because usize is always positive
        if row <= self.shape.0 && col <= self.shape.1 {
            Some(&self.array[i + row])
        } else {
            None
        }
    }
}

fn study_tree(row: usize, col: usize, grid: &Grid) -> (bool, u32) {
    let tree = grid.loc(row, col);
    let mut visible = false;
    let mut visible_left = 0;
    let mut visible_right = 0;
    let mut visible_up = 0;
    let mut visible_down = 0;

    // check left
    for c in (0..=col - 1).rev() {
        visible_left += 1;
        if grid.loc(row, c) >= tree {
            break;
        }
        if c == 0 {
            visible = true;
        }
    }

    // check right
    for c in col  + 1..grid.shape.1 {
        visible_right += 1;
        if grid.loc(row, c) >= tree {
            break;
        }
        if c == grid.shape.1 - 1 {
            visible = true;
        }
    }

    // check up
    for r in (0..=row - 1).rev() {
        visible_up += 1;
        if grid.loc(r, col) >= tree {
            break;
        }
        if r == 0 {
            visible = true;
        }
    }

    // check down
    for r in row + 1..grid.shape.0 {
        visible_down += 1;
        if grid.loc(r, col) >= tree {
            break;
        }
        if r == grid.shape.1 - 1 {
            visible = true;
        }
    }

    (
        visible, 
        visible_left * visible_right * visible_up * visible_down
    )
}



fn main() {
    let raw_input = include_bytes!("../../inputs/day08.in");
    let grid = Grid::new(raw_input);

    let mut highest_scenic_score = 0;
    let mut visible_trees = 0;

    for r in 1..grid.shape.0 - 1 {
        for c in 1..grid.shape.1 - 1 {
            let (visible, scenic_score) = study_tree(r, c, &grid);

            if visible {
                visible_trees += 1;
            }

            if scenic_score >= highest_scenic_score {
                highest_scenic_score = scenic_score;
            }

        }
    }
    println!("visible trees: {}", visible_trees + 2 * grid.shape.0 + 2 * grid.shape.1 - 4);
    println!("highest scenic score: {}", highest_scenic_score);
}