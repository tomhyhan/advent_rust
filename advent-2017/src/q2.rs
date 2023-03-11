use std::cmp;

use advent_2017::{Runner, get_file};

pub struct Q2 {
    
}

struct Spreadsheet {
    lines : Vec<Vec<i32>>
}

impl Spreadsheet {
    fn new() -> Self {
        let content = get_file("q2.txt").unwrap();
        let lines = content.lines().map(|line| {
            let new_line = line.split_whitespace().map(|num| num.to_string().parse().unwrap()).collect();
            new_line
        }).collect();
        Spreadsheet { lines }
    }
}

impl Q2 {
    pub fn new() -> Self {
        Q2 {}
    }

    fn part1(&mut self) -> i32{
        let spreadsheet = Spreadsheet::new();
        let total : i32 = spreadsheet.lines.iter().map(|line| {
            let mut max = i32::MIN;
            let mut min = i32::MAX;
            for num in line.iter() {
                max = cmp::max(max, *num);
                min = cmp::min(min, *num);
            };
            max - min
        }).sum();
        total
    }

    fn part2(&mut self) -> i32 {
        let mut spreadsheet = Spreadsheet::new();
        let total : i32 = spreadsheet.lines.iter_mut().map(|line| {
            line.sort();
            for i in 0..line.len() {
                for j in i+1..line.len() {
                    if line[j] % line[i] == 0 {
                        return line[j] / line[i]
                    }
                }
            }
            panic!("pair does not exist")
        }).sum();
        total
    }
}

impl Runner for Q2 {
    fn run(&mut self) {
        println!("{:?}", self.part1());
        println!("{:?}", self.part2());
    }
}