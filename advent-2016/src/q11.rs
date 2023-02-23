use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    thread::current,
};

use crate::{common::my_modules::get_file, Runner};

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum RTG {
    Chips(String),
    Generator(String),
}

#[derive(Debug, Default, Clone)]
struct Floor {
    chips: HashSet<RTG>,
    generators: HashSet<RTG>,
}

#[derive(Debug, Default, Clone)]
struct Building {
    floors: Vec<Floor>,
    elevator: usize,
}

impl Building {
    fn new(floor: usize) -> Self {
        Building {
            floors: vec![Floor::default(); floor],
            elevator: 0,
        }
    }

    fn add_chip(&mut self, chip: RTG, floor: usize) {
        self.floors[floor].chips.insert(chip);
    }

    fn add_gen(&mut self, generator: RTG, floor: usize) {
        self.floors[floor].generators.insert(generator);
    }
}

#[derive(Debug)]
pub struct Q11 {
    building: Building,
}

impl Q11 {
    fn parse(&mut self) {
        //  test
        self.building.add_chip(RTG::Chips("H".to_string()), 0);
        self.building.add_chip(RTG::Chips("L".to_string()), 0);
        self.building.add_gen(RTG::Generator("H".to_string()), 1);
        self.building.add_gen(RTG::Generator("L".to_string()), 2);
    }

    pub fn new() -> Self {
        Q11 {
            building: Building::new(4),
        }
    }

    fn part1(&mut self) {
        dijkstra(&self.building);
    }
}

impl Runner for Q11 {
    fn run(&mut self) -> () {
        self.parse();
        self.part1();
        println! {"{:?}", self.building}
    }
}

fn dijkstra(building: &Building) {
    // let mut queue = HashSet::new();
    // let mut dist = HashMap::new();
    // let mut visited = HashSet::new();

    //     queue.insert(building.clone());
    //     dist.insert(building.clone(), 0);
    //     visited.insert(building.clone());

    //     while !queue.is_empty() {}
}
