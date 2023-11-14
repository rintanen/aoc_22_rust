use nom::{
    bytes::complete::tag,
    character::complete,
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    *,
};
use std::collections::BTreeSet;

// Custom types
#[derive(Debug, Clone)]
struct Beacon {
    x: i64,
    y: i64,
}


#[derive(Debug, Clone)]
struct Sensor {
    x: i64,
    y: i64,
}

impl Sensor {
    fn distance_to_beacon(&self, beacon: &Beacon) -> u64 {
        let dx = self.x.abs_diff(beacon.x);
        let dy = self.y.abs_diff(beacon.y);
        dx + dy
    }
    fn distance_to_row(&self, row: i64) -> u64 {
        self.y.abs_diff(row)
    }
}

#[derive(Debug, Clone, Ord, PartialOrd, PartialEq, Eq)]
struct Range {
    min: i64,
    max: i64,
}

impl Range {
    fn contains(&self, value: i64) -> bool {
        self.min <= value && value <= self.max
    }

    fn overlaps(&self, other: &Self) -> bool {
        self.contains(other.min) || self.contains(other.max)
    }

    fn merge(&self, other: &Self) -> Self {
        Self {
            min: self.min.min(other.min),
            max: self.max.max(other.max),
        }
    }
}

// Parsing
fn position(input: &str) -> IResult<&str, (i64, i64)> {
    separated_pair(
        preceded(tag("x="), complete::i64),
        tag(", "),
        preceded(tag("y="), complete::i64),
    )(input)
}

fn parse_input(input: &str) -> IResult<&str, Vec<(Sensor, Beacon)>> {
    let (input, map) = separated_list1(
        complete::line_ending,
        preceded(
            tag("Sensor at "),
            separated_pair(
                position.map(|(x, y)| Sensor { x, y }),
                tag(": closest beacon is at "),
                position.map(|(x, y)| Beacon { x, y }),
            )
        ),
    )(input)?;
    Ok((input, map))
}

// Solution
fn range_at_given_row(sensor: &Sensor, beacon: &Beacon, row: i64) -> Option<Range> {
    let sensor_beacon_distance = sensor.distance_to_beacon(beacon) as i64;
    let sensor_row_distance = sensor.distance_to_row(row) as i64;
    if sensor_row_distance > sensor_beacon_distance {
        return None;
    }
    let delta_x = sensor_beacon_distance.abs_diff(sensor_row_distance) as i64;
    let range = Range{ min: sensor.x - delta_x, max: sensor.x + delta_x };

    Some(range)
}

fn merge_ranges(ordered_ranges: BTreeSet<Range>) -> Vec<Range> {
    ordered_ranges
        .into_iter()
        .fold(vec![], |mut acc, range| {
            if let Some(last_range) = acc.last_mut() {
                if last_range.overlaps(&range) {
                    *last_range = last_range.merge(&range);
                    return acc;
                }
            }
            acc.push(range);
            acc
        })
}

fn ranges_without_beacon_at_row(
    sensor_beacon_pairs: &Vec<(Sensor, Beacon)>,
    row: i64
) -> Vec<Range> {
    let ordered_ranges = sensor_beacon_pairs
        .iter()
        .filter_map(|(sensor, beacon)| range_at_given_row(sensor, beacon, row))
        .collect::<BTreeSet<Range>>();
    let merged_ranges = merge_ranges(ordered_ranges);
    merged_ranges
}

fn pt1_sum_ranges(ranges: Vec<Range>) -> u64 {
    ranges
        .into_iter()
        .map(|range| (range.max - range.min) as u64)
        .sum()
}

fn pt2_find_isolated_beacon(
    sensor_beacon_pairs: &Vec<(Sensor, Beacon)>,
    search_base: i64
) -> Option<Beacon> {
    for i in 0..search_base + 1 {
        let ranges = ranges_without_beacon_at_row(sensor_beacon_pairs, i);
        if ranges.len() > 1 {
            let hidden_beacon_x = (ranges[1].min + ranges[0].max).div_euclid(2);
            let hidden_beacon = Beacon { x: hidden_beacon_x, y: i };
            return Some(hidden_beacon)
        }
    }
    None
}

fn main() {
    let input = include_str!("../../inputs/day15.in");
    let (_, sensor_beacon_pairs) = parse_input(input).unwrap();
    let ranges = ranges_without_beacon_at_row(&sensor_beacon_pairs, 2000000);
    let cols_without_beacon = pt1_sum_ranges(ranges);
    println!("cols without beacon (at row 10): {:?}", cols_without_beacon);

    let hidden_beacon = pt2_find_isolated_beacon(&sensor_beacon_pairs, 4000000).unwrap();
    let tuning_freq = hidden_beacon.x * 4000000 + hidden_beacon.y;
    println!("hidden beacon: {:?}\ntuning frequency: {}", hidden_beacon, tuning_freq);
}
