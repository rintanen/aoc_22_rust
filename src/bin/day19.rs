use std::str::FromStr;

use itertools::Itertools;

#[derive(Debug)]
struct ObsidianRobotCost {
    ore: u16,
    clay: u16,
}


impl ObsidianRobotCost {
    fn new(ore: u16, clay: u16) -> Self {
        Self { ore, clay }
    }
}


#[derive(Debug)]
struct GeodeRobotCost {
    ore: u16,
    obsidian: u16,
}

impl GeodeRobotCost {
    fn new(ore: u16, obsidian: u16) -> Self {
        Self { ore, obsidian }
    }
}


#[derive(Debug)]
struct Blueprint {
    id: u16,
    ore_robot_cost: u16,
    clay_robot_cost: u16,
    obsidian_robot_cost: ObsidianRobotCost,
    geode_robot_cost: GeodeRobotCost
}

impl FromStr for Blueprint {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let lines: Vec<&str> = s.lines().collect();
        let id = extract_num(lines[0], "Blueprint ", ":");
        let ore_robot_cost = extract_num(lines[1], "Each ore robot costs ", " ore.");
        let clay_robot_cost = extract_num(lines[2], "Each clay robot costs ", " ore.");
        let obsidian_robot_cost = extract_tuple_num(lines[3]);
        let geode_robot_cost = extract_tuple_num(lines[4]);

        Ok(
            Blueprint {
                id,
                ore_robot_cost,
                clay_robot_cost,
                obsidian_robot_cost: ObsidianRobotCost::new(obsidian_robot_cost.0, obsidian_robot_cost.1),
                geode_robot_cost: GeodeRobotCost::new(geode_robot_cost.0, geode_robot_cost.1)
            }
        )
    }
}

fn extract_num(s: &str, trim_from_start: &str, trim_from_end: &str) -> u16 {
    s.trim()
        .trim_start_matches(trim_from_start)
        .trim_end_matches(trim_from_end)
        .parse()
        .unwrap()
}

fn extract_tuple_num(s: &str) -> (u16, u16) {
    s.trim()
        .split_whitespace()
        .skip(4)
        .filter_map(|s| s.parse().ok())
        .collect_tuple()
        .unwrap()
}

#[derive(Default, Clone, Copy, Hash, PartialEq, Eq)]
struct State {
    ore: u16,
    clay: u16,
    obsidian: u16,
    geode: u16,
    ore_robots: u16,
    clay_robots: u16,
    obsidian_robots: u16,
    geode_robots: u16,
}

impl State {
    fn collect_minerals(&mut self) {
        self.ore += self.ore_robots;
        self.clay += self.clay_robots;
        self.obsidian += self.obsidian_robots;
        self.geode += self.geode_robots;
    }

    fn try_build_ore_robot(&mut self, blueprint: &Blueprint) -> Option<self> {
        if self.ore >= blueprint.ore_robot_cost {
            self.ore -= blueprint.ore_robot_cost;
            self.ore_robots += 1;
            Some(*self)
        }
        None
    }

    fn try_build_clay_robot(&mut self, blueprint: &Blueprint) -> Option<self> {
        if self.ore >= blueprint.clay_robot_cost {
            self.ore -= blueprint.clay_robot_cost;
            self.clay_robots += 1;
            Some(*self)
        }
        None
    }

    fn try_build_obsidian_robot(&mut self, blueprint: &Blueprint) -> Option<self> {
        if self.ore >= blueprint.obsidian_robot_cost.ore && self.clay >= blueprint.obsidian_robot_cost.clay {
            self.ore -= blueprint.obsidian_robot_cost.ore;
            self.clay -= blueprint.obsidian_robot_cost.clay;
            self.obsidian_robots += 1;
            Some(*self)
        }
        None
    }

    fn try_build_geode_robot(&mut self, blueprint: &Blueprint) -> Option<self> {
        if self.ore >= blueprint.geode_robot_cost.ore && self.obsidian >= blueprint.geode_robot_cost.obsidian {
            self.ore -= blueprint.geode_robot_cost.ore;
            self.obsidian -= blueprint.geode_robot_cost.obsidian;
            self.geode_robots += 1;
            Some(*self)
        }
        None
    }
}

fn main() {
    let input = include_str!("../../inputs/day19.in");
    let blueprints = input
        .split("\n\n")
        .map(|blueprint| blueprint.parse::<Blueprint>().unwrap())
        .collect::<Vec<Blueprint>>();
    dbg!(blueprints);
}