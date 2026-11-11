use crate::aoc2022::Solution;

pub struct FourthSolution;

impl Solution for FourthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut overlap = 0;

        for line in input.trim().lines() {
            let ((a0, a1), (b0, b1)) = line_to_ranges(line);
            if (a0 <= b0 && b1 <= a1) || (b0 <= a0 && a1 <= b1) {
                overlap += 1;
            }
        }

        overlap.to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut overlap = 0;

        for line in input.trim().lines() {
            let ((a0, a1), (b0, b1)) = line_to_ranges(line);
            if a0.max(b0) <= a1.min(b1) {
                overlap += 1;
            }
        }

        overlap.to_string()
    }
}

fn line_to_ranges(line: &str) -> ((u64, u64), (u64, u64)) {
    let (r1, r2) = line.trim().split_once(',').unwrap();
    let (a0, a1): (u64, u64) = r1
        .split_once('-')
        .map(|(a0, a1)| (a0.parse().unwrap(), a1.parse().unwrap()))
        .unwrap();
    let (b0, b1): (u64, u64) = r2
        .split_once('-')
        .map(|(b0, b1)| (b0.parse().unwrap(), b1.parse().unwrap()))
        .unwrap();
    ((a0, a1), (b0, b1))
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"2-4,6-8
        2-3,4-5
        5-7,7-9
        2-8,3-7
        6-6,4-6
        2-6,4-8
    "#;

    #[test]
    fn day04_part1_example1() {
        let actual = FourthSolution::part1(INPUT);
        assert_eq!("2", actual);
    }

    #[test]
    fn day04_part2_example1() {
        let actual = FourthSolution::part2(INPUT);
        assert_eq!("4", actual);
    }
}
