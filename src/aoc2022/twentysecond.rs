use std::collections::HashMap;

use crate::{
    aoc2022::Solution,
    utils::{Point, SPLIT_PATTERN},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Free,
    Wall,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Action {
    Move(i64),
    Rotate(char),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn rotate(self, d: char) -> Self {
        match self {
            Direction::Up => {
                if d == 'R' {
                    Direction::Right
                } else {
                    Direction::Left
                }
            }
            Direction::Right => {
                if d == 'R' {
                    Direction::Down
                } else {
                    Direction::Up
                }
            }
            Direction::Down => {
                if d == 'R' {
                    Direction::Left
                } else {
                    Direction::Right
                }
            }
            Direction::Left => {
                if d == 'R' {
                    Direction::Up
                } else {
                    Direction::Down
                }
            }
        }
    }
}

impl From<Direction> for Point {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => Point::UP,
            Direction::Right => Point::RIGHT,
            Direction::Down => Point::DOWN,
            Direction::Left => Point::LEFT,
        }
    }
}

impl From<Direction> for i64 {
    fn from(value: Direction) -> Self {
        match value {
            Direction::Up => 3,
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
        }
    }
}

pub struct TwentySecondSolution;

impl Solution for TwentySecondSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let (map, actions) = parse_data(input);
        let walker = walk(&map, &actions);
        let pass: i64 = 1000 * (walker.0.y + 1) + 4 * (walker.0.x + 1) + &walker.1.into();
        pass.to_string()
    }
}

fn walk(map: &HashMap<Point, Tile>, actions: &[Action]) -> (Point, Direction) {
    let len_x = map.keys().max_by_key(|p| p.x).unwrap().x + 1;
    let len_y = map.keys().max_by_key(|p| p.y).unwrap().y + 1;

    let init_point = (0..len_x)
        .map(|x| Point::new(x, 0))
        .find(|p| map.contains_key(p) && map[p] == Tile::Free)
        .unwrap();
    let mut walker: (Point, Direction) = (init_point, Direction::Right);

    for &act in actions {
        match act {
            Action::Rotate(d) => walker.1 = walker.1.rotate(d),
            Action::Move(mut amount) => {
                while amount > 0 {
                    amount -= 1;
                    let mut new_p = walker.0 + walker.1.into();
                    // wrap around depends on direction of movements
                    // just take first free tile from map
                    if !map.contains_key(&new_p) {
                        new_p = match walker.1 {
                            Direction::Up => (0..len_y)
                                .rev()
                                .map(|y| Point::new(new_p.x, y))
                                .find(|p| map.contains_key(p))
                                .unwrap(),
                            Direction::Down => (0..len_y)
                                .map(|y| Point::new(new_p.x, y))
                                .find(|p| map.contains_key(p))
                                .unwrap(),
                            Direction::Left => (0..len_x)
                                .rev()
                                .map(|x| Point::new(x, new_p.y))
                                .find(|p| map.contains_key(p))
                                .unwrap(),
                            Direction::Right => (0..len_x)
                                .map(|x| Point::new(x, new_p.y))
                                .find(|p| map.contains_key(p))
                                .unwrap(),
                        };
                    }
                    if map[&new_p] == Tile::Wall {
                        break;
                    }
                    walker.0 = new_p;
                }
            }
        }
    }

    walker
}

fn parse_data(input: &str) -> (HashMap<Point, Tile>, Vec<Action>) {
    let mut map = HashMap::new();
    let mut actions = Vec::new();

    let (map_data, action_data) = input.trim_end().split_once(SPLIT_PATTERN).unwrap();

    for (y, line) in map_data.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                ' ' => continue,
                '.' => Tile::Free,
                '#' => Tile::Wall,
                _ => unreachable!(),
            };
            map.insert(Point::new(x as i64, y as i64), tile);
        }
    }

    let mut number = String::new();
    for c in action_data.chars() {
        if c.is_ascii_digit() {
            number.push(c);
        } else {
            let rot = Action::Rotate(c);
            if !number.is_empty() {
                actions.push(Action::Move(number.parse::<i64>().unwrap()));
                number.clear();
            }
            actions.push(rot);
        }
    }
    if !number.is_empty() {
        actions.push(Action::Move(number.parse::<i64>().unwrap()));
    }

    (map, actions)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#"        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5
"#;

    #[test]
    fn day22_part1_example1() {
        let actual = TwentySecondSolution::part1(INPUT);
        assert_eq!(actual, "6032");
    }

    #[test]
    fn day22_part2_example1() {
        let actual = TwentySecondSolution::part2(INPUT);
        assert_eq!(actual, "5031");
    }
}
