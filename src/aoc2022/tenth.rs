use crate::aoc2022::Solution;

pub struct TenthSolution;

impl Solution for TenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut cycle: i64 = 1;
        let mut reg_x: i64 = 1;
        let mut strength: i64 = 0;
        for line in input.trim().lines().map(|l| l.trim()) {
            if cycle > 221 {
                break;
            }
            if line.starts_with("noop") {
                cycle += 1;
                if (cycle - 20) % 40 == 0 {
                    strength += cycle * reg_x;
                }
            } else {
                cycle += 2;
                let val = line[5..].parse::<i64>().unwrap();
                reg_x += val;
                if (cycle - 20) % 40 == 0 {
                    strength += cycle * reg_x;
                } else if (cycle - 21) % 40 == 0 {
                    // must revert register value in case of first tick
                    strength += (cycle - 1) * (reg_x - val);
                }
            }
        }
        strength.to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut result = String::from("\n");
        let mut cycle = 0;
        let mut sprite_mid = 1;
        for line in input.trim().lines().map(|l| l.trim()) {
            draw_pixel(&mut result, cycle, sprite_mid);
            cycle += 1;
            if line.starts_with("noop") {
                continue;
            } else {
                draw_pixel(&mut result, cycle, sprite_mid);
                cycle += 1;
                sprite_mid += line[5..].parse::<i64>().unwrap();
            }
        }
        result
    }
}

fn draw_pixel(s: &mut String, cycle: i64, sprite_mid: i64) {
    if cycle >= 240 {
        return;
    }
    let cycle_row = cycle % 40;
    if cycle_row >= sprite_mid - 1 && cycle_row <= sprite_mid + 1 {
        s.push('#');
    } else {
        s.push('.');
    }
    // new line for row but not for last one
    if (cycle + 1) % 40 == 0 && cycle != 239 {
        s.push('\n');
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"addx 15
addx -11
addx 6
addx -3
addx 5
addx -1
addx -8
addx 13
addx 4
noop
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx 5
addx -1
addx -35
addx 1
addx 24
addx -19
addx 1
addx 16
addx -11
noop
noop
addx 21
addx -15
noop
noop
addx -3
addx 9
addx 1
addx -3
addx 8
addx 1
addx 5
noop
noop
noop
noop
noop
addx -36
noop
addx 1
addx 7
noop
noop
noop
addx 2
addx 6
noop
noop
noop
noop
noop
addx 1
noop
noop
addx 7
addx 1
noop
addx -13
addx 13
addx 7
noop
addx 1
addx -33
noop
noop
noop
addx 2
noop
noop
noop
addx 8
noop
addx -1
addx 2
addx 1
noop
addx 17
addx -9
addx 1
addx 1
addx -3
addx 11
noop
noop
addx 1
noop
addx 1
noop
noop
addx -13
addx -19
addx 1
addx 3
addx 26
addx -30
addx 12
addx -1
addx 3
addx 1
noop
noop
noop
addx -9
addx 18
addx 1
addx 2
noop
noop
addx 9
noop
noop
noop
addx -1
addx 2
addx -37
addx 1
addx 3
noop
addx 15
addx -21
addx 22
addx -6
addx 1
noop
addx 2
addx 1
noop
addx -10
noop
noop
addx 20
addx 1
addx 2
addx 2
addx -6
addx -11
noop
noop
noop
"#;

    #[test]
    fn day10_part1_example1() {
        let actual = TenthSolution::part1(INPUT);
        assert_eq!("13140", actual);
    }

    #[test]
    fn day10_part2_example1() {
        let expected = r#"
##..##..##..##..##..##..##..##..##..##..
###...###...###...###...###...###...###.
####....####....####....####....####....
#####.....#####.....#####.....#####.....
######......######......######......####
#######.......#######.......#######....."#;
        let actual = TenthSolution::part2(INPUT);
        assert_eq!(expected, actual);
    }
}
