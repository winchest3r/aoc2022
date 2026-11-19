use std::collections::{HashMap, HashSet, VecDeque};

use crate::{aoc2022::Solution, utils::Point};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
struct State {
    pub rock_idx: usize,
    pub move_idx: usize,
    pub profile: Vec<u8>, // represents as row masks, because we need to reachable holes inside field's top
}

pub struct SeventeenthSolution;

impl Solution for SeventeenthSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        process(input, 2022).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        // we can't simulate 1 trillion of rocks
        // need to save state - which represented by
        // current rock shape - rock idx basically
        // current move - move idx basically
        // profile - top values in every row
        //
        // and process with cycle detection
        // if there is saved state with current data -
        // just take it and add its height to total
        // if not - construct new data and save it.

        process_with_cycle_detection(input, 1000000000000).to_string()
    }
}

fn get_rocks() -> Vec<Vec<Point>> {
    let mut result = Vec::new();
    // ####
    result.push(vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(2, 0),
        Point::new(3, 0),
    ]);
    // .#.
    // ###
    // .#.
    result.push(vec![
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
        Point::new(2, 1),
        Point::new(1, 2),
    ]);
    // ..#
    // ..#
    // ###
    result.push(vec![
        Point::new(2, 2),
        Point::new(2, 1),
        Point::new(2, 0),
        Point::new(1, 0),
        Point::new(0, 0),
    ]);
    // #
    // #
    // #
    // #
    result.push(vec![
        Point::new(0, 0),
        Point::new(0, 1),
        Point::new(0, 2),
        Point::new(0, 3),
    ]);
    // ##
    // ##
    result.push(vec![
        Point::new(0, 0),
        Point::new(1, 0),
        Point::new(0, 1),
        Point::new(1, 1),
    ]);
    result
}

const FIELD_WIDTH: i64 = 7;
const LEFT_SIDE_SHIFT: i64 = 2;

fn process_with_cycle_detection(input: &str, rocks: usize) -> i64 {
    let mut field: HashSet<Point> = HashSet::new();
    for x in 0..FIELD_WIDTH {
        field.insert(Point::new(x, 0));
    }

    let rock_shapes = get_rocks();
    let moves: Vec<char> = input.trim().chars().collect();

    let mut move_idx = 0;
    let mut field_top = 0;

    let mut rock_count = 0;
    let mut skipped_height = 0;

    // saves state with current number of rocks and tower height
    let mut seen: HashMap<State, (usize, i64)> = HashMap::new();
    let mut cycle_skipped = false;

    while rock_count < rocks {
        let rock_idx = rock_count % rock_shapes.len();
        let mut rock = rock_shapes[rock_idx].clone();
        // shift rock
        for p in rock.iter_mut() {
            p.x += LEFT_SIDE_SHIFT;
            p.y += field_top + 4;
        }
        // simulate one rock
        loop {
            let movement = moves[move_idx];
            move_idx = (move_idx + 1) % moves.len();

            match movement {
                '>' => rock = try_move(&field, rock, true),
                '<' => rock = try_move(&field, rock, false),
                _ => unreachable!(),
            }

            if !try_drop(&mut field, &mut rock) {
                field_top = field_top.max(rock.iter().map(|p| p.y).max().unwrap());
                break;
            }
        }

        // check cycles
        rock_count += 1;
        if !cycle_skipped {
            // form current state
            let state = State {
                rock_idx,
                move_idx,
                profile: get_profile(&field, field_top),
            };

            // if state presented - skip cycled rocks and height
            if let Some(&(prev_rocks, prev_height)) = seen.get(&state) {
                let cycle_rocks = rock_count - prev_rocks;
                let cycle_height = field_top - prev_height;

                let remaining = rocks - rock_count;
                let cycles = remaining / cycle_rocks;

                rock_count += cycles * cycle_rocks;
                skipped_height += cycles as i64 * cycle_height;

                cycle_skipped = true;
            } else {
                seen.insert(state, (rock_count, field_top));
            }
        }
    }

    field_top + skipped_height
}

fn get_profile(field: &HashSet<Point>, field_top: i64) -> Vec<u8> {
    // constructs bitmask array from top to all reachable points

    let top = field_top + 1;

    // just process with dfs
    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();

    // all above can be added immediately
    for x in 0..FIELD_WIDTH {
        queue.push_back((x, top));
        visited.insert((x, top));
    }

    let directions = [(1, 0), (-1, 0), (0, 1), (0, -1)];
    let mut min_y = top;
    while let Some((x, y)) = queue.pop_front() {
        min_y = min_y.min(y);

        for (dx, dy) in directions {
            let nx = x + dx;
            let ny = y + dy;

            if nx < 0 || nx >= FIELD_WIDTH || ny < 1 || ny > top {
                continue;
            }
            if field.contains(&Point::new(nx, ny)) {
                continue;
            }

            if visited.insert((nx, ny)) {
                queue.push_back((nx, ny));
            }
        }
    }

    // encode all rows containing reachable points
    let mut profile = Vec::new();
    for y in min_y..=field_top {
        let mut mask = 0;
        for x in 0..FIELD_WIDTH {
            if field.contains(&Point::new(x, y)) {
                mask |= 1 << x;
            }
        }
        profile.push(mask);
    }

    profile
}

fn process(input: &str, rocks: usize) -> i64 {
    let mut field: HashSet<Point> = HashSet::new();
    for i in 0..FIELD_WIDTH {
        field.insert(Point::new(i, 0));
    }

    let mut moves = input.trim().chars().cycle();
    let mut field_top: i64 = 0; // highest point regarding to field bottom at start and rock points further.
    for mut rock in get_rocks().into_iter().cycle().take(rocks) {
        // shift rock
        // in while
        //      move rock (attept to)
        //      drop it till we can
        for p in rock.iter_mut() {
            p.x += LEFT_SIDE_SHIFT;
            p.y += field_top + 4; // +4 because intial equal to 0 - field bottom
        }
        loop {
            match moves.next().unwrap() {
                '>' => rock = try_move(&field, rock, true),
                '<' => rock = try_move(&field, rock, false),
                _ => unreachable!(),
            }
            if !try_drop(&mut field, &mut rock) {
                field_top = field_top.max(rock.iter().map(|p| p.y).max().unwrap());
                break;
            }
        }
    }

    field_top
}

fn try_move(field: &HashSet<Point>, rock: Vec<Point>, right: bool) -> Vec<Point> {
    let mut updated = rock.clone();

    for p in updated.iter_mut() {
        if right {
            p.x += 1;
            if p.x >= FIELD_WIDTH || field.contains(&p) {
                return rock;
            }
        } else {
            p.x -= 1;
            if p.x < 0 || field.contains(&p) {
                return rock;
            }
        }
    }

    updated
}

fn try_drop(field: &mut HashSet<Point>, rock: &mut [Point]) -> bool {
    for &p in rock.iter() {
        let new_p = p + Point::new(0, -1);
        if field.contains(&new_p) {
            // add saved position of rock into field
            field.extend(rock.iter());
            return false;
        }
    }
    for p in rock.iter_mut() {
        p.y -= 1;
    }
    true
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &'static str = r#">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>"#;

    #[test]
    fn day17_part1_example1() {
        let actual = SeventeenthSolution::part1(INPUT);
        assert_eq!("3068", actual);
    }

    #[test]
    fn day17_part2_example1() {
        let actual = SeventeenthSolution::part2(INPUT);
        assert_eq!("1514285714288", actual);
    }
}
