use std::collections::HashSet;

use crate::aoc2022::Solution;

pub struct ThirdSolution;

impl Solution for ThirdSolution {
    fn part1(input: &str) -> String {
        let mut sum = 0;

        for line in input.trim().lines() {
            let line_len = line.trim().len();
            let (left, right) = line.trim().split_at(line_len / 2);

            let left_set: HashSet<u8> = HashSet::from_iter(left.bytes());
            let right_set: HashSet<u8> = HashSet::from_iter(right.bytes());
            for &b in left_set.intersection(&right_set) {
                sum += calc_value(b);
                break;
            }
        }

        sum.to_string()
    }

    fn part2(input: &str) -> String {
        let mut sum = 0;
        let lines = input.trim().lines().collect::<Vec<&str>>();

        const CHUNK_SIZE: usize = 3;
        for chunk in lines.chunks(CHUNK_SIZE) {
            let first: HashSet<u8> = HashSet::from_iter(chunk[0].trim().bytes());
            let second: HashSet<u8> = HashSet::from_iter(chunk[1].trim().bytes());
            let third: HashSet<u8> = HashSet::from_iter(chunk[2].trim().bytes());

            for b in first.intersection(&second) {
                if third.contains(b) {
                    sum += calc_value(*b);
                    break;
                }
            }
        }

        sum.to_string()
    }
}

fn calc_value(b: u8) -> u64 {
    if (b'a'..=b'z').contains(&b) {
        (b - b'a' + 1) as u64
    } else {
        (b - b'A' + 27) as u64
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"vJrwpWtwJgWrhcsFMMfFFhFp
        jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
        PmmdzqPrVvPwwTWBwg
        wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
        ttgJtRGJQctTZtZT
        CrZsJsPPZsGzwwsLwLmpwMDw
    "#;

    #[test]
    fn day03_part1_example1() {
        let actual = ThirdSolution::part1(INPUT);
        assert_eq!("157", actual);
    }

    #[test]
    fn day03_part2_example1() {
        let actual = ThirdSolution::part2(INPUT);
        assert_eq!("70", actual);
    }
}
