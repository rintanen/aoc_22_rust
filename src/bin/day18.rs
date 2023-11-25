use std::collections::HashSet;
use itertools::{Itertools, iproduct};

fn parse_line(s: &str) -> (i32, i32, i32) {
    let mut iter = s.split(',').map(|s| s.trim().parse::<i32>().expect("Invalid integer"));

    let x = iter.next().expect("Missing x value");
    let y = iter.next().expect("Missing y value");
    let z = iter.next().expect("Missing z value");

    (x, y, z)
}

fn sides_exposed(voxel: &(i32, i32, i32), voxels: &HashSet<(i32, i32, i32)>) -> usize {
    let deltas = [(-1, 0, 0), (1, 0, 0), (0, -1, 0), (0, 1, 0), (0, 0, -1), (0, 0, 1)];
    let n_neighbours = deltas.iter()
        .map(|(dx, dy, dz)| (voxel.0 + dx, voxel.1 + dy, voxel.2 + dz))
        .filter(|neighbour| voxels.contains(neighbour))
        .count();
    6 - n_neighbours
}


fn is_air_pocket(voxel: &(i32, i32, i32), voxels: &HashSet<(i32, i32, i32)>) -> bool {
    sides_exposed(voxel, voxels) == 0
}


fn main() {
    let input = include_str!("../../inputs/day18.in");
    let voxels = input
        .lines()
        .map(|line| parse_line(line))
        .collect::<HashSet<(i32, i32, i32)>>();

    let total_sides_exposed = voxels.iter()
        .map(|voxel| sides_exposed(voxel, &voxels))
        .sum::<usize>();

    let x_range = voxels.iter().map(|(x, _, _)| x).minmax().into_option().unwrap();
    let y_range = voxels.iter().map(|(_, y, _)| y).minmax().into_option().unwrap();
    let z_range = voxels.iter().map(|(_, _, z)| z).minmax().into_option().unwrap();

    let n_air_pockets = iproduct!(*x_range.0..=*x_range.1, *y_range.0..=*y_range.1, *z_range.0..=*z_range.1)
        .filter(|&(x, y, z)| !voxels.contains(&(x, y, z)) && is_air_pocket(&(x, y, z), &voxels))
        .count();

    println!("Part 1: {}", total_sides_exposed);
    println!("Part 2: {}", total_sides_exposed - (6 * n_air_pockets));
}