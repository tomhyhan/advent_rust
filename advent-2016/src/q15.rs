use crate::{common::my_modules::get_file, Runner};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug)]
struct Disk {
    idx: usize,
    positions: usize,
}

#[derive(Debug)]
pub struct Q15 {
    sculpture: Vec<Disk>,
    time: usize,
}

fn parse(line: &str) -> Disk {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let mut info = RE
        .find_iter(line)
        .enumerate()
        .filter(|(idx, _)| *idx == 1 || *idx == 3);

    let positions = info.next().unwrap().1.as_str().parse::<usize>().unwrap();
    let idx = info.next().unwrap().1.as_str().parse::<usize>().unwrap();

    Disk { idx, positions }
}

impl Q15 {
    pub fn new() -> Self {
        let context = get_file("q15.txt").unwrap();

        let sculpture: Vec<Disk> = context.lines().map(|line| parse(line)).collect();
        Q15 { sculpture, time: 0 }
    }

    fn part1(&mut self) {
        'bounce: loop {
            let mut get_capsule = true;
            for n in 1..=self.sculpture.len() {
                let c_s = &self.sculpture[n - 1];
                if (c_s.idx + n + self.time) % c_s.positions != 0 {
                    get_capsule = false
                }
            }
            if !get_capsule {
                self.time += 1;
                continue 'bounce;
            }
            break;
        }
        println!("{:?}", self.time);
        // 3208583
    }
}

impl Runner for Q15 {
    fn run(&mut self) -> () {
        self.part1()
    }
}
