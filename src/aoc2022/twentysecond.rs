use std::collections::{HashMap, HashSet, VecDeque};

use crate::{
    aoc2022::Solution,
    utils::{Point, Point3, SPLIT_PATTERN},
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

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
enum Direction {
    Up,
    #[default]
    Right,
    Down,
    Left,
}

impl Direction {
    fn index(self) -> usize {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }

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

/// possibly should name it like this from beginning heh
type Vec3 = Point3;

#[derive(Debug, Clone, Copy)]
struct Orientation {
    right: Vec3,
    down: Vec3,
    normal: Vec3,
}

impl Orientation {
    pub const INITIAL: Orientation = Orientation {
        right: Vec3::new(1, 0, 0),
        down: Vec3::new(0, 1, 0),
        normal: Vec3::new(0, 0, 1),
    };
}

#[derive(Debug, Clone, Copy)]
struct Face {
    block: Point,
    origin: Point,
    orientation: Orientation,
}

/// Main structure that contains info
/// about destination face id, direction
/// and in which side we move
#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
struct EdgeTransition {
    pub destination: usize,
    pub direction: Direction,
    pub reversed: bool,
}

/// Every face of cube (6) have 4 edges.
type TransitionTable = [[EdgeTransition; 4]; 6];

pub struct TwentySecondSolution;

impl Solution for TwentySecondSolution {
    fn part1(#[allow(unused)] input: &str) -> String {
        let (map, actions) = parse_data(input);
        let walker = walk(&map, &actions);
        calculate_password(walker).to_string()
    }

    fn part2(#[allow(unused)] input: &str) -> String {
        // do the same but basically generate edge transitions -
        // information about face change during walking around cube's surface.
        // face can be represented with face id (related to initial data),
        // basis vector for orientation (right, down, normal), starting point,
        //

        let (map, actions) = parse_data(input);
        let walker = walk_cube(&map, &actions);
        calculate_password(walker).to_string()
    }
}

fn calculate_password(walker: (Point, Direction)) -> i64 {
    1000 * (walker.0.y + 1) + 4 * (walker.0.x + 1) + &walker.1.into()
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

fn walk_cube(map: &HashMap<Point, Tile>, actions: &[Action]) -> (Point, Direction) {
    // total tiles = 6 faces * cube_size^2, cube_size = sqrt(tiles / faces)
    let cube_size: i64 = ((map.len() / 6) as f64).sqrt() as i64;

    // get faces with right orientations
    let (mut faces, face_by_block) = parse_faces(map, cube_size);
    assign_orientations(&mut faces, &face_by_block);

    // build transition table
    let transitions = build_transition_table(&faces);

    let init_point = (0..)
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

                    let current = walker.0;
                    let dir = walker.1;

                    let face_id = faces
                        .iter()
                        .position(|f| {
                            current.x >= f.origin.x
                                && current.x < f.origin.x + cube_size
                                && current.y >= f.origin.y
                                && current.y < f.origin.y + cube_size
                        })
                        .unwrap();
                    let face = &faces[face_id];

                    let (candidate, candidate_dir) =
                        // check if we at any corner
                        if is_crossing_edge(current, dir, face, cube_size) {
                            // if so - get transition and cross edge
                            let transition = transitions[face_id][dir.index()];
                            cross_edge(face, current, dir, transition, &faces, cube_size)
                        } else {
                            // or just walk by face
                            (current + dir.into(), dir)
                        };

                    // if blocked - cant move and leave
                    if map[&candidate] == Tile::Wall {
                        break;
                    }

                    walker = (candidate, candidate_dir);
                }
            }
        }
    }

    walker
}

fn is_crossing_edge(p: Point, dir: Direction, face: &Face, cube_size: i64) -> bool {
    let x = p.x - face.origin.x;
    let y = p.y - face.origin.y;

    match dir {
        Direction::Right => x == cube_size - 1,
        Direction::Left => x == 0,
        Direction::Down => y == cube_size - 1,
        Direction::Up => y == 0,
    }
}

fn fold(o: Orientation, dir: Direction) -> Orientation {
    // need to fold neighbor faces to assing orientations at some point
    // basically reorientate and rotate relatively to possible walker

    // orientation rotate to right -> normal takes o.right, right takes o.normal.negate(), down the same.
    // and so on.

    match dir {
        Direction::Right => Orientation {
            right: o.normal.negate(),
            down: o.down,
            normal: o.right,
        },
        Direction::Left => Orientation {
            right: o.normal,
            down: o.down,
            normal: o.right.negate(),
        },
        Direction::Down => Orientation {
            right: o.right,
            down: o.normal.negate(),
            normal: o.down,
        },
        Direction::Up => Orientation {
            right: o.right,
            down: o.normal,
            normal: o.down.negate(),
        },
    }
}

fn parse_faces(map: &HashMap<Point, Tile>, cube_size: i64) -> (Vec<Face>, HashMap<Point, usize>) {
    // get data related to faces from initial map
    // returns vector of faces and map with points divided to blocks and their face id

    // shift depends on point position in initial map
    let offset_x = map.keys().map(|p| p.x).min().unwrap() % cube_size;
    let offset_y = map.keys().map(|p| p.y).min().unwrap() % cube_size;
    let offset = Point::new(offset_x, offset_y);

    // save block as set - we don't care about collisions because
    // we have cube - all faces are the same in size
    let mut blocks = HashSet::new();
    for &p in map.keys() {
        blocks.insert((p - offset) / cube_size);
    }
    // order by rows, cols
    let mut blocks: Vec<Point> = blocks.into_iter().collect();
    blocks.sort_by_key(|p| (p.y, p.x));

    let mut faces = Vec::new();
    let mut face_by_block = HashMap::new();

    for (id, block) in blocks.into_iter().enumerate() {
        let origin = offset + block * cube_size;

        faces.push(Face {
            block,
            origin,
            orientation: Orientation::INITIAL, // will be replaced with right orientation next
                                               // in assign_orientations
        });

        face_by_block.insert(block, id);
    }

    (faces, face_by_block)
}

fn assign_orientations(faces: &mut [Face], face_by_block: &HashMap<Point, usize>) {
    // just walk with BFS and assign orientations related to walking directions
    // using fold function and previous point orientation in case of corner walk

    let mut assigned: Vec<Option<Orientation>> = vec![None; faces.len()];
    let mut queue = VecDeque::new();

    assigned[0] = Some(Orientation::INITIAL);
    queue.push_back(0);

    while let Some(id) = queue.pop_front() {
        let orientation = assigned[id].unwrap();
        let block = faces[id].block;

        for dir in [
            Direction::Right,
            Direction::Down,
            Direction::Left,
            Direction::Up,
        ] {
            let next_block = block + dir.into();

            // outside of block - continue or get face id
            let Some(&next_id) = face_by_block.get(&next_block) else {
                continue;
            };

            // first time at this face
            if assigned[next_id].is_none() {
                // rotate orientation compare to movement direciton
                assigned[next_id] = Some(fold(orientation, dir));
                queue.push_back(next_id);
            }
        }
    }

    // update orientations (must have (1,0,0), (-1,0,0), (0,1,0), (0,-1,0), (0,0,1), (0,0,-1) in some order)

    for (face, orientation) in faces.iter_mut().zip(assigned) {
        face.orientation = orientation.unwrap();
    }
}

fn cross_edge(
    face: &Face,
    position: Point,
    dir: Direction,
    transition: EdgeTransition,
    faces: &[Face],
    cube_size: i64,
) -> (Point, Direction) {
    // main function for transition between edges

    let local = position - face.origin;

    // offset along the edge we are leaving
    let mut t = match dir {
        Direction::Right | Direction::Left => local.y,
        Direction::Up | Direction::Down => local.x,
    };
    t = if transition.reversed {
        cube_size - 1 - t
    } else {
        t
    };

    let target = &faces[transition.destination];
    // offset along the edge we are entering
    let p = match transition.direction {
        Direction::Right => Point::new(0, t),
        Direction::Down => Point::new(t, 0),
        Direction::Left => Point::new(cube_size - 1, t),
        Direction::Up => Point::new(t, cube_size - 1),
    };

    (target.origin + p, transition.direction)
}

fn build_transition_table(faces: &[Face]) -> TransitionTable {
    // construct transition table related to every edge for every face
    let mut result = [[EdgeTransition::default(); 4]; 6];

    for face_id in 0..result.len() {
        for dir_idx in 0..4 {
            let dir = match dir_idx {
                0 => Direction::Right,
                1 => Direction::Down,
                2 => Direction::Left,
                3 => Direction::Up,
                _ => unreachable!(),
            };

            let source = faces[face_id].orientation;
            let target_normal = destination_normal(source, dir);
            let target_id = faces
                .iter()
                .position(|f| f.orientation.normal == target_normal)
                .unwrap();
            let target = faces[target_id].orientation;

            let new_direction = direction_from_vector(source.normal.negate(), target);
            let source_tangent = edge_tangent(source, dir);
            let target_tangent = edge_tangent(target, new_direction);

            // dot product will be -1 if source tangent and target's are opposite (cos(180deg))
            let reversed = source_tangent.dot(target_tangent) == -1;

            let edge_trans = EdgeTransition {
                destination: target_id,
                direction: new_direction,
                reversed,
            };

            result[face_id][dir_idx] = edge_trans;
        }
    }

    result
}

fn destination_normal(o: Orientation, dir: Direction) -> Vec3 {
    // get vector in direction related to current orientation

    match dir {
        Direction::Right => o.right,
        Direction::Down => o.down,
        Direction::Left => o.right.negate(),
        Direction::Up => o.down.negate(),
    }
}

fn direction_from_vector(v: Vec3, o: Orientation) -> Direction {
    // oposite to function above

    if v == o.right {
        Direction::Right
    } else if v == o.down {
        Direction::Down
    } else if v == o.right.negate() {
        Direction::Left
    } else if v == o.down.negate() {
        Direction::Up
    } else {
        unreachable!()
    }
}

fn edge_tangent(o: Orientation, dir: Direction) -> Vec3 {
    // get tangent for edges

    match dir {
        Direction::Right | Direction::Left => o.down,
        Direction::Down | Direction::Up => o.right,
    }
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
