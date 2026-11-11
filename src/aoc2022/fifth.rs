use crate::{aoc2022::Solution, utils::SPLIT_PATTERN};

pub struct FifthSolution;

impl Solution for FifthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut crates = get_crates(input);
        let moves = get_moves(input);

        for (amount, from_idx, to_idx) in moves {
            for _ in 0..amount {
                let top = crates[from_idx].pop().unwrap();
                crates[to_idx].push(top);
            }
        }

        top_crates(&crates)
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut crates = get_crates(input);
        let moves = get_moves(input);

        for (amount, from_idx, to_idx) in moves {
            let mut temp = Vec::new();
            for _ in 0..amount {
                let top = crates[from_idx].pop().unwrap();
                temp.push(top)
            }
            crates[to_idx].extend(temp.iter().rev());
        }

        top_crates(&crates)
    }
}

fn get_crates(input: &str) -> Vec<Vec<u8>> {
    let (first, _) = input.split_once(SPLIT_PATTERN).unwrap();
    let crates_data = first
        .lines()
        .map(|s| s.bytes().collect::<Vec<u8>>())
        .collect::<Vec<_>>();

    let mut result = Vec::new();
    for line in crates_data.iter().rev().skip(1) {
        for (i, crt) in line.chunks(4).enumerate() {
            if result.len() <= i {
                result.push(Vec::new())
            }
            let crt = crt[1];
            if crt == b' ' {
                continue;
            }
            result[i].push(crt);
        }
    }
    result
}

fn top_crates(crates: &[Vec<u8>]) -> String {
    crates.iter().map(|v| *v.last().unwrap() as char).collect()
}

//                                amount from   to
fn get_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let (_, second) = input.split_once(SPLIT_PATTERN).unwrap();
    let mut result = Vec::new();
    for line in second.trim().lines() {
        let words = line.trim().split_whitespace().collect::<Vec<_>>();
        let amount = words[1].parse::<usize>().unwrap();
        let from_idx = words[3].parse::<usize>().unwrap() - 1;
        let to_idx = words[5].parse::<usize>().unwrap() - 1;
        result.push((amount, from_idx, to_idx));
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"    [D]
[N] [C]
[Z] [M] [P]
 1   2   3

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2

"#;

    #[test]
    fn day05_part1_example1() {
        let expected = FifthSolution::part1(INPUT);
        assert_eq!("CMZ", expected);
    }

    #[test]
    fn day05_part2_example1() {
        let expected = FifthSolution::part2(INPUT);
        assert_eq!("MCD", expected);
    }
}
