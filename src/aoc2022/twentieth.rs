use crate::aoc2022::Solution;

pub struct TwentiethSolution;

impl Solution for TwentiethSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let numbers = get_numbers(input);
        let mixed = mixing(&numbers, 1);
        grove_coordinates(&mixed).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut numbers = get_numbers(input);
        const DECRYPTION_KEY: i64 = 811589153;
        numbers.iter_mut().for_each(|n| *n *= DECRYPTION_KEY);
        let mixed = mixing(&numbers, 10);
        grove_coordinates(&mixed).to_string()
    }
}

fn get_numbers(input: &str) -> Vec<i64> {
    input
        .trim()
        .lines()
        .map(|l| l.trim().parse::<i64>().unwrap())
        .collect()
}

fn mixing(numbers: &[i64], amount: usize) -> Vec<i64> {
    // save original idx
    // parse using original idx
    // kinda slower with vectors but easier than making some circular lists

    let mut data: Vec<(usize, i64)> = numbers.iter().copied().enumerate().collect();

    for _ in 0..amount {
        for idx in 0..data.len() {
            let orig_idx = data.iter().position(|(j, _)| *j == idx).unwrap();
            let elem = data.remove(orig_idx);

            // we can't use % because default reminder operator works wrong in case of negative operands.
            // we need it as euclidean modulo
            let new_idx = (orig_idx as i64 + elem.1).rem_euclid((data.len()) as i64) as usize;

            data.insert(new_idx, elem);
        }
    }
    data.iter().map(|(_, n)| *n).collect()
}

fn grove_coordinates(numbers: &[i64]) -> i64 {
    let zero_idx = numbers.iter().position(|n| *n == 0).unwrap();
    let len = numbers.len();
    numbers[(zero_idx + 1000) % len]
        + numbers[(zero_idx + 2000) % len]
        + numbers[(zero_idx + 3000) % len]
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"1
2
-3
3
-2
0
4
"#;

    #[test]
    fn day20_part1_example1() {
        let actual = TwentiethSolution::part1(INPUT);
        assert_eq!(actual, "3");
    }

    #[test]
    fn day20_part2_example1() {
        let actual = TwentiethSolution::part2(INPUT);
        assert_eq!(actual, "1623178306");
    }
}
