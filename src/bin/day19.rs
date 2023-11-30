use std::str::FromStr;
use std::collections::HashSet;
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
    fn new() -> Self {
        Self {
            ore_robots: 1,
            ..Default::default()
        }
    }

    fn collect_minerals(self) -> State {
        State {
            ore: (self.ore + self.ore_robots).min(3*4),
            clay: (self.clay + self.clay_robots).min(3*14),
            obsidian: (self.obsidian + self.obsidian_robots).min(3*12),
            geode: self.geode + self.geode_robots,
            ..self
        }
    }

    fn try_build_ore_robot(&self, blueprint: &Blueprint) -> Option<State> {
        if self.ore_robots < 4 && self.ore >= blueprint.ore_robot_cost {
            let mut new_state = self.collect_minerals();
            new_state.ore -= blueprint.ore_robot_cost;
            new_state.ore_robots += 1;
            return Some(new_state);
        }
        None
    }

    fn try_build_clay_robot(&self, blueprint: &Blueprint) -> Option<State> {
        if self.clay_robots < 14 && self.ore >= blueprint.clay_robot_cost {
            let mut new_state = self.collect_minerals();
            new_state.ore -= blueprint.clay_robot_cost;
            new_state.clay_robots += 1;
            return Some(new_state);
        }
        None
    }

    fn try_build_obsidian_robot(&self, blueprint: &Blueprint) -> Option<State> {
        if self.obsidian_robots < blueprint.geode_robot_cost.obsidian
            && self.ore >= blueprint.obsidian_robot_cost.ore
            && self.clay >= blueprint.obsidian_robot_cost.clay
        {
            let mut new_state = self.collect_minerals();
            new_state.ore -= blueprint.obsidian_robot_cost.ore;
            new_state.clay -= blueprint.obsidian_robot_cost.clay;
            new_state.obsidian_robots += 1;
            return Some(new_state);
        }
        None
    }

    fn try_build_geode_robot(&self, blueprint: &Blueprint) -> Option<State> {
        if self.ore >= blueprint.geode_robot_cost.ore
            && self.obsidian >= blueprint.geode_robot_cost.obsidian
        {
            let mut new_state = self.collect_minerals();
            new_state.ore -= blueprint.geode_robot_cost.ore;
            new_state.obsidian -= blueprint.geode_robot_cost.obsidian;
            new_state.geode_robots += 1;
            return Some(new_state);
        }
        None
    }
}


fn create_state_tree(blueprint: &Blueprint, time: u16) -> Vec<State> {
    let mut states = vec![State::new()];
    let mut seen = HashSet::new();
    let mut most_geodes = 0;
    let mut i = 0;
    for t in 0..time {
        let mut next_states = vec![];
        let time_left = time - t - 1;
        for state in states {
            if !seen.insert(state) {
                continue;
            }
            if state.geode + state.geode_robots * 2 * time_left < most_geodes {
                continue
            }
            most_geodes = most_geodes.max(state.geode);
            if let Some(new_state) = state.try_build_geode_robot(blueprint) {
                next_states.push(new_state);
                continue;
            }
            if let Some(new_state) = state.try_build_obsidian_robot(blueprint) {
                next_states.push(new_state);
                continue;
            }
            if let Some(new_state) = state.try_build_ore_robot(blueprint) {
                next_states.push(new_state);
                continue;
            }
            if let Some(new_state) = state.try_build_clay_robot(blueprint) {
                next_states.push(new_state);
                continue;
            }
            next_states.push(state.collect_minerals());
            i = i + 1;
        }
        states = next_states;
        let db = 1;
    }
    states
}


fn main() {
    let input = include_str!("../../inputs/day19.in");
    let blueprints = input
        .split("\n\n")
        .map(|blueprint| blueprint.parse::<Blueprint>().unwrap())
        .collect::<Vec<Blueprint>>();
    dbg!(&blueprints[0]);
    let state_tree = create_state_tree(&blueprints[0], 24);
    println!("len state tree {:?}", state_tree.len());
    print!("max geodes: {:?}", state_tree.iter().map(|state| state.geode).max());
}