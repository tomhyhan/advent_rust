use crate::common::my_modules::get_file;
use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet, VecDeque};

pub fn run() {
    let binding = get_file("q1.txt").unwrap();
    let content: Vec<_> = binding.split(", ").collect();
    println!("{:?}", content);

    find_shortest_path(content);
}

lazy_static! {
    static ref DIRECTIONS: HashMap<Direction, (i32, i32)> = HashMap::from([
        (Direction::East, (1, 0)),
        (Direction::West, (-1, 0)),
        (Direction::North, (0, -1)),
        (Direction::South, (0, 1))
    ]);
}

#[derive(Debug)]
struct Coord {
    x: i32,
    y: i32,
    direction: Direction,
}

#[derive(Debug, PartialEq, Hash, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Coord {
    fn new() -> Self {
        Coord {
            x: 0,
            y: 0,
            direction: Direction::North,
        }
    }
}

fn find_shortest_path(content: Vec<&str>) {
    let mut instructions = VecDeque::from(content);

    let mut coord: Coord = Coord::new();

    let mut visited = HashSet::new();

    while let Some(instruction) = instructions.pop_front() {
        let mut instruction = instruction.chars();
        let face = instruction.next().unwrap();
        let distance = instruction.as_str().parse::<i32>().unwrap();

        change_direction(face, &mut coord);

        match coord.direction {
            Direction::North => {
                if find_intersection(distance, &mut visited, &mut coord, Direction::North) {
                    break;
                }
            }
            Direction::South => {
                if find_intersection(distance, &mut visited, &mut coord, Direction::South) {
                    break;
                }
            }
            Direction::West => {
                if find_intersection(distance, &mut visited, &mut coord, Direction::West) {
                    break;
                }
            }
            Direction::East => {
                if find_intersection(distance, &mut visited, &mut coord, Direction::East) {
                    break;
                }
            }
        }
    }

    println!("{coord:?}");
    // println!("{visited:?}");
}

fn find_intersection(
    distance: i32,
    visited: &mut HashSet<(i32, i32)>,
    coord: &mut Coord,
    direction: Direction,
) -> bool {
    for _ in 1..=distance {
        let move_to = DIRECTIONS.get(&direction).unwrap();
        coord.y += move_to.0;
        coord.x += move_to.1;
        if visited.contains(&(coord.x, coord.y)) {
            return true;
        }
        visited.insert((coord.x, coord.y));
    }
    false
}
fn change_direction(face: char, coord: &mut Coord) {
    match face {
        'R' => match coord.direction {
            Direction::North => coord.direction = Direction::East,
            Direction::South => coord.direction = Direction::West,
            Direction::East => coord.direction = Direction::South,
            Direction::West => coord.direction = Direction::North,
        },
        'L' => match coord.direction {
            Direction::North => coord.direction = Direction::West,
            Direction::South => coord.direction = Direction::East,
            Direction::East => coord.direction = Direction::North,
            Direction::West => coord.direction = Direction::South,
        },
        _ => panic!("invalid direction"),
    }
}

#[cfg(test)]
mod tests {
    use mockall_double::double;

    use super::*;

    // #[double]
    use crate::common::mock_my_modules;

    #[test]
    fn mock_read_file() {
        let ctx = mock_my_modules::get_file_context();
        ctx.expect().returning(|x| Ok("asdf".to_string()));
        assert_eq!(Ok("asdf".to_string()), mock_my_modules::get_file("asd"));
    }

    #[test]
    fn it_change_direction_to_east() {
        let mut coord = Coord::new();
        let face = 'R';

        change_direction(face, &mut coord);
        assert_eq!(coord.direction, Direction::East);
    }

    #[test]
    fn it_change_direction_to_west() {
        let mut coord = Coord::new();
        let face = 'L';
        change_direction(face, &mut coord);
        assert_eq!(coord.direction, Direction::West);
    }

    #[test]
    #[should_panic]
    fn it_should_chagne_to_wrong_direction() {
        let mut coord = Coord::new();
        let face = 'R';

        change_direction(face, &mut coord);
        assert_eq!(coord.direction, Direction::West);
    }
}
