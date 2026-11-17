use crate::{aoc2022::Solution, utils::Point};

pub struct FifteenthSolution;

impl Solution for FifteenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let data = parse_sensors_and_beacons(input);
        cannot_contain_beacon(&data, 2000000).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let data = parse_sensors_and_beacons(input);
        let distress_beacon = distress_beacon_placement(&data, 4000000);
        tuning_freq(distress_beacon).to_string()
    }
}

fn parse_sensors_and_beacons(input: &str) -> Vec<(Point, Point)> {
    let mut result = Vec::new();

    for line in input.trim().lines() {
        let num_str: String = line
            .chars()
            .map(|c| if c.is_digit(10) || c == '-' { c } else { ' ' })
            .collect();
        let nums = num_str.split_whitespace().collect::<Vec<_>>();
        result.push((
            Point::new(nums[0].parse().unwrap(), nums[1].parse().unwrap()),
            Point::new(nums[2].parse().unwrap(), nums[3].parse().unwrap()),
        ));
    }

    result
}

fn covered_intervals(data: &[(Point, Point)], row: i64) -> Vec<(i64, i64)> {
    // find intervals, merge them, collect total
    let mut intervals: Vec<(i64, i64)> = Vec::new();
    for (s, b) in data {
        let dist = s.manhattan_distance(*b);

        // ignore what outside of sensor range
        let dy = (s.y - row).abs() as u64;
        if dy > dist {
            continue;
        }

        // if in range - get dx which will represent range coverage
        // in [s.x - dx, s.x + dx]
        let dx = dist - dy;
        intervals.push((s.x - dx as i64, s.x + dx as i64))
    }

    // merge - sort by starts, if a.start <= b.end + 1 -> b.end.max(a.end)
    intervals.sort_unstable();
    let mut merged: Vec<(i64, i64)> = Vec::new();
    for (start, end) in intervals {
        if let Some(last) = merged.last_mut() {
            if start <= last.1 + 1 {
                last.1 = last.1.max(end);
                continue;
            }
        }
        merged.push((start, end));
    }
    merged
}

fn cannot_contain_beacon(data: &[(Point, Point)], target_row: i64) -> u64 {
    covered_intervals(data, target_row)
        .iter()
        .map(|i| i.1 - i.0 + 1)
        .sum::<i64>() as u64
        - 1
}

fn distress_beacon_placement(data: &[(Point, Point)], largest: i64) -> Point {
    for y in 0..=largest {
        let clamped_intervals: Vec<(i64, i64)> = covered_intervals(data, y)
            .iter()
            .map(|i| (i.0.max(0), i.1.min(largest)))
            .filter(|i| i.0 < i.1)
            .collect();

        if clamped_intervals.is_empty() {
            panic!("can't have all uncovered space in [0;{largest}]");
        }

        // check if there is only one range and it could be off by 1 - possible place
        if clamped_intervals.first().unwrap().0 - 1 == 0 {
            return (0, y).into();
        } else if clamped_intervals.last().unwrap().1 + 1 == largest {
            return (largest, y).into();
        }
        if clamped_intervals.len() == 1 {
            continue;
        }

        let mut prev = clamped_intervals[0];
        for i in &clamped_intervals[1..] {
            if prev.1 < i.0 {
                return (prev.1 + 1, y).into();
            }
            prev = *i;
        }
    }
    unreachable!()
}

fn tuning_freq(p: Point) -> u64 {
    p.x as u64 * 4000000 + p.y as u64
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"Sensor at x=2, y=18: closest beacon is at x=-2, y=15
Sensor at x=9, y=16: closest beacon is at x=10, y=16
Sensor at x=13, y=2: closest beacon is at x=15, y=3
Sensor at x=12, y=14: closest beacon is at x=10, y=16
Sensor at x=10, y=20: closest beacon is at x=10, y=16
Sensor at x=14, y=17: closest beacon is at x=10, y=16
Sensor at x=8, y=7: closest beacon is at x=2, y=10
Sensor at x=2, y=0: closest beacon is at x=2, y=10
Sensor at x=0, y=11: closest beacon is at x=2, y=10
Sensor at x=20, y=14: closest beacon is at x=25, y=17
Sensor at x=17, y=20: closest beacon is at x=21, y=22
Sensor at x=16, y=7: closest beacon is at x=15, y=3
Sensor at x=14, y=3: closest beacon is at x=15, y=3
Sensor at x=20, y=1: closest beacon is at x=15, y=3
"#;

    #[test]
    fn day15_part1_example1() {
        let data = parse_sensors_and_beacons(INPUT);
        let actual = cannot_contain_beacon(&data, 10);
        assert_eq!(26, actual);
    }

    #[test]
    fn day15_part2_example1() {
        let data = parse_sensors_and_beacons(INPUT);
        let distress_beacon = distress_beacon_placement(&data, 20);
        assert_eq!(56000011, tuning_freq(distress_beacon));
    }
}
