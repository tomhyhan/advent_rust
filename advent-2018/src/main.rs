use std::env;
use advent_2018::{copy_file, Runner};

use q1::Q1;
use q2::Q2;
use q3::Q3;

mod q1;
mod q2;
mod q3;

fn main() {
    let args : Vec<_>= env::args().collect();

    let mut q1 = Q1::new();
    let mut q2 = Q2::new();
    let mut q3 = Q3::new();

    match args[1].as_str() {
        "cp" => copy_file(args[2].as_str()).unwrap(),
        "1" => q1.run(),
        "2" => q2.run(),
        "3" => q3.run(),
        _ => panic!("invalid instruction!")
    }
}
