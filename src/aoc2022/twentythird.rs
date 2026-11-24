use std::collections::{HashMap, HashSet};

use crate::{aoc2022::Solution, utils::Point};

/// Possible directions in initial order
const MOVE_DIRECTIONS: [[Point; 3]; 4] = [
    [Point::new(-1, -1), Point::new(0, -1), Point::new(1, -1)],
    [Point::new(-1, 1), Point::new(0, 1), Point::new(1, 1)],
    [Point::new(-1, -1), Point::new(-1, 0), Point::new(-1, 1)],
    [Point::new(1, -1), Point::new(1, 0), Point::new(1, 1)],
];

const ALL_DIRECTIONS: [Point; 8] = [
    Point::new(-1, -1),
    Point::new(0, -1),
    Point::new(1, -1),
    Point::new(-1, 0),
    Point::new(1, 0),
    Point::new(-1, 1),
    Point::new(0, 1),
    Point::new(1, 1),
];

fn get_round_directions(round: usize) -> [[Point; 3]; 4] {
    [
        MOVE_DIRECTIONS[(0 + round) % MOVE_DIRECTIONS.len()],
        MOVE_DIRECTIONS[(1 + round) % MOVE_DIRECTIONS.len()],
        MOVE_DIRECTIONS[(2 + round) % MOVE_DIRECTIONS.len()],
        MOVE_DIRECTIONS[(3 + round) % MOVE_DIRECTIONS.len()],
    ]
}

pub struct TwentyThirdSolution;

impl Solution for TwentyThirdSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut grid = parse_grid(input);
        let mut round = 0;
        while round < 10 {
            let directions = get_round_directions(round);
            step(&mut grid, &directions);
            round += 1;
        }
        empty_tiles(&grid).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut grid = parse_grid(input);
        let mut round = 0;
        while step(&mut grid, &get_round_directions(round)) {
            round += 1;
        }
        (round + 1).to_string()
    }
}

fn parse_grid(input: &str) -> HashSet<Point> {
    let mut result: HashSet<Point> = HashSet::new();

    for (y, line) in input.trim().lines().map(|l| l.trim()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            if c == '#' {
                result.insert((x as i64, y as i64).into());
            }
        }
    }

    result
}

fn step(grid: &mut HashSet<Point>, directions: &[[Point; 3]]) -> bool {
    let mut proposals: HashMap<Point, usize> = HashMap::new();
    let mut moves: Vec<(Point, Point)> = Vec::new();

    // form proposals
    for &elf_p in grid.iter() {
        // if no other elves around - continue
        if ALL_DIRECTIONS.iter().all(|&d| !grid.contains(&(elf_p + d))) {
            continue;
        }

        // if there is any - fill proposal
        for side in directions {
            if side.iter().all(|&d| !grid.contains(&(elf_p + d))) {
                let prop_p = elf_p + side[1];
                proposals.entry(prop_p).and_modify(|v| *v += 1).or_insert(1);
                moves.push((elf_p, prop_p));
                break;
            }
        }
    }

    let success_moves: Vec<(Point, Point)> = moves
        .into_iter()
        .filter(|(_, prop_p)| proposals[prop_p] == 1)
        .collect();

    // can leave without moves
    if success_moves.is_empty() {
        return false;
    }

    // move elves without collision
    for (elf_p, move_p) in success_moves {
        if proposals[&move_p] != 1 {
            continue;
        }
        grid.remove(&elf_p);
        grid.insert(move_p);
    }

    true
}

fn empty_tiles(grid: &HashSet<Point>) -> usize {
    let min_x = grid.iter().map(|p| p.x).min().unwrap();
    let min_y = grid.iter().map(|p| p.y).min().unwrap();
    let max_x = grid.iter().map(|p| p.x).max().unwrap();
    let max_y = grid.iter().map(|p| p.y).max().unwrap();
    let width = max_x - min_x + 1;
    let height = max_y - min_y + 1;
    (width * height) as usize - grid.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"....#..
..###.#
#...#.#
.#...##
#.###..
##.#.##
.#..#..
";

    #[test]
    fn day23_part1_example1() {
        let actual = TwentyThirdSolution::part1(INPUT);
        assert_eq!(actual, "110");
    }

    #[test]
    fn day23_part2_example1() {
        let actual = TwentyThirdSolution::part2(INPUT);
        assert_eq!(actual, "20");
    }
}
