use std::collections::{BTreeSet, HashSet};

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete,
    character::complete::{newline, line_ending},
    multi::separated_list1, sequence::separated_pair, *,
};


fn line(input: &str) -> IResult<&str, impl Iterator<Item = (i32, i32)>> {
    let (_, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(
            complete::i32,
            complete::char(','),
            complete::i32,
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
    let (input, rocks_from_all_lines) = separated_list1(line_ending, line)(input)?;
    // menee vituiksi tässä ei tule kun eka rivi
    let rocks: BTreeSet<(i32, i32)> = rocks_from_all_lines.into_iter().flatten().collect();
    let floor_level = rocks.iter().map(|(_, y)| y).max().cloned().unwrap();
    let grid = Grid { arr: rocks, floor_level };
    Ok((input, grid))
}

#[derive(Debug, Clone)]
struct Grid {
    arr: BTreeSet<(i32, i32)>,
    floor_level: i32,
}

impl Grid {
}


struct SandDropSimulation {
    grid: Grid,
    sand_drop_stop_condition: fn(i32, i32, i32) -> bool,
}

impl SandDropSimulation {
    fn execute(&mut self) {
        let number_of_stones = self.grid.arr.len();
        self.sand_drop();
        let units_sands_dropped = self.grid.arr.len() - number_of_stones;
        println!("{}", units_sands_dropped);
    }

    fn sand_drop(&mut self) {
        let initial_sand_location = (500, 0);
        let mut sand_location = initial_sand_location.clone();
        loop {
            let (x, y) = sand_location;

            let below = (x, y + 1);
            let below_left = (x - 1, y + 1);
            let below_right = (x + 1, y + 1);

            if (self.sand_drop_stop_condition)(y, self.grid.floor_level, initial_sand_location.1){
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
}

fn execute_simulation(simulation: &mut SandDropSimulation) {
    let number_of_stones = simulation.grid.arr.len();
    simulation.sand_drop();
    let units_sands_dropped = simulation.grid.arr.len() - number_of_stones;
    println!("{}", units_sands_dropped);
}


fn main() {
    let input = include_str!("../../inputs/day14.in");
    let (_, grid) = parse_initial_grid(input).unwrap();

    for element in &grid.arr {
        println!("{:?}", element);
    }

    let mut pt_1_simulation = SandDropSimulation {
        grid: grid.clone(),
        sand_drop_stop_condition: |y, floor_level, _| {
            y > floor_level
        },
    };
    println!("PT1: ");
    pt_1_simulation.execute();


    let mut pt_2_simulation = SandDropSimulation {
        grid,
        sand_drop_stop_condition: |y, _, initial_sand_level| {
            y == initial_sand_level
        },

    };
    println!("PT2: ");
    pt_2_simulation.execute();

}
