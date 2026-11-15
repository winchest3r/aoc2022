use std::collections::{HashMap, HashSet, VecDeque};

use crate::{aoc2022::Solution, utils::Point};

pub struct TwelfthSolution;

impl Solution for TwelfthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let (grid, start, end) = get_grid(input);

        let mut queue: VecDeque<(Point, i32)> = VecDeque::new();
        queue.push_back((start, 0));

        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(start);

        while let Some((point, steps)) = queue.pop_front() {
            if point == end {
                return steps.to_string();
            }
            for dir in [Point::UP, Point::RIGHT, Point::DOWN, Point::LEFT] {
                let new_point = point + dir;
                if !grid.contains_key(&new_point) || visited.contains(&new_point) {
                    continue;
                }
                let height_diff = grid[&new_point] - grid[&point];
                if height_diff <= 1 {
                    visited.insert(new_point);
                    queue.push_back((new_point, steps + 1));
                }
            }
        }

        unreachable!()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        // we can start from E and get to closest a with BFS.
        let (grid, _, end) = get_grid(input);

        let mut queue: VecDeque<(Point, i32)> = VecDeque::new();
        queue.push_back((end, 0));

        let mut visited: HashSet<Point> = HashSet::new();
        visited.insert(end);

        while let Some((point, steps)) = queue.pop_front() {
            if grid[&point] == 0 {
                return steps.to_string();
            }
            for dir in [Point::UP, Point::RIGHT, Point::DOWN, Point::LEFT] {
                let new_point = point + dir;
                if !grid.contains_key(&new_point) || visited.contains(&new_point) {
                    continue;
                }
                let height_diff = grid[&point] - grid[&new_point];
                if height_diff <= 1 {
                    visited.insert(new_point);
                    queue.push_back((new_point, steps + 1));
                }
            }
        }

        unreachable!()
    }
}

fn get_grid(input: &str) -> (HashMap<Point, i32>, Point, Point) {
    let mut map: HashMap<Point, i32> = HashMap::new();
    let mut start = Point::ZERO;
    let mut end = Point::ZERO;

    for (y, line) in input.trim().lines().map(|l| l.trim()).enumerate() {
        for (x, &c) in line.as_bytes().iter().enumerate() {
            let height: i32 = if c == b'S' {
                0
            } else if c == b'E' {
                (b'z' - b'a') as i32
            } else {
                (c - b'a') as i32
            };
            let point = Point::new(x as i64, y as i64);
            if c == b'S' {
                start = point;
            } else if c == b'E' {
                end = point;
            }
            map.insert(point, height);
        }
    }

    (map, start, end)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"Sabqponm
abcryxxl
accszExk
acctuvwj
abdefghi
"#;

    #[test]
    fn day12_part1_example1() {
        let actual = TwelfthSolution::part1(INPUT);
        assert_eq!("31", actual);
    }

    #[test]
    fn day12_part2_example1() {
        let actual = TwelfthSolution::part2(INPUT);
        assert_eq!("29", actual);
    }
}
