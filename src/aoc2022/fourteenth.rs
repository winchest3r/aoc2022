use std::collections::HashMap;

use crate::{aoc2022::Solution, utils::Point};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Cell {
    Cave,
    Rock,
}

pub struct FourteenthSolution;

impl Solution for FourteenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut map = get_map(input);

        let highest_point = map.keys().map(|p| p.y).max().unwrap();

        while process_new_rock(&mut map, highest_point, false) {}

        map.values()
            .filter(|&v| *v == Cell::Rock)
            .count()
            .to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut map = get_map(input);

        let highest_point = map.keys().map(|p| p.y).max().unwrap() + 2;
        while process_new_rock(&mut map, highest_point, true) {}

        map.values()
            .filter(|&v| *v == Cell::Rock)
            .count()
            .to_string()
    }
}

/// Returns false in case of falling into open space or blocking spawn point.
fn process_new_rock(map: &mut HashMap<Point, Cell>, highest_point: i64, has_floor: bool) -> bool {
    let spawn_point = Point::new(500, 0);
    let mut rock_point = spawn_point;

    loop {
        // in case of we get till cave part or other rock
        rock_point += Point::DOWN;
        // if has floor - save point which get till it
        if has_floor && rock_point.y == highest_point {
            rock_point += Point::UP;
            break;
        // if rock point more than highest_point -> leave
        } else if !has_floor && rock_point.y > highest_point {
            return false;
        }
        if map.contains_key(&rock_point) {
            if !map.contains_key(&(rock_point + Point::LEFT)) {
                rock_point += Point::LEFT;
            } else if !map.contains_key(&(rock_point + Point::RIGHT)) {
                rock_point += Point::RIGHT;
            } else {
                rock_point += Point::UP;
                break;
            }
        }
    }

    map.insert(rock_point, Cell::Rock);
    // end when we fill spawn point
    if rock_point == spawn_point {
        return false;
    }

    true
}

fn get_map(input: &str) -> HashMap<Point, Cell> {
    let mut map = HashMap::new();

    for line in input.trim().lines().map(|l| l.trim()) {
        let coords: Vec<Point> = line
            .split(" -> ")
            .map(|s| {
                let (x, y) = s.split_once(',').unwrap();
                Point::new(x.parse().unwrap(), y.parse().unwrap())
            })
            .collect();
        let mut p = coords[0];
        for &next in &coords[1..] {
            while p != next {
                map.insert(p, Cell::Cave);
                let diff = next - p;
                if diff.x == 0 {
                    p.y += diff.y.signum();
                } else {
                    p.x += diff.x.signum();
                }
            }
            map.insert(next, Cell::Cave);
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"498,4 -> 498,6 -> 496,6
503,4 -> 502,4 -> 502,9 -> 494,9
"#;

    #[test]
    fn day14_part1_example1() {
        let actual = FourteenthSolution::part1(INPUT);
        assert_eq!("24", actual);
    }

    #[test]
    fn day14_part2_example1() {
        let actual = FourteenthSolution::part2(INPUT);
        assert_eq!("93", actual);
    }
}
