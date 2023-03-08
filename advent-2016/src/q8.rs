use core::panic;
use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use lazy_static::lazy_static;
use regex::Regex;

use crate::{common::my_modules::get_file, Runner};

#[derive(Debug)]
enum State {
    On,
    Off,
}

struct Display {
    screen: HashMap<(usize, usize), State>,
    row: i32,
    col: i32,
}

impl Display {
    fn new(row : i32, col : i32) -> Self {
        let screen = HashMap::new();
        Display { screen, row, col }
    }

    fn fetch_instruction(&mut self, ins: &str) {
        let mut ins = ins.split(" ");
        let mut new_screen = HashMap::new();
        match ins.next().unwrap() {
            "rect" =>  {
                let (col, row) = ins.next().unwrap().split_once("x").unwrap();

                for r in 0..row.parse::<usize>().unwrap() {
                    for c in 0..col.parse::<usize>().unwrap() {
                        new_screen.insert((r,c), State::On);
                    }
                }
                for ((row,col),_) in self.screen.iter() {
                    new_screen.insert((*row,*col), State::On); 
                }
                
            },
            "rotate" => {
                println!("asdf");
                let direction = ins.next().unwrap();
                match direction {
                    "row" => {
                        let (_, row_c) = ins.nth(0).unwrap().split_once("=").unwrap();
                        let move_to = ins.nth(1).unwrap().parse::<usize>().unwrap();
                        self.screen.iter().for_each(|((row,col),_)| {
                            if *row == row_c.parse().unwrap() {
                                let new_col = (*col + move_to) % self.col as usize;
                                new_screen.insert((*row,new_col),  State::On);
                            } else {
                                new_screen.insert((*row,*col), State::On);
                            }
                        })
                    },
                    "column" => {
                        let (_, col_c) = ins.nth(0).unwrap().split_once("=").unwrap();
                        let move_to = ins.nth(1).unwrap().parse::<usize>().unwrap();
                        println!("{:?} , {:?}", col_c,move_to);
                        self.screen.iter().for_each(|((row,col),_)| {
                            if *col == col_c.parse().unwrap() {
                                let new_row = (*row + move_to) % self.row as usize;
                                new_screen.insert((new_row,*col),  State::On);
                            } else {
                                new_screen.insert((*row,*col), State::On);
                            }
                        })
                    },
                    _ => panic!("invalid direction!")
                }
            }
            _ => panic!("invalid instruction")
        }
        self.screen = new_screen;
        println!("{:?}", self.screen);
    } 
}

#[derive(Debug)]
pub struct Q8 {
    instructions: String,
}

impl Q8 {
    pub fn new() -> Self {
        let instructions = get_file("q8.txt").unwrap();
        Q8 {
            instructions,
        }
    }

    fn part1(&self){
        let mut display = Display::new(6,50);
        self.instructions.lines().for_each(|line| display.fetch_instruction(line));
        let mut screen = vec![['.';50];6];
        
        for row in 0..6 {
            for col in 0..50 {
                if display.screen.contains_key(&(row,col)) {
                    screen[row][col] = '#';
                }
            }
        }

        for line in screen {
            println!("{:?}", line.iter().collect::<String>());
        }
    }

}

impl Runner for Q8 {
    fn run(&mut self) {
        self.part1();
    }
}

#[cfg(test)]
mod test{
    use super::*;

    // #[test]
    // fn turns_on_all_lights() {
    //     let mut display = Display::new(3,7);
    //     let ins = "rect 3x2";
    //     display.fetch_instruction(ins);
    //     assert_eq!(display.screen.len(), 6);
    // }

    #[test]
    fn move_lights() {
        let mut display = Display::new(3,7);
        let ins = "rect 3x2\nrotate column x=1 by 1";
        display.fetch_instruction(ins);
        print!("{:?}", display.screen);
        assert_eq!(display.screen.len(), 6);
    }
}