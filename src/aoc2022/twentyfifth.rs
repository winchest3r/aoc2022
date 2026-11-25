use crate::aoc2022::Solution;

pub struct TwentyFifthSolution;

impl Solution for TwentyFifthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut total = 0;
        for line in input.trim().lines() {
            total += snafu_to_decimal(line);
        }
        decimal_to_snafu(total).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        "done!".to_string()
    }
}

fn decimal_to_snafu(mut n: u64) -> String {
    let mut result = String::new();
    const SNAFU: [char; 5] = ['0', '1', '2', '=', '-'];

    loop {
        let (mut d, m) = (n / 5, n % 5);
        if m >= 3 {
            d += 1;
        }
        result.push(SNAFU[(m % 5) as usize]);
        n = d;
        if n == 0 {
            return result.chars().rev().collect();
        }
    }
}

fn snafu_to_decimal(s: &str) -> u64 {
    let mut n: i64 = 0;
    for (p, c) in s.chars().rev().enumerate() {
        let c_val: i64 = match c {
            '0' => 0,
            '1' => 1,
            '2' => 2,
            '=' => -2,
            '-' => -1,
            _ => unreachable!(),
        };
        n += c_val * 5i64.pow(p as u32);
    }
    n as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r"1=-0-2
12111
2=0=
21
2=01
111
20012
112
1=-1=
1-12
12
1=
122
";

    const DEC_TO_SNAFU_CASES: &[(u64, &'static str)] = &[
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0"),
    ];

    #[test]
    fn day25_part1_example1() {
        for &(n, expect) in DEC_TO_SNAFU_CASES {
            assert_eq!(decimal_to_snafu(n), expect);
        }
    }

    #[test]
    fn day25_part1_example2() {
        let actual = TwentyFifthSolution::part1(INPUT);
        assert_eq!(actual, "2=-1=0");
    }
}
