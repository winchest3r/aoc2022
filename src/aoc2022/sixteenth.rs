use std::collections::{HashMap, VecDeque};

use crate::aoc2022::Solution;

#[derive(Debug, Clone)]
struct Valve {
    pub flow: i64,
    pub distances: Vec<u64>,
}

struct State {
    pub position: usize,
    pub time_left: u64,
    pub opened: u64,
}

pub struct SixteenthSolution;

impl Solution for SixteenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let (map, flow) = get_valve_map_optimized(input);

        let state = State {
            position: 0,
            time_left: 30,
            opened: 0,
        };

        pressure_calc_dfs(&map, &flow, state).to_string()
    }
}

fn pressure_calc_dfs(map: &[Vec<usize>], flow: &[Option<Valve>], state: State) -> u64 {
    let mut max_pressure = 0;

    // try for every non-opened and reachable
    for &next_idx in &map[state.position] {
        let bit = 1 << next_idx;

        // if opened
        if state.opened & bit != 0 {
            continue;
        }
        let valve = flow[next_idx].as_ref().unwrap();

        // state.position -> next_idx
        let distance = flow[state.position].as_ref().unwrap().distances[next_idx];

        // not enough time
        let cost = distance + 1;
        if cost >= state.time_left {
            continue;
        }
        let remaining_time = state.time_left - cost;

        // get pressure
        let pressure = remaining_time * valve.flow as u64;
        let next_state = State {
            position: next_idx,
            time_left: remaining_time,
            opened: state.opened | bit,
        };
        // and recursively processs through valves
        let total_pressure = pressure + pressure_calc_dfs(map, flow, next_state);
        // and save best result
        max_pressure = max_pressure.max(total_pressure);
    }

    max_pressure
}

/// Returns all adjacent valves with significant flow.
fn valve_optimization_dfs(map: &[Vec<usize>], flow: &[i64], idx: usize) -> Vec<(usize, u64)> {
    let mut visited: usize = 1 << idx;
    let mut queue = VecDeque::new();
    queue.push_back((idx, 0));

    let mut result = Vec::new();

    while let Some((node, dist)) = queue.pop_front() {
        if flow[node] > 0 {
            result.push((node, dist));
        }

        for &next in &map[node] {
            if visited & (1 << next) == 0 {
                visited |= 1 << next;
                queue.push_back((next, dist + 1));
            }
        }
    }

    result
}

fn get_valve_map_optimized(input: &str) -> (Vec<Vec<usize>>, Vec<Option<Valve>>) {
    // there is a lot of valves with flow = 0 -> we can remove them and just
    // save distance between signinficant valves.
    // only exclusion is for AA (idx=0) - because it's starting point.

    let (map, flow): (Vec<Vec<usize>>, Vec<i64>) = gev_valve_map(input);

    let mut map_opt: Vec<Vec<usize>> = Vec::new();
    map_opt.resize(map.len(), Vec::new());
    let mut flow_opt: Vec<Option<Valve>> = Vec::new();
    flow_opt.resize(flow.len(), None);

    for idx in 0..map.len() {
        // skip zero flowed except AA
        if flow[idx] == 0 && idx != 0 {
            continue;
        }

        let mut valve = Valve {
            flow: flow[idx],
            distances: Vec::new(),
        };
        valve.distances.resize(flow.len(), 0);

        for (adj_idx, adj_dist) in valve_optimization_dfs(&map, &flow, idx) {
            valve.distances[adj_idx] = adj_dist;
            map_opt[idx].push(adj_idx);
        }
        flow_opt[idx] = Some(valve);
    }

    (map_opt, flow_opt)
}

fn gev_valve_map(input: &str) -> (Vec<Vec<usize>>, Vec<i64>) {
    let regex = regex::regex!(
        r#"Valve (?P<in>\w+) has flow rate=(?P<flow>\d+); tunnels? leads? to valves? (?<out>.*)"#
    );

    // get data represented by strings
    let mut string_map: HashMap<String, Vec<String>> = HashMap::new();
    let mut string_flow: HashMap<String, i64> = HashMap::new();
    for cap in regex.captures_iter(input.trim()) {
        let valve_in = cap["in"].to_string();
        let flow_rate = cap["flow"].parse::<i64>().unwrap();
        let valves_out = cap["out"].to_string();
        for valve_out in valves_out.split(", ") {
            string_map
                .entry(valve_in.clone())
                .or_default()
                .push(valve_out.to_string());
            string_map
                .entry(valve_out.to_string())
                .or_default()
                .push(valve_in.to_string());
        }
        string_flow.insert(valve_in.clone(), flow_rate);
    }

    // optimize using integers
    let indices = get_valve_indices(&string_map);
    let mut map: Vec<Vec<usize>> = Vec::new();
    map.resize(string_map.len(), Vec::new());
    let mut flow: Vec<i64> = Vec::new();
    flow.resize(string_flow.len(), 0);
    for (valve, neighbors) in string_map {
        let valve_idx = indices[&valve];
        for adj in neighbors {
            let adj_idx = indices[&adj];
            if !map[valve_idx].contains(&adj_idx) {
                map[valve_idx].push(adj_idx);
            }
        }
        let flow_value = string_flow[&valve];
        flow[valve_idx] = flow_value;
    }

    (map, flow)
}

fn get_valve_indices(valves: &HashMap<String, Vec<String>>) -> HashMap<String, usize> {
    let mut result = HashMap::new();

    let mut keys: Vec<String> = valves.keys().cloned().collect();
    // sort so AA will be at index 0
    keys.sort();
    for (idx, valve) in keys.into_iter().enumerate() {
        result.insert(valve, idx);
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
Valve BB has flow rate=13; tunnels lead to valves CC, AA
Valve CC has flow rate=2; tunnels lead to valves DD, BB
Valve DD has flow rate=20; tunnels lead to valves CC, AA, EE
Valve EE has flow rate=3; tunnels lead to valves FF, DD
Valve FF has flow rate=0; tunnels lead to valves EE, GG
Valve GG has flow rate=0; tunnels lead to valves FF, HH
Valve HH has flow rate=22; tunnel leads to valve GG
Valve II has flow rate=0; tunnels lead to valves AA, JJ
Valve JJ has flow rate=21; tunnel leads to valve II
"#;

    #[test]
    fn day16_part1_example1() {
        let actual = SixteenthSolution::part1(INPUT);
        assert_eq!("1651", actual);
    }

    #[test]
    fn day16_part2_example1() {
        let actual = SixteenthSolution::part2(INPUT);
        assert_eq!("1707", actual);
    }
}
