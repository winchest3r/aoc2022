use crate::{aoc2022::Solution, utils::SPLIT_PATTERN};

pub struct ThirteenthSolution;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Packet {
    Int(u64),
    List(Vec<Packet>),
}

impl Solution for ThirteenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let mut right_order = 0;

        for (i, pair) in input.trim().split(SPLIT_PATTERN).enumerate() {
            let (p1_str, p2_str) = pair.trim().split_once('\n').unwrap();
            let p1 = parse_packet(&mut p1_str.trim().chars().peekable());
            let p2 = parse_packet(&mut p2_str.trim().chars().peekable());

            if compare_packets(&p1, &p2) == std::cmp::Ordering::Less {
                right_order += i + 1;
            }
        }

        right_order.to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        let mut packets: Vec<Packet> = input
            .trim()
            .split_whitespace()
            .map(|s| parse_packet(&mut s.chars().peekable()))
            .collect();

        let divider1 = Packet::List(vec![Packet::List(vec![Packet::Int(2)])]);
        let divider2 = Packet::List(vec![Packet::List(vec![Packet::Int(6)])]);

        packets.push(divider1.clone());
        packets.push(divider2.clone());

        packets.sort_by(compare_packets);

        let a = packets.iter().position(|p| *p == divider1).unwrap() + 1;
        let b = packets.iter().position(|p| *p == divider2).unwrap() + 1;

        (a * b).to_string()
    }
}

fn compare_packets(p1: &Packet, p2: &Packet) -> std::cmp::Ordering {
    // use ordering is simpler when  recursively processing packets
    match (p1, p2) {
        (Packet::Int(a), Packet::Int(b)) => a.cmp(&b),
        (Packet::Int(_), Packet::List(_)) => compare_packets(&Packet::List(vec![p1.clone()]), p2),
        (Packet::List(_), Packet::Int(_)) => compare_packets(p1, &Packet::List(vec![p2.clone()])),
        (Packet::List(a), Packet::List(b)) => {
            for (x, y) in a.iter().zip(b.iter()) {
                let ordering = compare_packets(x, y);

                if ordering != std::cmp::Ordering::Equal {
                    return ordering;
                }
            }

            a.len().cmp(&b.len())
        }
    }
}

/// Form packet from peekable iterator.
fn parse_packet(chars: &mut std::iter::Peekable<std::str::Chars>) -> Packet {
    match chars.peek() {
        Some('[') => {
            chars.next();
            let mut items: Vec<Packet> = Vec::new();

            while chars.peek() != Some(&']') {
                if chars.peek() == Some(&',') {
                    chars.next();
                } else {
                    // recursively parse all
                    items.push(parse_packet(chars));
                }
            }
            chars.next();

            Packet::List(items)
        }
        Some(_) => {
            let mut number = String::new();

            while let Some(c) = chars.peek() {
                if c.is_ascii_digit() {
                    number.push(*c);
                    chars.next();
                } else {
                    break;
                }
            }

            Packet::Int(number.parse::<u64>().unwrap())
        }
        None => unreachable!(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"[1,1,3,1,1]
[1,1,5,1,1]

[[1],[2,3,4]]
[[1],4]

[9]
[[8,7,6]]

[[4,4],4,4]
[[4,4],4,4,4]

[7,7,7,7]
[7,7,7]

[]
[3]

[[[]]]
[[]]

[1,[2,[3,[4,[5,6,7]]]],8,9]
[1,[2,[3,[4,[5,6,0]]]],8,9]
"#;

    #[test]
    fn day13_part1_example1() {
        let actual = ThirteenthSolution::part1(INPUT);
        assert_eq!("13", actual);
    }

    #[test]
    fn day13_part2_example1() {
        let actual = ThirteenthSolution::part2(INPUT);
        assert_eq!("140", actual);
    }
}
