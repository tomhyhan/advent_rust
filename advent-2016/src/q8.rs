use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{common::my_modules::get_file, Runner};

#[derive(Debug)]
enum State {
    on,
    off,
}

#[derive(Debug)]
pub struct Q8 {
    screen: HashMap<(usize, usize), State>,
    instructions: Instructions,
    row: i32,
    col: i32,
}

impl Q8 {
    pub fn new(row: usize, col: usize) -> Self {
        let mut screen = HashMap::new();

        for y in 0..=row {
            for x in 0..=col {
                screen.insert((x, y), State::off);
            }
        }

        let content = get_file("q8.txt").unwrap();
        let instructions: Instructions = content.parse().unwrap();

        Q8 {
            screen,
            instructions,
            row: row as i32,
            col: col as i32,
        }
    }

    fn display(&mut self) {
        self.instructions.contents.iter().for_each(|line| {
            let mut ins = line.split(" ");
            match ins.next().unwrap() {
                "rect" => {
                    let (col, row) = ins.next().unwrap().split_once("x").unwrap();
                    for y in 0..=row.parse::<usize>().unwrap() {
                        for x in 0..=col.parse::<usize>().unwrap() {
                            self.screen.insert((x, y), State::on);
                        }
                    }
                }
                "rotate" => {
                    // println!("asdf");
                    let ins_rc = if ins.next().unwrap() == "row" {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"\d+").unwrap();
                        }
                        let rest = ins.collect::<String>();
                        println!("{rest:?}");
                        let nums: Vec<_> = RE.find_iter(&rest).collect();
                        let current_row = nums[0].as_str().parse::<i32>().unwrap();
                        let right = nums[0].as_str().parse::<i32>().unwrap();
                        // let mut ons = HashSet::new();
                        // self.screen
                        //     .iter_mut()
                        //     .filter(|(k, v)| k.0 as i32 == current_row)
                        //     .for_each(|(key, value)| {
                        //         self.screen.insert(*key, State::off);
                        //         self.screen.insert(
                        //             (
                        //                 ((key.0 as i32 + right) % self.row).try_into().unwrap(),
                        //                 key.1,
                        //             ),
                        //             State::on,
                        //         );
                        //     });
                    } else {
                    };
                }
                _ => panic!("unknown instructions"),
            }
        });

        println!("{:?}", self.instructions);
    }
}

impl Runner for Q8 {
    fn run(&mut self) {
        self.display();
        println!("{:?}", self.screen);
    }
}
#[derive(Debug)]
struct Instructions {
    contents: Vec<String>,
}

impl FromStr for Instructions {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(Instructions {
            contents: s.lines().map(|line| line.to_string()).collect(),
        })
    }
}
