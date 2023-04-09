use std::collections::HashMap;

use advent_2017::{get_file, Runner};

pub struct Q25 {}

#[derive(Debug)]
struct TuringMachine {
    tape: HashMap<i32, i32>,
    states: HashMap<char, State>,
    cursor: i32,
}

impl TuringMachine {
    fn new() -> Self {
        let content = get_file("q25.txt").unwrap();
        let mut states = HashMap::new();
        // windows
        content.split("\r\n\r\n").for_each(|state| {
            let lines: Vec<_> = state.lines().collect();
            let current_state = extract_value(lines[0]).chars().next().unwrap();
            let write_0 = extract_value(lines[2]).parse::<i32>().unwrap();
            let moveto_0 = left_or_right(&extract_value(lines[3]));
            let continue_0 = extract_value(lines[4]).chars().next().unwrap();
            let write_1 = extract_value(lines[6]).parse::<i32>().unwrap();
            let moveto_1 = left_or_right(&extract_value(lines[7]));
            let continue_1 = extract_value(lines[8]).chars().next().unwrap();
            let left_data = Data {
                write: write_0,
                moveto: moveto_0,
                contin: continue_0,
            };
            let right_data = Data {
                write: write_1,
                moveto: moveto_1,
                contin: continue_1,
            };
            states.insert(current_state, State::new(left_data, right_data));
        });
        let tape = HashMap::from([(0, 0)]);
        let cursor = 0;
        Self {
            tape,
            states,
            cursor,
        }
    }

    fn generate_checksums(&mut self, mut state: char, mut steps: i32) {
        while steps > 0 {
            let current_state = self.states.get(&state).unwrap();
            let checksum = self.tape.entry(self.cursor).or_insert(0);
            let moving;
            match checksum {
                0 => {
                    moving = &current_state.left;
                }
                1 => {
                    moving = &current_state.right;
                }
                _ => panic!("invalid checksum!"),
            };

            self.tape.insert(self.cursor.clone(), moving.write);
            self.cursor += moving.moveto;
            state = moving.contin;

            steps -= 1;
        }
    }
}

fn left_or_right(move_to: &str) -> i32 {
    match move_to {
        "right" => 1,
        "left" => -1,
        _ => panic!("unkown direction!"),
    }
}

fn extract_value(line: &str) -> String {
    line.split_whitespace()
        .last()
        .unwrap()
        .trim_matches(|p| p == ':' || p == '.')
        .to_string()
}

#[derive(Debug)]
struct State {
    left: Data,
    right: Data,
}

impl State {
    fn new(left: Data, right: Data) -> Self {
        State { left, right }
    }
}

impl State {
    fn diagnos(&self, checksum: i32, tape: &mut HashMap<i32, i32>, cursor: &mut i32) -> char {
        match checksum {
            0 => {
                tape.insert(cursor.clone(), self.left.write);
                *cursor += self.left.moveto;
                self.left.contin
            }
            1 => {
                tape.insert(cursor.clone(), self.right.write);
                *cursor += self.right.moveto;
                self.right.contin
            }
            _ => panic!("invalid checksum!"),
        }
    }
}

#[derive(Debug)]
struct Data {
    write: i32,
    moveto: i32,
    contin: char,
}

impl Q25 {
    pub fn new() -> Self {
        Q25 {}
    }

    fn part1(&mut self) {
        let mut machine = TuringMachine::new();
        machine.generate_checksums('A', 12459852);
        println!("{:?}", machine.tape.iter().filter(|(_, &v)| v == 1).count());
    }
}

impl Runner for Q25 {
    fn run(&mut self) {
        self.part1()
    }
}
