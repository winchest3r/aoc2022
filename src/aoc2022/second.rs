use crate::aoc2022::Solution;

pub struct SecondSolution;

impl Solution for SecondSolution {
    fn part1(input: &str) -> String {
        let mut score: u64 = 0;
        for line in input.trim().lines() {
            let (op, you) = line.trim().split_once(' ').unwrap();
            let you = match you {
                "X" => "A",
                "Y" => "B",
                "Z" => "C",
                _ => unreachable!(),
            };
            score += shape_score(you);
            score += match (op, you) {
                (op, you) if op == you => 3,
                ("A", "B") | ("B", "C") | ("C", "A") => 6,
                _ => 0,
            }
        }
        score.to_string()
    }

    fn part2(input: &str) -> String {
        let mut score: u64 = 0;
        for line in input.trim().lines() {
            let (op, you) = line.trim().split_once(' ').unwrap();
            let idx = (op.as_bytes()[0] - b'A') as usize;
            let (res_sc, you) = match you {
                "X" => (0, ["C", "A", "B"][idx]),
                "Y" => (3, op),
                "Z" => (6, ["B", "C", "A"][idx]),
                _ => unreachable!(),
            };
            score += shape_score(you) + res_sc;
        }
        score.to_string()
    }
}

fn shape_score(s: &str) -> u64 {
    (s.as_bytes()[0] - b'A') as u64 + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE_INPUT: &'static str = r#"
        A Y
        B X
        C Z
    "#;

    #[test]
    fn day02_part1_example1() {
        let res = SecondSolution::part1(EXAMPLE_INPUT);
        assert_eq!("15", res);
    }

    #[test]
    fn day02_part2_example1() {
        let res = SecondSolution::part2(EXAMPLE_INPUT);
        assert_eq!("12", res);
    }
}
