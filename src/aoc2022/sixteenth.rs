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
    pub pressure: u64,
}

pub struct SixteenthSolution;

impl Solution for SixteenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let (map, flow) = get_valve_map_optimized(input);
        let total_results = 1 << (map.len() - 1); // enough without AA (doubled in size with it)

        let state = State {
            position: 0,
            time_left: 30,
            opened: 0,
            pressure: 0,
        };

        let mut results: Vec<u64> = vec![0u64; total_results];
        pressure_calc_dfs(&map, &flow, state, &mut results);
        results.iter().max().unwrap().to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let (map, flow) = get_valve_map_optimized(input);
        let total_results = 1 << (map.len() - 1);

        let state = State {
            position: 0,
            time_left: 26,
            opened: 0,
            pressure: 0,
        };
        let mut results: Vec<u64> = vec![0u64; total_results];
        results.resize(total_results, 0);
        pressure_calc_dfs(&map, &flow, state, &mut results);

        // subset max propagation
        let n = map.len() - 1;
        for bit in 0..n {
            for mask in 0..(1 << n) {
                if mask & (1 << bit) != 0 {
                    results[mask] = results[mask].max(results[mask ^ (1 << bit)]);
                }
            }
        }

        // mask disjoint of our best movements and elephant's best - best approach overall
        let answer = (0..results.len())
            .map(|mask| results[mask] + results[(results.len() - 1) ^ mask])
            .max()
            .unwrap();
        answer.to_string()
    }
}

fn pressure_calc_dfs(map: &[Vec<usize>], flow: &[Valve], state: State, results: &mut [u64]) {
    results[state.opened as usize] = results[state.opened as usize].max(state.pressure);

    // try for every non-opened and reachable
    for &next_idx in &map[state.position] {
        let bit = 1 << (next_idx - 1);

        // if opened
        if state.opened & bit != 0 {
            continue;
        }
        let valve = &flow[next_idx];

        // state.position -> next_idx
        let distance = flow[state.position].distances[next_idx];

        // not enough time
        let cost = distance + 1;
        if cost >= state.time_left {
            continue;
        }
        let remaining_time = state.time_left - cost;

        // get pressure
        let next_state = State {
            position: next_idx,
            time_left: remaining_time,
            opened: state.opened | bit,
            pressure: state.pressure + remaining_time * valve.flow as u64,
        };
        // and recursively processs through valves
        pressure_calc_dfs(map, flow, next_state, results);
    }
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

fn get_valve_map_optimized(input: &str) -> (Vec<Vec<usize>>, Vec<Valve>) {
    // there is a lot of valves with flow = 0 -> we can remove them and just
    // save distance between signinficant valves.
    // only exclusion is for AA (idx=0) - because it's starting point.

    let (map, flow): (Vec<Vec<usize>>, Vec<i64>) = gev_valve_map(input);

    // build index mappings without 0 flow valves.
    let mut old_to_new = vec![usize::MAX; map.len()];
    let mut new_to_old = Vec::new();

    for idx in 0..map.len() {
        if flow[idx] > 0 || idx == 0 {
            old_to_new[idx] = new_to_old.len();
            new_to_old.push(idx);
        }
    }

    let useful_count = new_to_old.len();

    let mut map_opt = vec![Vec::new(); useful_count];
    let mut flow_opt = Vec::with_capacity(useful_count);

    for (new_idx, &old_idx) in new_to_old.iter().enumerate() {
        let mut valve = Valve {
            flow: flow[old_idx],
            distances: vec![0; useful_count],
        };

        for (adj_old_idx, distance) in valve_optimization_dfs(&map, &flow, old_idx) {
            let adj_new_idx = old_to_new[adj_old_idx];

            valve.distances[adj_new_idx] = distance;
            map_opt[new_idx].push(adj_new_idx);
        }

        flow_opt.push(valve);
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
