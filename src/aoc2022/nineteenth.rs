use std::collections::HashMap;

use crate::aoc2022::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Blueprint {
    pub blueprint_id: u64,
    pub ore_robot_cost_ore: u64,
    pub clay_robot_cost_ore: u64,
    pub obsidian_robot_cost_ore: u64,
    pub obsidian_robot_cost_clay: u64,
    pub geode_robot_cost_ore: u64,
    pub geode_robot_cost_obsidian: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct State {
    pub minutes_left: u64,

    pub ore: u64,
    pub clay: u64,
    pub obsidian: u64,

    pub ore_robots: u64,
    pub clay_robots: u64,
    pub obsidian_robots: u64,
    pub geode_robots: u64,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Action {
    BuildOreRobot,
    BuildClayRobot,
    BuildObsidianRobot,
    BuildGeodeRobot,
}

const ACTIONS: [Action; 4] = [
    Action::BuildGeodeRobot,
    Action::BuildObsidianRobot,
    Action::BuildClayRobot,
    Action::BuildOreRobot,
];

pub struct NineteenthSolution;

impl Solution for NineteenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let blueprints = get_blueprints(input);
        let state = State {
            minutes_left: 24,

            ore: 0,
            clay: 0,
            obsidian: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        };
        let mut quality_level = 0;
        for bp in blueprints.iter() {
            let best_geodes = get_most_geodes(state, bp);
            quality_level += best_geodes * bp.blueprint_id;
        }
        quality_level.to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        // main optimization for solving - memoization
        // optimizations trigger cache hits which drastically decrease
        // memory consumption and steps

        let blueprints = get_blueprints(input);
        let state = State {
            minutes_left: 32,

            ore: 0,
            clay: 0,
            obsidian: 0,

            ore_robots: 1,
            clay_robots: 0,
            obsidian_robots: 0,
            geode_robots: 0,
        };
        let mut geodes_mul = 1;
        for bp in blueprints.iter().take(3) {
            let best_geodes = get_most_geodes(state, bp);
            geodes_mul *= best_geodes;
        }
        geodes_mul.to_string()
    }
}

fn get_blueprints(input: &str) -> Vec<Blueprint> {
    let mut results = Vec::new();

    for (i, line) in input.trim().lines().enumerate() {
        let numbers: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse::<u64>())
            .filter(|n| n.is_ok())
            .map(|n| n.unwrap())
            .collect();
        let blueprint = Blueprint {
            blueprint_id: i as u64 + 1,
            ore_robot_cost_ore: numbers[0],
            clay_robot_cost_ore: numbers[1],
            obsidian_robot_cost_ore: numbers[2],
            obsidian_robot_cost_clay: numbers[3],
            geode_robot_cost_ore: numbers[4],
            geode_robot_cost_obsidian: numbers[5],
        };
        results.push(blueprint);
    }

    results
}

fn get_most_geodes(state: State, bp: &Blueprint) -> u64 {
    let mut memo = HashMap::new();
    dfs_memo(state, bp, &mut memo)
}

fn dfs_memo(state: State, bp: &Blueprint, memo: &mut HashMap<State, u64>) -> u64 {
    let state = cap_resources(state, bp);

    if let Some(&cached) = memo.get(&state) {
        return cached;
    }

    let mut most_geodes_count = state.geode_robots * state.minutes_left;

    // don't build new robots with unnecesssary costs (we can build 1 robot in 1 minute anyway)
    let max_ore_cost = bp
        .ore_robot_cost_ore
        .max(bp.clay_robot_cost_ore)
        .max(bp.obsidian_robot_cost_ore)
        .max(bp.geode_robot_cost_ore);
    let max_clay_cost = bp.obsidian_robot_cost_clay;
    let max_obsidian_cost = bp.geode_robot_cost_obsidian;
    for act in ACTIONS {
        if act == Action::BuildOreRobot && state.ore_robots >= max_ore_cost {
            continue;
        } else if act == Action::BuildClayRobot && state.clay_robots >= max_clay_cost {
            continue;
        } else if act == Action::BuildObsidianRobot && state.obsidian_robots >= max_obsidian_cost {
            continue;
        }

        if let Some(next) = step(state, act, bp) {
            most_geodes_count = most_geodes_count.max(
                state.geode_robots * (state.minutes_left - next.minutes_left)
                    + dfs_memo(next, bp, memo),
            );
        }
    }

    memo.insert(state, most_geodes_count);
    most_geodes_count
}

fn cap_resources(mut state: State, bp: &Blueprint) -> State {
    // cap resources to trigger cache hits
    // we don't care about resources which
    // exceed time left. So, limit it for
    // optimization.

    let max_ore_cost = bp
        .ore_robot_cost_ore
        .max(bp.clay_robot_cost_ore)
        .max(bp.obsidian_robot_cost_ore)
        .max(bp.geode_robot_cost_ore);
    let max_clay_cost = bp.obsidian_robot_cost_clay;
    let max_obsidian_cost = bp.geode_robot_cost_obsidian;

    let minutes = state.minutes_left;

    let max_ore = max_ore_cost * minutes - state.ore_robots * (minutes - 1);
    let max_clay = max_clay_cost * minutes - state.clay_robots * (minutes - 1);
    let max_obsidian = max_obsidian_cost * minutes - state.obsidian_robots * (minutes - 1);

    state.ore = state.ore.min(max_ore);
    state.clay = state.clay.min(max_clay);
    state.obsidian = state.obsidian.min(max_obsidian);

    state
}

fn step(mut state: State, action: Action, bp: &Blueprint) -> Option<State> {
    // save changes during selected action and apply them after time calculation
    // in case of impossible of production new robot - return None

    let ore_spent;
    let mut clay_spent = 0;
    let mut obsidian_spent = 0;

    let mut new_ore_robot = 0;
    let mut new_clay_robot = 0;
    let mut new_obsidial_robot = 0;
    let mut new_geode_robot = 0;

    let elapsed = match action {
        Action::BuildOreRobot => {
            let ore_wait = wait_time(state.ore, state.ore_robots, bp.ore_robot_cost_ore)?;
            ore_spent = bp.ore_robot_cost_ore;
            new_ore_robot = 1;
            ore_wait + 1
        }
        Action::BuildClayRobot => {
            let ore_wait = wait_time(state.ore, state.ore_robots, bp.clay_robot_cost_ore)?;
            ore_spent = bp.clay_robot_cost_ore;
            new_clay_robot = 1;
            ore_wait + 1
        }
        Action::BuildObsidianRobot => {
            let ore_wait = wait_time(state.ore, state.ore_robots, bp.obsidian_robot_cost_ore)?;
            let clay_wait = wait_time(state.clay, state.clay_robots, bp.obsidian_robot_cost_clay)?;
            ore_spent = bp.obsidian_robot_cost_ore;
            clay_spent = bp.obsidian_robot_cost_clay;
            new_obsidial_robot = 1;
            ore_wait.max(clay_wait) + 1
        }
        Action::BuildGeodeRobot => {
            let ore_wait = wait_time(state.ore, state.ore_robots, bp.geode_robot_cost_ore)?;
            let obsidian_wait = wait_time(
                state.obsidian,
                state.obsidian_robots,
                bp.geode_robot_cost_obsidian,
            )?;
            ore_spent = bp.geode_robot_cost_ore;
            obsidian_spent = bp.geode_robot_cost_obsidian;
            new_geode_robot = 1;
            ore_wait.max(obsidian_wait) + 1
        }
    };

    // return None if we can't produce new robot
    if elapsed >= state.minutes_left {
        return None;
    }

    let ore_after_wait = state.ore + state.ore_robots * elapsed;
    state.ore = ore_after_wait - ore_spent;
    let clay_after_wait = state.clay + state.clay_robots * elapsed;
    state.clay = clay_after_wait - clay_spent;
    let obsidian_after_wait = state.obsidian + state.obsidian_robots * elapsed;
    state.obsidian = obsidian_after_wait - obsidian_spent;

    state.minutes_left -= elapsed;

    state.ore_robots += new_ore_robot;
    state.clay_robots += new_clay_robot;
    state.obsidian_robots += new_obsidial_robot;
    state.geode_robots += new_geode_robot;

    Some(state)
}

fn wait_time(current: u64, production: u64, required: u64) -> Option<u64> {
    if current >= required {
        return Some(0);
    }

    if production == 0 {
        return None;
    }

    Some((required - current + production - 1) / production)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"Blueprint 1: Each ore robot costs 4 ore. Each clay robot costs 2 ore. Each obsidian robot costs 3 ore and 14 clay. Each geode robot costs 2 ore and 7 obsidian.
Blueprint 2: Each ore robot costs 2 ore. Each clay robot costs 3 ore. Each obsidian robot costs 3 ore and 8 clay. Each geode robot costs 3 ore and 12 obsidian.
"#;

    #[test]
    fn day19_part1_example1() {
        let actual = NineteenthSolution::part1(INPUT);
        assert_eq!(actual, "33");
    }

    #[test]
    fn day19_part2_example1() {
        let actual = NineteenthSolution::part2(INPUT);
        assert_eq!(actual, (62 * 56).to_string());
    }
}
