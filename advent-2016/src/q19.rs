use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    collections::HashMap,
};

use crate::Runner;

pub struct Q19 {}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Elf {
    visited: bool,
    presents: usize,
}

impl Elf {
    fn new() -> Self {
        Elf {
            visited: false,
            presents: 1,
        }
    }
}

#[derive(Debug)]
struct Table {
    elves: HashMap<usize, Elf>,
    length: usize,
}

impl Table {
    fn new(num_of_elves: usize) -> Self {
        let mut elves = HashMap::new();

        for n in 0..num_of_elves {
            elves.insert(n, Elf::new());
        }
        Table {
            length: elves.iter().len(),
            elves,
        }
    }

    fn get_next_elf(&mut self, turn: usize) -> usize {
        let mut next_turn = turn % self.length;

        loop {
            if !self.elves.get(&next_turn).unwrap().visited {
                return next_turn;
            }
            next_turn = (next_turn + 1) % self.length
        }
    }

    fn get_next_turn(&mut self, turn: usize) -> usize {}
}

impl Q19 {
    pub fn new() -> Self {
        Q19 {}
    }

    fn part1(&self) {
        let mut table = Table::new(3018458);
        let mut turn = 0;
        loop {
            if !table.elves.get(&turn).is_none() && table.elves.get(&turn).unwrap().visited {
                turn = (turn + 1) % table.length;
                continue;
            }
            // let next_turn = table.get_next_elf(turn + 1);
            let next_turn = table.get_next_turn(turn);
            // println!("{} {}", turn, next_turn);
            if turn == next_turn {
                println!("{}", turn + 1);
                break;
            }
            table.elves.get_mut(&turn).unwrap().presents +=
                table.elves.get_mut(&next_turn).unwrap().presents;
            table.elves.get_mut(&next_turn).unwrap().presents = 0;
            table.elves.get_mut(&next_turn).unwrap().visited = true;

            turn = (1 + next_turn) % table.length;
        }
        // println!("{:?}", table.elves);
    }
}

impl Runner for Q19 {
    fn run(&mut self) -> () {
        self.part1()
    }
}
