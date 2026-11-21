use std::collections::{HashMap, VecDeque};

use crate::aoc2022::Solution;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Job {
    Value(i64),
    Operation { lhs: usize, op: char, rhs: usize },
}

impl Job {
    fn is_operation(self) -> bool {
        match self {
            Job::Operation { .. } => true,
            Job::Value(_) => false,
        }
    }

    fn get_value(self) -> i64 {
        match self {
            Job::Value(val) => val,
            Job::Operation { .. } => panic!("can't get value from Job::Operation"),
        }
    }
}

pub struct TwentyFirstSolution;

impl Solution for TwentyFirstSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let (mut jobs, names) = parse(input);
        process_jobs(&mut jobs);
        jobs[names["root"]].get_value().to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        // we can't bruteforce it.
        // solution - walk through chain from root to humn
        // humn related to only one branch of root operation
        // because we walk from root -> inverse operations
        // inversed operations related from known or unknown operands
        // something like that

        let (jobs, names) = parse(input);
        solve_for_humn(&jobs, names["humn"], names["root"]).to_string()
    }
}

fn process_jobs(jobs: &mut [Job]) {
    // get all operations
    let mut queue: VecDeque<(usize, Job)> = jobs
        .iter()
        .enumerate()
        .filter(|(_, j)| j.is_operation())
        .map(|(i, &j)| (i, j))
        .collect();

    while let Some((idx, job_op)) = queue.pop_front() {
        match job_op {
            Job::Operation { lhs, op, rhs } => {
                // in case of we can't calculate value at this point
                // add this operation back in queue
                if jobs[lhs].is_operation() || jobs[rhs].is_operation() {
                    queue.push_back((idx, job_op));
                    continue;
                }
                // or calculate and save it in initial jobs
                let lhs = jobs[lhs].get_value();
                let rhs = jobs[rhs].get_value();
                let res = match op {
                    '+' => lhs + rhs,
                    '-' => lhs - rhs,
                    '*' => lhs * rhs,
                    '/' => lhs / rhs,
                    _ => unreachable!(),
                };
                jobs[idx] = Job::Value(res);
            }
            Job::Value(_) => unreachable!(),
        }
    }
}

fn solve_for_humn(jobs: &[Job], humn_idx: usize, root_idx: usize) -> i64 {
    // determine which root branch depends on humn
    let (lhs, rhs) = match jobs[root_idx] {
        Job::Operation { lhs, rhs, .. } => (lhs, rhs),
        Job::Value(_) => unreachable!(),
    };

    // only one branch expected to be related to humn
    let lhs_dep = depends_on(lhs, jobs, humn_idx);

    let mut evaluated = jobs.to_vec();
    process_jobs(&mut evaluated);

    let (mut current, mut target) = if lhs_dep {
        (lhs, evaluated[rhs].get_value())
    } else {
        (rhs, evaluated[lhs].get_value())
    };

    // walk toawrd humn with reversing all operations
    while current != humn_idx {
        match jobs[current] {
            Job::Operation { lhs, op, rhs } => {
                let lhs_dep = depends_on(lhs, jobs, humn_idx);

                if lhs_dep {
                    let known = evaluated[rhs].get_value();
                    target = reverse_op(op, target, known, true);
                    current = lhs;
                } else {
                    let known = evaluated[lhs].get_value();
                    target = reverse_op(op, target, known, false);
                    current = rhs;
                }
            }
            Job::Value(_) => unreachable!(),
        }
    }

    target
}

fn reverse_op(op: char, target: i64, known: i64, unknown_left: bool) -> i64 {
    // because we move toward humn from root we need to inverse operations.
    // rhs operand needs different formulas for sub and div.

    match (op, unknown_left) {
        ('+', _) => target - known,
        ('-', true) => target + known,
        ('-', false) => known - target,
        ('*', _) => target / known,
        ('/', true) => target * known,
        ('/', false) => known / target,
        _ => unreachable!(),
    }
}

fn depends_on(idx: usize, jobs: &[Job], humn_idx: usize) -> bool {
    // we check index chain and if it's related to target humn - return true

    if idx == humn_idx {
        return true;
    }

    match jobs[idx] {
        Job::Value(_) => false,
        Job::Operation { lhs, rhs, .. } => {
            depends_on(lhs, jobs, humn_idx) || depends_on(rhs, jobs, humn_idx)
        }
    }
}

fn parse(input: &str) -> (Vec<Job>, HashMap<String, usize>) {
    let mut names = HashMap::new();
    for (i, line) in input.trim().lines().map(|l| l.trim()).enumerate() {
        let (name, _) = line.split_once(' ').unwrap();
        names.insert(name[0..(name.len() - 1)].to_string(), i);
    }

    let mut results = Vec::new();
    for line in input.trim().lines().map(|l| l.trim()) {
        let (_, job_str) = line.split_once(' ').unwrap();
        if job_str.chars().all(|c| c.is_ascii_digit()) {
            results.push(Job::Value(job_str.parse::<i64>().unwrap()));
        } else {
            let parts: Vec<&str> = job_str.split_whitespace().collect();
            let lhs = names[parts[0]];
            let op = parts[1].chars().last().unwrap() as char;
            let rhs = names[parts[2]];
            results.push(Job::Operation { lhs, op, rhs });
        }
    }

    (results, names)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"root: pppw + sjmn
dbpl: 5
cczh: sllz + lgvd
zczc: 2
ptdq: humn - dvpt
dvpt: 3
lfqf: 4
humn: 5
ljgn: 2
sjmn: drzm * dbpl
sllz: 4
pppw: cczh / lfqf
lgvd: ljgn * ptdq
drzm: hmdt - zczc
hmdt: 32
"#;

    #[test]
    fn day21_part1_example1() {
        let actual = TwentyFirstSolution::part1(INPUT);
        assert_eq!(actual, "152");
    }

    #[test]
    fn day21_part2_example1() {
        let actual = TwentyFirstSolution::part2(INPUT);
        assert_eq!(actual, "301");
    }
}
