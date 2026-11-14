use std::collections::VecDeque;

use crate::{aoc2022::Solution, utils::SPLIT_PATTERN};

#[derive(Debug)]
struct Monkey {
    pub items: VecDeque<u128>,
    pub op: char,
    pub operand: String,
    pub test: u128,
    pub true_throw: usize,
    pub false_throw: usize,
    pub total_inspections: u64,
}

pub struct EleventhSolution;

impl Solution for EleventhSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut monkeys = parse_monkeys(input);

        for _ in 0..20 {
            round(&mut monkeys, true);
        }

        monkeys.sort_by(|a, b| b.total_inspections.cmp(&a.total_inspections));

        (monkeys[0].total_inspections * monkeys[1].total_inspections).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut monkeys = parse_monkeys(input);

        for _ in 0..10000 {
            round(&mut monkeys, false);
        }

        monkeys.sort_by(|a, b| b.total_inspections.cmp(&a.total_inspections));

        (monkeys[0].total_inspections * monkeys[1].total_inspections).to_string()
    }
}

fn round(monkeys: &mut Vec<Monkey>, part1: bool) {
    let monkeys_len = monkeys.len();
    let modulo = monkeys.iter().map(|m| m.test).product::<u128>();
    for i in 0..monkeys_len {
        while let Some(mut item) = monkeys[i].items.pop_front() {
            monkeys[i].total_inspections += 1;
            // get right new worry level
            let right_operand = if monkeys[i].operand == "old" {
                item
            } else {
                monkeys[i].operand.parse::<u128>().unwrap()
            };
            item = match monkeys[i].op {
                '+' => item + right_operand,
                '*' => item * right_operand,
                _ => unreachable!(),
            };
            // divide by 3 before testing in case of part1
            if part1 {
                item /= 3;
            // or use modulo in case of part2 (items overflows with u128 even otherwise)
            } else {
                // modulo = d1 * ... * dn, item % modulo is equal to item item % di
                item %= modulo;
            }
            let throw_idx = if item % monkeys[i].test == 0 {
                monkeys[i].true_throw
            } else {
                monkeys[i].false_throw
            };
            monkeys[throw_idx].items.push_back(item);
        }
    }
}

fn parse_monkeys(input: &str) -> Vec<Monkey> {
    let regex = regex::regex!(
        r#"\s*Monkey \d+:\s+Starting items: (?P<items>.*)\s+Operation: new = old (?P<op>.*)\s+Test: divisible by (?P<test>.*)\s+If true: throw to monkey (?P<true_throw>\d+)\s+If false: throw to monkey (?<false_throw>\d+)\s*"#
    );

    let mut monkeys = Vec::new();
    for monkey_str in input.split(SPLIT_PATTERN) {
        let cap = regex.captures(monkey_str).unwrap();
        let (op, operand) = cap["op"].split_once(' ').unwrap();
        let monkey = Monkey {
            items: cap["items"]
                .split(", ")
                .map(|s| s.parse::<u128>().unwrap())
                .collect(),
            op: op.as_bytes()[0] as char,
            operand: operand.to_string(),
            test: cap["test"].parse::<u128>().unwrap(),
            true_throw: cap["true_throw"].parse::<usize>().unwrap(),
            false_throw: cap["false_throw"].parse::<usize>().unwrap(),
            total_inspections: 0,
        };
        monkeys.push(monkey);
    }

    monkeys
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"Monkey 0:
      Starting items: 79, 98
      Operation: new = old * 19
      Test: divisible by 23
        If true: throw to monkey 2
        If false: throw to monkey 3

    Monkey 1:
      Starting items: 54, 65, 75, 74
      Operation: new = old + 6
      Test: divisible by 19
        If true: throw to monkey 2
        If false: throw to monkey 0

    Monkey 2:
      Starting items: 79, 60, 97
      Operation: new = old * old
      Test: divisible by 13
        If true: throw to monkey 1
        If false: throw to monkey 3

    Monkey 3:
      Starting items: 74
      Operation: new = old + 3
      Test: divisible by 17
        If true: throw to monkey 0
        If false: throw to monkey 1
"#;

    #[test]
    fn day11_part1_example1() {
        let actual = EleventhSolution::part1(INPUT);
        assert_eq!("10605", actual);
    }

    #[test]
    fn day11_part2_example1() {
        let actual = EleventhSolution::part2(INPUT);
        assert_eq!("2713310158", actual);
    }
}
