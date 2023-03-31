use std::{cmp::Ordering, collections::HashSet};

use advent_2017::{Runner, get_file};
use regex::Regex;
use lazy_static::lazy_static;

pub struct Q20 {
    particles: Vec<Particle>
}

#[derive(Debug, Clone)]
struct Particle {
    id: usize,
    position: Coordinate,
    velocity: Coordinate,
    accel: Coordinate,
}

impl Particle {
    fn move_once(&mut self) {
        self.velocity.x +=  self.accel.x;
        self.velocity.y +=  self.accel.y;
        self.velocity.z +=  self.accel.z;

        self.position.x +=  self.velocity.x;
        self.position.y +=  self.velocity.y;
        self.position.z +=  self.velocity.z;
    }
}

impl Q20 {
    pub fn new() -> Self {
        let content = get_file("q20.txt").unwrap();
        let particles: Vec<Particle> = content.lines().enumerate().map(|(idx,line)| {
            parse(idx, line)
        }).collect();
        Q20 {particles}
    }

    fn part1(&mut self) {
        let smallest_accel = self.particles.iter().min_by(|p1,p2|p1.accel.cmp(&p2.accel)).unwrap();
        println!("{:?}", smallest_accel);
    }

    fn part2(&mut self) {
        for _ in 0..10000 {
            let mut particles = HashSet::new();
            let mut collisions = HashSet::new();
            for particle in self.particles.iter_mut() {
                particle.move_once();
            }
            for particle in self.particles.iter() {
                if !particles.insert(particle.position.clone()) {
                    collisions.insert(particle.position.clone());
                };
            }

            self.particles = self.particles.clone().into_iter().filter(|p| !collisions.contains(&p.position)).collect::<Vec<_>>();
            println!("{:?}", self.particles.len());
        }
    }
}

#[derive(Debug, Eq, Clone, Hash)]
struct Coordinate {
    x:isize,
    y:isize,
    z:isize
}

impl Coordinate {
    fn new(x:isize,y:isize,z:isize) -> Coordinate {
        Coordinate {x,y,z}
    }
    fn dist(&self) -> isize {
        self.x.abs() + self.y.abs() + self.z.abs()
    } 
}

impl Ord for Coordinate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.dist().cmp(&other.dist())
    }
}

impl PartialOrd for Coordinate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Coordinate {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z 
    }
}


fn parse(id: usize, line: &str) -> Particle{
    lazy_static!(
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    );
    let nums: Vec<_>= RE.find_iter(line).map(|num| num.as_str().parse::<isize>().unwrap() ).collect();
    let position = Coordinate::new(nums[0],nums[1],nums[2]);
    let velocity = Coordinate::new(nums[3],nums[4],nums[5]);
    let accel = Coordinate::new(nums[6],nums[7],nums[8]);
    Particle {id,accel,position,velocity}
}

impl Runner for Q20 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}