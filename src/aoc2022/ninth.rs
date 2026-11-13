use std::collections::HashSet;

use crate::{aoc2022::Solution, utils::Point};

pub struct NinthSolution;

impl Solution for NinthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut head: Point = Point::ZERO;
        let mut tail: Point = Point::ZERO;
        let mut tail_visited: HashSet<Point> = HashSet::new();
        tail_visited.insert(tail);

        for line in input.trim().lines().map(|l| l.trim()) {
            let (letter, amount) = line.split_once(' ').unwrap();
            let dir = letter_to_dir(letter);
            let amount = amount.parse::<u64>().unwrap();
            for _ in 0..amount {
                head += dir;
                let diff = head - tail;
                // in case of amount > 1 only
                if diff.x.abs() > 1 || diff.y.abs() > 1 {
                    tail += (diff.x.signum(), diff.y.signum()).into();
                    tail_visited.insert(tail);
                }
            }
        }
        tail_visited.len().to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut rope: [Point; 10] = [Point::ZERO; 10];
        let mut tail_visited = HashSet::new();
        tail_visited.insert(Point::ZERO);

        for line in input.trim().lines().map(|l| l.trim()) {
            let (letter, amount) = line.split_once(' ').unwrap();
            let dir = letter_to_dir(letter);
            let amount = amount.parse::<u64>().unwrap();
            for _ in 0..amount {
                // move head
                rope[0] += dir;
                // and update others
                for i in 0..(rope.len() - 1) {
                    let diff = rope[i] - rope[i + 1];
                    if diff.x.abs() > 1 || diff.y.abs() > 1 {
                        rope[i + 1] += (diff.x.signum(), diff.y.signum()).into();
                    } else {
                        break;
                    }
                }
                // add position of tail
                tail_visited.insert(rope[rope.len() - 1]);
            }
        }

        tail_visited.len().to_string()
    }
}

fn letter_to_dir(s: &str) -> Point {
    match s {
        "U" => Point::UP,
        "R" => Point::RIGHT,
        "D" => Point::DOWN,
        "L" => Point::LEFT,
        _ => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2
"#;

    const INPUT2: &'static str = r#"R 5
U 8
L 8
D 3
R 17
D 10
L 25
U 20
"#;

    #[test]
    fn day09_part1_example1() {
        let actual = NinthSolution::part1(INPUT);
        assert_eq!("13", actual);
    }

    #[test]
    fn day09_part2_example1() {
        let actual = NinthSolution::part2(INPUT);
        assert_eq!("1", actual);
    }

    #[test]
    fn day09_part2_example2() {
        let actual = NinthSolution::part2(INPUT2);
        assert_eq!("36", actual);
    }
}
