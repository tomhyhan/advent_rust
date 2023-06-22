use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static; 

use advent_2018::{Runner, get_file};

pub struct Q23 {

}

struct Teleportation {
    nanobots: HashMap<(i32,i32,i32), i32>,
    max_nanobot: (i32,i32,i32),
    max_signal: i32
}

impl Teleportation {
    fn new() -> Self {
        let content = get_file("src/input/q23.txt").unwrap();
        let mut nanobots = HashMap::new();
        let mut max_signal = i32::MIN;
        let mut max_nanobot = (0,0,0);
        content.lines().for_each(|line| {
            lazy_static!(
                static ref RE: Regex = Regex::new(r"[-]?\d+").unwrap();
            );
            let nums: Vec<_>= RE.find_iter(line).collect();
            let (x,y,z,r) = (nums[0].as_str().parse::<i32>().unwrap(),nums[1].as_str().parse::<i32>().unwrap(),nums[2].as_str().parse::<i32>().unwrap(),nums[3].as_str().parse::<i32>().unwrap());
            nanobots.insert((x,y,z), r);
            if r > max_signal {
                max_signal = max_signal.max(r);
                max_nanobot = (x,y,z)
            } 
        });
        Self {nanobots, max_nanobot, max_signal}
    }

    // pos=<10,12,12>, r=2  => cover from 8,12,12 | 10,10,12 | 10,12,10 to 
    // pos=<12,14,12>, r=2  
    // pos=<16,12,12>, r=4
    // pos=<14,14,14>, r=6
    // pos=<50,50,50>, r=200
    // pos=<10,10,10>, r=5

    fn find_nanobots_within(&self) {
        println!("{:?}", self.max_nanobot);
        println!("{:?}", self.max_signal);
        let nanobots_within = self.nanobots.iter().filter(|((x,y,z), _)|{
            let (max_x, max_y, max_z) = self.max_nanobot;
            let r = (max_x - x).abs() + (max_y - y).abs() + (max_z - z).abs(); 
            r <= self.max_signal
        }).count();
        println!("{:?}", nanobots_within);

        println!("done");
    }
}

impl Q23 {
    pub fn new() -> Self {
        Q23 {}
    }

    fn part1(&mut self) {
        let teleportation = Teleportation::new();
        teleportation.find_nanobots_within();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q23 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q23() {
        assert_eq!(1, 1);
    }
}