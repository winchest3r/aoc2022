use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    aoc2022::Solution,
    utils::{self, Point},
};

pub struct TwentyFourthSolution;

impl Solution for TwentyFourthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        // we need save timeline states - blizzards configurations on map during time.
        // and we can use that the don't collide to each other - basically have period
        // it equals to lcm(w, h)
        //
        // also we can precompute all as Vec<HashSet<Point>> of blocked cells
        //
        // then we do bfs using timeline map configurations

        let (grid, size) = parse_blizzards(input);
        let blizzards = compute_timeline(&grid, size);
        let start = Point::new(1, 0);
        let end = Point::new(size.x - 1, size.y);
        bfs(&blizzards, size, start, end, 0).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        // the same but need to walk through three times
        // start walking from previous time
        // return with last one

        let (grid, size) = parse_blizzards(input);
        let blizzards = compute_timeline(&grid, size);
        let start = Point::new(1, 0);
        let end = Point::new(size.x - 1, size.y);

        let t1 = bfs(&blizzards, size, start, end, 0);
        let t2 = bfs(&blizzards, size, end, start, t1);
        let t3 = bfs(&blizzards, size, start, end, t2);

        t3.to_string()
    }
}

fn bfs(blocked: &[HashSet<Point>], size: Point, start: Point, end: Point, start_time: i64) -> i64 {
    // must have -1 because we have corners inside size
    let period = utils::lcm(size.x - 1, size.y - 1);
    let mut queue: VecDeque<(Point, i64)> = VecDeque::new();
    let mut visited: HashSet<(Point, i64)> = HashSet::new();

    // queue contains final time, visited just state related to period
    queue.push_back((start, start_time));
    visited.insert((start, start_time % period));

    while let Some((p, time)) = queue.pop_front() {
        let next_time = time + 1;

        let grid = &blocked[(next_time % period) as usize];
        for d in [
            Point::UP,
            Point::DOWN,
            Point::LEFT,
            Point::RIGHT,
            Point::ZERO,
        ] {
            let new_p = p + d;
            // filter walls except start and end
            if new_p != start && new_p != end {
                if new_p.x <= 0 || new_p.y <= 0 || new_p.x >= size.x || new_p.y >= size.y {
                    continue;
                }
            }
            if grid.contains(&new_p) || visited.contains(&(new_p, next_time % period)) {
                continue;
            }
            if new_p == end {
                return next_time;
            }
            visited.insert((new_p, next_time % period));
            queue.push_back((new_p, next_time));
        }
    }

    unreachable!()
}

/// Returns vector of blocked points related to current minute
fn compute_timeline(grid: &HashMap<Point, char>, size: Point) -> Vec<HashSet<Point>> {
    // must have -1 because we have corners inside size
    let period = utils::lcm(size.x - 1, size.y - 1);
    let mut results = Vec::with_capacity(period as usize);

    // init blizzard positions with possibility of collision
    let mut current: HashMap<Point, Vec<char>> = HashMap::new();
    for (&p, &c) in grid {
        current.entry(p).or_default().push(c);
    }

    results.push(current.keys().copied().collect());

    // simulate minutes
    for _ in 1..period {
        // create new and fill it with new positions
        let mut next: HashMap<Point, Vec<char>> = HashMap::new();
        for (&p, blizzards) in &current {
            for &c in blizzards {
                let dir = match c {
                    '>' => Point::RIGHT,
                    'v' => Point::DOWN,
                    '<' => Point::LEFT,
                    '^' => Point::UP,
                    _ => unreachable!(),
                };

                let mut new_p = p + dir;

                // wrap
                if new_p.x >= size.x {
                    new_p.x = 1;
                } else if new_p.x <= 0 {
                    new_p.x = size.x - 1;
                }
                if new_p.y >= size.y {
                    new_p.y = 1;
                } else if new_p.y <= 0 {
                    new_p.y = size.y - 1;
                }

                next.entry(new_p).or_default().push(c);
            }
        }

        results.push(next.keys().copied().collect());
        current = next;
    }

    results
}

/// Returns hashmap of blizzard cells and grid size
fn parse_blizzards(input: &str) -> (HashMap<Point, char>, Point) {
    let mut grid = HashMap::new();
    let mut max_x = 0;
    let mut max_y = 0;

    for (y, line) in input.trim().lines().map(|l| l.trim()).enumerate() {
        for (x, c) in line.chars().enumerate() {
            max_x = max_x.max(x as i64);
            if ['<', '>', 'v', '^'].contains(&c) {
                grid.insert((x as i64, y as i64).into(), c);
            }
        }
        max_y = max_y.max(y as i64);
    }

    (grid, (max_x, max_y).into())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"#.######
#>>.<^<#
#.<..<<#
#>v.><>#
#<^v^^>#
######.#
";

    #[test]
    fn day24_part1_example1() {
        let actual = TwentyFourthSolution::part1(INPUT);
        assert_eq!(actual, "18");
    }

    #[test]
    fn day24_part2_example1() {
        let actual = TwentyFourthSolution::part2(INPUT);
        assert_eq!(actual, "54");
    }
}
