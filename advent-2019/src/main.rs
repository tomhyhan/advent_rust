use advent_2019::{copy_file, Runner};
use std::env;
use q1::Q1;


mod q1;

fn main() {
    let args: Vec<_> = env::args().collect();
    let mut q1 = Q1::new();

    match args[1].as_str() {
        "cp" => copy_file(args[2].as_str()).unwrap(),
        "1" => q1.run(),
        _ => panic!("invalid instruction!"),
    }
}
