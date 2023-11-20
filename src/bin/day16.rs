use itertools::Itertools;
use itertools::iproduct;
// #[derive(Debug)]
// struct Valve<'a> {
//     name: &'a str,
//     flow_rate: u32,
//     neighbours: Vec<&'a str>,
// }

// parsing
fn parse_input(input: &str) -> Vec<(&str, i32, Vec<&str>)> {
    input
        .lines()
        .map(|line| parse_line(line))
        .sorted_by_key(|(_, flow_rate, _)| -(*flow_rate))
        .collect()
}

fn parse_line(line: &str) -> (&str, i32, Vec<&str>) {
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

fn create_initial_graph(valves: Vec<&str>, neighbours: Vec<Vec<&str>>) -> Vec<Vec<u32>> {
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



fn main() {
    let input = include_str!("../../inputs/day16.in");
    let valves = parse_input(input);


    let labels = valves.iter().map(|(label, _, _)| *label).collect::<Vec<&str>>();
    let neighbours = valves
        .iter()
        .map(|(_, _, ref neighbours)| neighbours.clone())
        .collect::<Vec<Vec<&str>>>();

    let mut graph = create_initial_graph(labels, neighbours);
    shortest_distance_between_valves(&mut graph);


    println!("{:?}", graph);
    println!("{:?}", valves);
}