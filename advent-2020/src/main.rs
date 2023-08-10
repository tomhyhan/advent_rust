use std::env;
use advent_2020::{copy_file, Runner, copy_txt};

use q1::Q1;
use q2::Q2;
use q7::Q7;


mod q1;
mod q2;
mod q7;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut q1 = Q1::new();
    let mut q2 = Q2::new();
    let mut q7 = Q7::new();


    match args[1].as_str() {
        "cp" => copy_file(args[2].as_str()).unwrap(),
        "txt" => copy_txt(args[2].as_str()).unwrap(),
        "1" => q1.run(),
        "2" => q2.run(),
        "7" => q7.run(),
        _ => panic!("invalid instruction!"),
    }
}
