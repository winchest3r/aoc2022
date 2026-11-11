mod aoc2022;
mod utils;

use std::time::Instant;

use aoc2022::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() != 2 {
        println!("Usage: {} <day[1..=25]>", args[0]);
        return;
    }

    let Ok(day) = args[1].parse::<u8>() else {
        println!("Error during argument parsing: must be a number");
        return;
    };

    if !(1u8..=25u8).contains(&day) {
        println!("Error during number range check: must be in range [1..=25]");
        return;
    }

    run(day);
}

fn run(day: u8) {
    match day {
        1 => run_day::<FirstSolution>(day),
        2 => run_day::<SecondSolution>(day),
        3 => run_day::<ThirdSolution>(day),
        4 => run_day::<FourthSolution>(day),
        other => {
            if (1u8..=25u8).contains(&other) {
                println!("unimplemented day!")
            } else {
                panic!("Forbidden value for task day: {other}");
            }
        }
    }
}

fn run_day<T: Solution>(day: u8) {
    let fname = format!("data/day{day:02}.dat");
    let input = std::fs::read_to_string(fname).unwrap();

    let mut now = Instant::now();
    let part1_res = T::part1(&input);
    let part1_elapsed = now.elapsed();

    now = Instant::now();
    let part2_res = T::part2(&input);
    let part2_elapsed = now.elapsed();

    let total_elapsed = part1_elapsed + part2_elapsed;

    const SECS_IN_MICROSECS: f64 = 1000000.0;

    println!("--------------day{day:2}--------------");
    println!(
        "part1:\t{part1_res}\t\t{:.6}s",
        part1_elapsed.as_micros() as f64 / SECS_IN_MICROSECS
    );
    println!(
        "part2:\t{part2_res}\t\t{:.6}s",
        part2_elapsed.as_micros() as f64 / SECS_IN_MICROSECS
    );
    println!(
        "total:\t\t\t{:.6}s",
        total_elapsed.as_micros() as f64 / SECS_IN_MICROSECS
    )
}
