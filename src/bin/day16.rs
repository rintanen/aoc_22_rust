use itertools::{Itertools, iproduct};

// parsing
fn parse_input(input: &str) -> Vec<(&str, u32, Vec<&str>)> {
    input
        .lines()
        .map(|line| parse_line(line))
        .sorted_by_key(|(_, flow_rate, _)| *flow_rate)
        .collect()
}

fn parse_line(line: &str) -> (&str, u32, Vec<&str>) {
    let mut parts_to_take = line
        .split(|c| not_uppercase_or_numeric(c))
        .filter(|s| !s.is_empty())
        .skip(1);

    let name = parts_to_take.next().unwrap();
    let flow_rate = parts_to_take.next().unwrap().parse().unwrap();
    let neighbours = parts_to_take.collect();

    (name, flow_rate, neighbours)
}

fn not_uppercase_or_numeric(c: char) -> bool {
    !c.is_uppercase() && !c.is_ascii_digit()
}

// solution
fn create_initial_graph(valves: &Vec<&str>, neighbours: Vec<Vec<&str>>) -> Vec<Vec<u32>> {
    let inf = u32::MAX;
    let mut graph = vec![vec![inf; valves.len()]; valves.len()];
    for i in 0..valves.len() {
        for neighbour in neighbours[i].iter() {
            let neighbour_index = valves.iter().position(|v| v == neighbour).unwrap();
            graph[i][i] = 0;
            graph[i][neighbour_index] = 1;
        }
    }
    graph
}

fn shortest_distance_between_valves(graph: &mut Vec<Vec<u32>>) {
    /*
    floyd-warshall algorithm
    https://favtutor.com/blogs/floyd-warshall-algorithm
    */
    let n = graph.len();
    for (r, p, q) in iproduct!(0..n, 0..n, 0..n) {
        let possible_new_value = match graph[p][r].checked_add(graph[r][q]) {
            Some(sum) => sum,
            None => u32::MAX,
        };
        graph[p][q] = graph[p][q].min(possible_new_value);
    }
}


struct ValveFars<'a> {
    valve_names: &'a Vec<&'a str>,
    flow_rates: &'a Vec<u32>,
    distances: &'a Vec<Vec<u32>>,
    opened_valves: Vec<&'a str>,
    non_zero_flow_rates: Vec<usize>,
}

impl <'a> ValveFars<'a> {
    fn new(valve_names: &'a Vec<&'a str>, flow_rates: &'a Vec<u32>, distances: &'a Vec<Vec<u32>>) -> Self {
        let non_zero_flow_rates: Vec<usize> = flow_rates
            .iter()
            .enumerate()
            .filter(|&(_, &value)| value > 0)
            .map(|(index, _)| index)
            .collect();
        Self {
            valve_names,
            flow_rates,
            distances,
            opened_valves: vec![],
            non_zero_flow_rates
        }
    }

    fn depth_first_search(&mut self, i_current_valve: usize, time_elapsed: u32, total_released: u32) -> u32 {
        let mut current_total = self.total_flow_rate() * (30 - time_elapsed);

        for i_valve in self.non_zero_flow_rates.clone().iter() {
            if self.opened_valves.contains(&self.valve_names[*i_valve]) {
                continue;
            }
            let time_to_next = 1 + self.distances[i_current_valve][*i_valve];

            if (time_elapsed + time_to_next) >= 30 {
                continue;
            }

            let new_total = total_released + time_to_next * self.total_flow_rate();

            self.opened_valves.push(self.valve_names[*i_valve]);

            let max_from_this_valve = self.depth_first_search(
                *i_valve, time_elapsed + time_to_next, new_total
            );

            if max_from_this_valve > current_total {
                current_total = max_from_this_valve;
            }

            self.opened_valves.pop();
        }
        current_total
    }

    fn total_flow_rate(&self) -> u32 {
        self.opened_valves
            .iter()
            .filter_map(|&valve| self.valve_names.iter().position(|v| *v == valve))
            .map(|i| self.flow_rates[i])
            .sum()
    }
}



fn main() {
    let input = include_str!("../../inputs/day16.in");
    let input = parse_input(input);

    let flow_rates = input.iter().map(|(_, flow_rate, _)| *flow_rate).collect::<Vec<u32>>();
    let valve_names = input.iter().map(|(label, _, _)| *label).collect::<Vec<&str>>();
    let neighbours = input
        .iter()
        .map(|(_, _, ref neighbours)| neighbours.clone())
        .collect::<Vec<Vec<&str>>>();

    let mut distances = create_initial_graph(&valve_names, neighbours);
    shortest_distance_between_valves(&mut distances);


    let mut valve_fars = ValveFars::new(&valve_names, &flow_rates, &distances);

    let start_idx = valve_names.iter().position(|&v| v == "AA").unwrap();

    let max_flow_rate = valve_fars.depth_first_search(start_idx, 0, 0);

    println!("{:?}", flow_rates);
    println!("{:?}", valve_names);
    println!("{}", max_flow_rate);
}