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
    fn forward(&self) {

    }

    fn sand_drop(&self) -> bool {
        true
    }
}



fn main() {
    let input = include_str!("../../inputs/day14.in");
    let (_, mut grid) = parse_initial_grid(input).unwrap();

    let mut simulation = SandDropSimulation { grid };

}
