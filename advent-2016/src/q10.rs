use crate::{common::my_modules::get_file, Runner};
use core::panic;
use lazy_static::lazy_static;
use regex::Regex;
use std::{any, collections::HashMap, hash::Hash, rc::Rc};

pub struct Q10 {
    bots: HashMap<usize, Bot>,
    bins: HashMap<usize, Vec<i32>>,
}

#[derive(Debug)]
struct Bot {
    vals: Vec<i32>,
    actions: (Rec, Rec),
}

impl Bot {
    fn new() -> Self {
        Bot {
            vals: Vec::new(),
            actions: (Rec::Empty, Rec::Empty),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Rec {
    Output(i32),
    Bot(i32),
    Empty,
}

impl Rec {
    fn parse(out: &str, val: &str) -> Self {
        match out {
            "bot" => Self::Bot(val.parse().unwrap()),
            "output" => Self::Output(val.parse().unwrap()),
            _ => panic!("output is invalid"),
        }
    }
}

fn parse(line: &str, bots: &mut HashMap<usize, Bot>) {
    let mut line: Vec<_> = line.split(" ").collect();

    match line[0] {
        "value" => bots
            .entry(line[5].parse::<usize>().unwrap())
            .or_insert(Bot::new())
            .vals
            .push(line[1].parse::<i32>().unwrap()),
        "bot" => {
            bots.entry(line[1].parse::<usize>().unwrap())
                .or_insert(Bot::new())
                .actions = (Rec::parse(line[5], line[6]), Rec::parse(line[10], line[11]))
        }
        _ => panic!("invalid input"),
    }
}

impl Q10 {
    pub fn new() -> Q10 {
        let content = get_file("q10.txt").unwrap();
        let mut bots = HashMap::new();
        let bins = HashMap::new();

        // println!("asdf");
        content.lines().for_each(|line| parse(&line, &mut bots));

        // println!("{instructions:?}");
        // println!("{:?}", bots);

        Q10 { bots, bins }
    }

    fn part1(&mut self) {
        loop {
            let (bot_num, bot) = match self.bots.iter_mut().find(|(_, p)| p.vals.len() == 2) {
                Some(bot) => bot,
                None => break,
            };

            let (low_val, high_val) = if bot.vals[0] <= bot.vals[1] {
                (bot.vals[0], bot.vals[1])
            } else {
                (bot.vals[1], bot.vals[0])
            };
            bot.vals.clear();

            let (low_action, high_action) = (bot.actions.0, bot.actions.1);

            // if low_val == 17 && high_val == 61 {
            //     println!("{bot_num:?} {bot:?}");
            //     return;
            // }

            match low_action {
                Rec::Bot(num) => self
                    .bots
                    .get_mut(&(num as usize))
                    .unwrap()
                    .vals
                    .push(low_val),
                Rec::Output(num) => self
                    .bins
                    .entry(num as usize)
                    .or_insert(Vec::new())
                    .push(low_val),
                Rec::Empty => panic!("invalid"),
            };

            match high_action {
                Rec::Bot(num) => self
                    .bots
                    .get_mut(&(num as usize))
                    .unwrap()
                    .vals
                    .push(high_val),
                Rec::Output(num) => self
                    .bins
                    .entry(num as usize)
                    .or_insert(Vec::new())
                    .push(high_val),
                Rec::Empty => panic!("invalid"),
            };
        }
    }
}

impl Runner for Q10 {
    fn run(&mut self) -> () {
        self.part1();
        println!("{:?}", self.bins);
        println!("{:?}", 67 * 11 * 31)
    }
}
