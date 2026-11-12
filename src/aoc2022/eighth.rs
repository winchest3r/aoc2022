use std::collections::HashMap;

use crate::{aoc2022::Solution, utils::Point};

pub struct EighthSolution;

const DIRECTIONS: &[Point] = &[
    Point::new(0, -1),
    Point::new(1, 0),
    Point::new(0, 1),
    Point::new(-1, 0),
];

impl Solution for EighthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let grid = get_grid(input);
        let mut visible = 0;

        for (&p, &height) in grid.iter() {
            'outer: for &dir in DIRECTIONS {
                // check points in direction
                let mut curr = p;
                loop {
                    curr = curr + dir;
                    if grid.contains_key(&curr) {
                        if grid[&curr] >= height {
                            break;
                        }
                    // if got outside of grid - visible
                    } else {
                        visible += 1;
                        break 'outer;
                    }
                }
            }
        }

        visible.to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let grid = get_grid(input);
        let mut best_scenic_score = 0;

        for (&p, &height) in grid.iter() {
            let mut scenic_score = 1;
            for &dir in DIRECTIONS {
                let mut curr = p;
                let mut dist = 0;
                loop {
                    curr = curr + dir;
                    if grid.contains_key(&curr) {
                        dist += 1;
                        if grid[&curr] >= height {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                scenic_score *= dist;
            }
            best_scenic_score = best_scenic_score.max(scenic_score);
        }

        best_scenic_score.to_string()
    }
}

fn get_grid(input: &str) -> HashMap<Point, u64> {
    let mut grid = HashMap::new();
    for (y, line) in input.lines().map(|l| l.trim()).enumerate() {
        for (x, c) in line.bytes().enumerate() {
            grid.insert(Point::new(x as i64, y as i64), (c - b'0') as u64);
        }
    }
    grid
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"30373
25512
65332
33549
35390
"#;

    #[test]
    fn day08_part1_example1() {
        let actual = EighthSolution::part1(INPUT);
        assert_eq!("21", actual);
    }

    #[test]
    fn day08_part2_example1() {
        let actual = EighthSolution::part2(INPUT);
        assert_eq!("8", actual);
    }
}
