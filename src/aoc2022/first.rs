use std::collections::BinaryHeap;

use crate::{aoc2022::Solution, utils::SPLIT_PATTERN};

pub struct FirstSolution;

impl Solution for FirstSolution {
    fn part1(input: &str) -> String {
        let calories = get_calories_data(input);
        calories
            .into_iter()
            .map(|elf| elf.iter().sum::<u64>())
            .max()
            .unwrap()
            .to_string()
    }

    fn part2(input: &str) -> String {
        let sum_calories: Vec<u64> = get_calories_data(input)
            .into_iter()
            .map(|elf| elf.iter().sum::<u64>())
            .collect();
        let mut max_heap = BinaryHeap::from(sum_calories);
        (0..3)
            .filter_map(|_| max_heap.pop())
            .sum::<u64>()
            .to_string()
    }
}

fn get_calories_data(input: &str) -> Vec<Vec<u64>> {
    input
        .trim()
        .split(SPLIT_PATTERN)
        .map(|elf| {
            elf.trim()
                .split_whitespace()
                .map(|cal| cal.parse::<u64>().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<_>>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &'static str = r#"1000
    2000
    3000

    4000

    5000
    6000

    7000
    8000
    9000

    10000
    "#;

    #[test]
    fn day01_part1_example1() {
        assert_eq!("24000", FirstSolution::part1(TEST_INPUT));
    }

    #[test]
    fn day01_part2_example() {
        assert_eq!("45000", FirstSolution::part2(TEST_INPUT));
    }
}
