use std::{collections::HashMap, str::FromStr};

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
                        let nums = RE.captures(&rest).unwrap();
                        println!("{:?}", nums.get(0).map_or("", |m| m.as_str()));
                        // for caps in nums.iter() {
                        //     println!(" a - {caps:?}")
                        // }
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
