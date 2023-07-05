use std::collections::HashSet;

use advent_2019::{get_file, Runner};

pub struct Q3 {}

#[derive(Debug)]
enum Direction {
    Up(i32),
    Down(i32),
    Left(i32),
    Right(i32),
}

fn add_up_number(bytes: &[u8]) -> i32 {
    let mut sum = 0;
    let mut idx = (bytes.len() - 1) as i32;
    let mut mul = 1;

    while idx >= 0 {
        sum += (bytes[idx as usize] as char).to_digit(10).unwrap() * mul;
        idx -= 1;
        mul *= 10;
    }
    sum as i32
}

impl Direction {
    fn parse(dir: &str) -> Direction {
        let dir_b = dir.as_bytes();
        match dir_b[0] as char {
            'U' => Direction::Up(add_up_number(&dir_b[1..])),
            'D' => Direction::Down(add_up_number(&dir_b[1..])),
            'R' => Direction::Right(add_up_number(&dir_b[1..])),
            'L' => Direction::Left(add_up_number(&dir_b[1..])),
            _ => panic!("unknown input!"),
        }
    }
}

struct System {
    wires: Vec<Vec<Direction>>,
}

impl System {
    fn new() -> Self {
        let content = get_file("src/input/q3.txt").unwrap();
        let wires = content
            .lines()
            .map(|line| line.split(",").map(|dir| Direction::parse(dir)).collect())
            .collect();
        Self { wires }
    }

    fn find_matching_points(&self) -> HashSet<(i32, i32)> {
        let mut matching = HashSet::new();
        let mut first_visits = HashSet::new();
        for wire in self.wires.iter() {
            let mut visited = HashSet::new();
            let mut pos = (0, 0);
            for dir in wire.iter() {
                let move_info;
                match dir {
                    Direction::Up(distance) => move_info = (-1, 0, *distance),
                    Direction::Down(distance) => move_info = (1, 0, *distance),
                    Direction::Right(distance) => move_info = (0, 1, *distance),
                    Direction::Left(distance) => move_info = (0, -1, *distance),
                }
                self.move_to(
                    &mut pos,
                    move_info,
                    &mut visited,
                    &mut matching,
                    &first_visits,
                );
            }
            first_visits = visited
        }
        matching.remove(&(0, 0));
        // self.find_min_distance_to_core(matching)
        matching
    }

    fn find_min_distance_to_core(&self, matching: HashSet<(i32, i32)>) -> i32 {
        self.manhattan_distance(
            *matching
                .iter()
                .min_by(|&&pos1, &&pos2| {
                    self.manhattan_distance(pos1)
                        .cmp(&self.manhattan_distance(pos2))
                })
                .unwrap(),
        )
    }

    fn manhattan_distance(&self, pos: (i32, i32)) -> i32 {
        pos.0.abs() + pos.1.abs()
    }

    fn move_to(
        &self,
        pos: &mut (i32, i32),
        move_info: (i32, i32, i32),
        visited: &mut HashSet<(i32, i32)>,
        matching: &mut HashSet<(i32, i32)>,
        first_visits: &HashSet<(i32, i32)>,
    ) {
        let (y, x, mut distance) = move_info;
        while distance > 0 {
            if first_visits.contains(pos) {
                matching.insert(pos.clone());
            }
            visited.insert(pos.clone());
            pos.0 += y;
            pos.1 += x;
            distance -= 1
        }
    }

    fn find_steps_to_matching(&self, matching: &HashSet<(i32, i32)>) {
        let mut steps_matching = HashSet::new();
        for match_ in matching.iter() {
            let mut distances = (0, 0);
            for (turn, wire) in self.wires.iter().enumerate() {
                let mut pos = (0, 0);
                let mut steps = 0;
                'outer: for dir in wire.iter() {
                    let move_info;
                    match dir {
                        Direction::Up(distance) => move_info = (-1, 0, *distance),
                        Direction::Down(distance) => move_info = (1, 0, *distance),
                        Direction::Right(distance) => move_info = (0, 1, *distance),
                        Direction::Left(distance) => move_info = (0, -1, *distance),
                    }
                    let (y, x, mut distance) = move_info;
                    while distance > 0 {
                        pos.0 += y;
                        pos.1 += x;
                        distance -= 1;
                        steps += 1;
                        if pos == *match_ {
                            match turn {
                                0 => distances.1 = steps,
                                1 => distances.0 = steps,
                                _ => panic!("invalid index"),
                            }
                            break 'outer;
                        }
                    }
                }
            }
            steps_matching.insert(distances);
        }
        println!(
            "{:?}",
            steps_matching
                .iter()
                .min_by(|x, y| (x.0 + x.1).cmp(&(y.0 + y.1)))
                .map(|(x, y)| x + y)
                .unwrap()
        );
    }
}

impl Q3 {
    pub fn new() -> Self {
        Q3 {}
    }

    fn part1(&mut self) {
        let system = System::new();
        println!("{:?}", system.wires);
        println!("{:?}", system.find_matching_points())
    }

    fn part2(&mut self) {
        let system = System::new();
        let matching = system.find_matching_points();
        system.find_steps_to_matching(&matching);
    }
}

impl Runner for Q3 {
    fn part1(&mut self) {
        self.part1()
    }

    fn part2(&mut self) {
        self.part2()
    }
}

#[cfg(test)]
mod test {
    #[test]
    fn Q3() {
        assert_eq!(1, 1);
    }
}
