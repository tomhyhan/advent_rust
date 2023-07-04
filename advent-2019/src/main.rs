use advent_2019::{copy_file, Runner};
use std::env;
use q1::Q1;
use q2::Q2;
use q4::Q4;
use q5::Q5;


mod q1;
mod q2;
mod q4;
mod q5;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut q1 = Q1::new();
    let mut q2 = Q2::new();
    let mut q4 = Q4::new();
    let mut q5 = Q5::new();

    match args[1].as_str() {
        "cp" => copy_file(args[2].as_str()).unwrap(),
        "1" => q1.run(),
        "2" => q2.run(),
        "4" => q4.run(),
        "5" => q5.run(),
        _ => panic!("invalid instruction!"),
    }
}
