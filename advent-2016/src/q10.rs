use crate::{common::my_modules::get_file, Runner};
use lazy_static::lazy_static;
use regex::Regex;
use std::{any, collections::HashMap, hash::Hash, rc::Rc};

#[derive(Debug)]
struct Bot {
    num: i32,
    vals: Vec<Option<i32>>,
}

// impl Bot {
//     fn new() {
//         Bot {

//         }
//     }
// }
#[derive(Debug)]
pub struct Q10 {
    instructions: HashMap<i32, Vec<Instruction>>,
    bots: Vec<Bot>,
}

#[derive(Debug, Clone, Copy)]
enum Instruction {
    output(i32),
    bot(i32),
}

fn parse(line: &str, bots: &mut Vec<Bot>, instructions: &mut HashMap<i32, Vec<Instruction>>) {
    if line.starts_with("value") {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        };
        let nums: Vec<_> = RE.find_iter(line).collect();
        let value = nums[0].as_str().parse::<i32>().unwrap();
        let bot_num = nums[1].as_str().parse::<i32>().unwrap();
        let bot = bots.iter_mut().find(|p| p.num == bot_num);
        if bot.is_none() {
            bots.push(Bot {
                num: bot_num,
                vals: vec![Some(value)],
            })
        } else {
            bot.unwrap().vals.push(Some(value));
        }
    } else if line.starts_with("bot") {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"output \d+|bot \d+").unwrap();
        }
        let data: Vec<_> = RE.find_iter(line).collect();
        // println!("{data:?}");
        let (_, bot) = data[0].as_str().split_once(" ").unwrap();
        let (low, low_val) = data[1].as_str().split_once(" ").unwrap();
        let (high, high_val) = data[2].as_str().split_once(" ").unwrap();
        let mut ins = Vec::new();
        if low.contains("output") {
            ins.push(Instruction::output(low_val.parse::<i32>().unwrap()));
        } else {
            ins.push(Instruction::bot(low_val.parse::<i32>().unwrap()));
        }

        if high.contains("output") {
            ins.push(Instruction::output(high_val.parse::<i32>().unwrap()));
        } else {
            ins.push(Instruction::bot(high_val.parse::<i32>().unwrap()));
        }

        // if high.contains("output") {}

        instructions.insert(bot.parse::<i32>().unwrap(), ins);
    }
}

impl Q10 {
    pub fn new() -> Q10 {
        let content = get_file("q10.txt").unwrap();
        let mut instructions = HashMap::new();
        let mut bots = Vec::new();

        content
            .lines()
            .for_each(|line| parse(&line, &mut bots, &mut instructions));

        println!("{instructions:?}");
        println!("{bots:?}");

        Q10 { instructions, bots }
    }

    fn part1(&mut self) {}
}

impl Runner for Q10 {
    fn run(&mut self) -> () {
        self.part1()
    }
}
