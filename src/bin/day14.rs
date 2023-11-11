use std::collections::BTreeSet;

use itertools::Itertools;
use nom::{
    bytes::complete::tag, character::complete,
    character::complete::line_ending,
    multi::separated_list1, sequence::separated_pair, *,
};


fn line(input: &str) -> IResult<&str, impl Iterator<Item = (u32, u32)>> {
    let (_, pairs) = separated_list1(
        tag(" -> "),
        separated_pair(
            complete::u32,
            complete::char(','),
            complete::u32,
        ),
    )(input)?;
    let it = pairs.into_iter().tuple_windows().flat_map(
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
    Ok((input, it))
}

fn parse_initial_grid(input: &str) -> IResult<&str, Grid> {
    let (input, pairs) = separated_list1(line_ending, line)(input)?;
    let map = pairs.into_iter().flatten().collect();
    let grid = Grid { arr: map };
    Ok((input, grid))
}


struct Grid {
    arr: BTreeSet<(u32, u32)>,
}

impl Grid {
}


struct SandDropSimulation {
    grid: Grid,
}


impl SandDropSimulation {
    fn forward(&mut self) {
        self.sand_drop((500, 0));
    }

    fn sand_drop(&mut self, sand_location: (u32, u32)) {
        let (x, y) = sand_location;
        let below = self.grid.arr.get(&(x, y - 1));
        let below_left = self.grid.arr.get(&(x - 1, y - 1));
        let below_right = self.grid.arr.get(&(x + 1, y - 1));

        match (below, below_left, below_right) {
            (Some(_), Some(_), Some(_)) => {
                // comes to rest drop new sand
                self.grid.arr.insert((x, y));
                self.sand_drop((500, 0))
            }
            (None, _, _) => {
                // free fall
                self.sand_drop((x, y - 1))
            }
            (Some(_), None, _) => {
                // spread left
                self.sand_drop((x - 1, y - 1))
            }
            (Some(_), Some(_), None) => {
                // spread right
                self.sand_drop((x + 1, y - 1))
            }
        }
    }
}



fn main() {
    let input = include_str!("../../inputs/day14.in");
    let (_, mut grid) = parse_initial_grid(input).unwrap();

    let number_of_stones = grid.arr.len();
    let mut simulation = SandDropSimulation { grid };

}
