use std::collections::HashMap;

use crate::aoc2022::Solution;

pub struct SixthSolution;

impl Solution for SixthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let bytes: Vec<u8> = input.bytes().collect();
        scan_windows(&bytes, 4).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let bytes: Vec<u8> = input.bytes().collect();
        scan_windows(&bytes, 14).to_string()
    }
}

fn scan_windows(bytes: &[u8], size: usize) -> usize {
    for (i, window) in bytes.windows(size).enumerate() {
        let mut count: HashMap<u8, usize> = HashMap::new();
        window.iter().for_each(|&b| {
            count.entry(b).and_modify(|b| *b += 1).or_default();
        });
        if count.len() == size {
            return i + size;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUTS: &[&'static str] = &[
        "mjqjpqmgbljsphdztnvjfqwrcgsmlb",
        "bvwbjplbgvbhsrlpgdmjqwftvncz",
        "nppdvjthqldpwncqszvftbrmjlhg",
        "nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg",
        "zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw",
    ];

    #[test]
    fn day06_part1_example1() {
        let expected = ["7", "5", "6", "10", "11"];
        for (&input, expect) in INPUTS.iter().zip(expected) {
            assert_eq!(expect, SixthSolution::part1(input));
        }
    }

    #[test]
    fn day06_part2_example1() {
        let expected = ["19", "23", "23", "29", "26"];
        for (&input, expect) in INPUTS.iter().zip(expected) {
            assert_eq!(expect, SixthSolution::part2(input));
        }
    }
}
