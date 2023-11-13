use std::collections::HashSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete,
    character::complete::newline,
    multi::separated_list1, sequence::separated_pair, *,
};


fn one_line(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (input, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(
            complete::u32,
            complete::char(','),
            complete::u32,
        ),
    )(input)?;
    let rocks_from_one_line = pairs.into_iter().tuple_windows().flat_map(
        |((ax, ay), (bx, by))| {
            let x_min = ax.min(bx);
            let x_max = ax.max(bx);
            let x_range = x_min..=x_max;

            let y_min = ay.min(by);
            let y_max = ay.max(by);
            let y_range = y_min..=y_max;
            x_range.cartesian_product(y_range)
        },
    );
    Ok((input, rocks_from_one_line))
}

fn parse_initial_grid(input: &str) -> IResult<&str, Grid> {
    let (input, rocks_from_all_lines) = separated_list1(newline, one_line)(input)?;
    let rocks: HashSet<(u32, u32)> = rocks_from_all_lines.into_iter().flatten().collect();
    let lowest_rocks = rocks.iter().map(|(_, y)| y).max().cloned().unwrap();
    let grid = Grid { arr: rocks, lowest_rocks };
    Ok((input, grid))
}

#[derive(Debug, Clone)]
struct Grid {
    arr: HashSet<(u32, u32)>,
    lowest_rocks: u32,
}

impl Grid {
}

#[derive(Debug)]
enum Part {
    PT1,
    PT2,
}

struct SandDropSimulation {
    grid: Grid,
    part: Part
}

impl SandDropSimulation {
    fn execute(&mut self) {
        let number_of_stones = self.grid.arr.len();
        match self.part {
            Part::PT1 => self.pt1_sand_drop(),
            Part::PT2 => self.pt2_sand_drop(),
        }
        let units_sands_dropped = self.grid.arr.len() - number_of_stones;
        println!("{:?}: {} units of sand dropped before condition is met",
                 self.part, units_sands_dropped);
    }

    fn pt1_sand_drop(&mut self) {
        let mut sand_location = (500, 0);
        loop {
            let (x, y) = sand_location;

            let below = (x, y + 1);
            let below_left = (x - 1, y + 1);
            let below_right = (x + 1, y + 1);

            if y > self.grid.lowest_rocks {
                return;
            }

            match (self.grid.arr.get(&below),
                   self.grid.arr.get(&below_left),
                   self.grid.arr.get(&below_right)) {
                (Some(_), Some(_), Some(_)) => {
                    // comes to rest drop new sand at origin (500, 0)
                    self.grid.arr.insert((x, y));
                    sand_location = (500, 0);
                }
                (None, _, _) => {
                    // free fall
                    sand_location = below;
                }
                (Some(_), None, _) => {
                    // spread left
                    sand_location = below_left;
                }
                (Some(_), Some(_), None) => {
                    // spread right
                    sand_location = below_right;
                }
            }
        }
    }

    fn pt2_sand_drop(&mut self) {
        let mut sand_location = (500, 0);
        let floor_level = self.grid.lowest_rocks + 2;

        while let None = self.grid.arr.get(&(500, 0)) {
            let (x, y) = sand_location;

            let below = (x, y + 1);
            let below_left = (x - 1, y + 1);
            let below_right = (x + 1, y + 1);

            if below.1 == floor_level {
                self.grid.arr.insert(below);
                self.grid.arr.insert(below_left);
                self.grid.arr.insert(below_right);
            }

            match (self.grid.arr.get(&below),
                   self.grid.arr.get(&below_left),
                   self.grid.arr.get(&below_right)) {
                (Some(_), Some(_), Some(_)) => {
                    // comes to rest drop new sand at origin (500, 0)
                    self.grid.arr.insert((x, y));
                    sand_location = (500, 0);
                }
                (None, _, _) => {
                    // free fall
                    sand_location = below;
                }
                (Some(_), None, _) => {
                    // spread left
                    sand_location = below_left;
                }
                (Some(_), Some(_), None) => {
                    // spread right
                    sand_location = below_right;
                }
            }
        }
        // remove floor level from arr, because of the way we calculate sands in execute()
        self.grid.arr.retain(|(_, y)| y != &floor_level);
    }
}


fn main() {
    let input = include_str!("../../inputs/day14.in");
    let (_, grid) = parse_initial_grid(input).unwrap();

    // pt1
    let mut pt_1_simulation = SandDropSimulation {
        grid: grid.clone(),
        part: Part::PT1
    };
    pt_1_simulation.execute();

    // pt2
    let mut pt_2_simulation = SandDropSimulation {
        grid: grid.clone(),
        part: Part::PT2
    };
    pt_2_simulation.execute();
}