use advent_2017::{Runner, get_file};
use regex::Regex;
use lazy_static::lazy_static;

pub struct Q20 {
    particles: Vec<Particle>
}

#[derive(Debug)]
struct Particle {
    id: usize,
    position: (i128,i128,i128),
    velocity: (i128,i128,i128),
    acel: (i128,i128,i128),
}

impl Particle {
    fn move_once(&mut self) {
        self.velocity.0 +=  self.acel.0;
        self.velocity.1 +=  self.acel.1;
        self.velocity.2 +=  self.acel.2;

        self.position.0 +=  self.velocity.0;
        self.position.1 +=  self.velocity.1;
        self.position.2 +=  self.velocity.2;
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
        let closest = self.particles.iter().min_by(|x,y| (x.acel.0.abs() + x.acel.1.abs() + x.acel.2.abs()).cmp(&(y.acel.0.abs() + y.acel.1.abs() + y.acel.2.abs()))).map(|p| p.acel.0.abs() + p.acel.1.abs() + p.acel.2.abs()).unwrap();
        let s = self.particles.iter().enumerate().filter(|(_ ,p)| (p.acel.0.abs() + p.acel.1.abs() + p.acel.2.abs()) == closest).min_by(|a,b| (a.1.position.0.abs() + a.1.position.1.abs() + a.1.position.2.abs()).cmp(&(b.1.position.0.abs() + b.1.position.1.abs() + b.1.position.2.abs()))).unwrap();
        // 
        println!("{:?}", closest);
        println!("{:?}", s)
    }

}

fn parse(id: usize, line: &str) -> Particle{
    lazy_static!(
        static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
    );
    let nums: Vec<_>= RE.find_iter(line).map(|num| num.as_str().parse::<i128>().unwrap() ).collect();
    let position = (nums[0],nums[1],nums[2]);
    let velocity = (nums[3],nums[4],nums[5]);
    let acel = (nums[6],nums[7],nums[8]);
    Particle {id,acel,position,velocity}

}
impl Runner for Q20 {
    fn run(&mut self) {
        self.part1();
    }
}