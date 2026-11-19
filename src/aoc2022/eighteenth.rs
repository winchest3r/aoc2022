use std::collections::{HashSet, VecDeque};

use crate::{aoc2022::Solution, utils::Point3};

pub struct EighteenthSolution;

impl Solution for EighteenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let cubes = get_cubes(input);
        let mut free_sides = 0;
        for &cube in cubes.iter() {
            for dir in Point3::DIRECTIONS {
                let nb = cube + dir;
                if !cubes.contains(&nb) {
                    free_sides += 1;
                }
            }
        }
        free_sides.to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        // define min and max in every side
        // start from any corner and try to get to cubes with bfs
        // any side we get inside of our space - exterriors

        let cubes = get_cubes(input);

        let min_x = cubes.iter().map(|p| p.x).min().unwrap() - 1;
        let max_x = cubes.iter().map(|p| p.x).max().unwrap() + 1;
        let min_y = cubes.iter().map(|p| p.y).min().unwrap() - 1;
        let max_y = cubes.iter().map(|p| p.y).max().unwrap() + 1;
        let min_z = cubes.iter().map(|p| p.z).min().unwrap() - 1;
        let max_z = cubes.iter().map(|p| p.z).max().unwrap() + 1;

        let mut queue: VecDeque<Point3> = VecDeque::new();
        let mut visited: HashSet<Point3> = HashSet::new();
        let start = Point3::new(min_x, min_y, min_z);

        queue.push_back(start);
        visited.insert(start);

        let mut exterior_side = 0;
        while let Some(p) = queue.pop_front() {
            for dir in Point3::DIRECTIONS {
                let nb = p + dir;
                // if we can reach cube from this side -> exterior
                if cubes.contains(&nb) {
                    exterior_side += 1;
                // if in range and not visited -> add to queue
                } else if nb.x >= min_x
                    && nb.x <= max_x
                    && nb.y >= min_y
                    && nb.y <= max_y
                    && nb.z >= min_z
                    && nb.z <= max_z
                    && visited.insert(nb)
                {
                    queue.push_back(nb);
                }
            }
        }

        exterior_side.to_string()
    }
}

fn get_cubes(input: &str) -> HashSet<Point3> {
    let mut result = HashSet::new();

    for line in input.trim().lines().map(|l| l.trim()) {
        let cube: Vec<i64> = line.split(',').map(|v| v.parse::<i64>().unwrap()).collect();
        result.insert(Point3::new(cube[0], cube[1], cube[2]));
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"2,2,2
1,2,2
3,2,2
2,1,2
2,3,2
2,2,1
2,2,3
2,2,4
2,2,6
1,2,5
3,2,5
2,1,5
2,3,5
"#;

    #[test]
    fn day18_part1_example1() {
        let actual = EighteenthSolution::part1(INPUT);
        assert_eq!(actual, "64");
    }

    #[test]
    fn day18_part2_example1() {
        let actual = EighteenthSolution::part2(INPUT);
        assert_eq!(actual, "58");
    }
}
