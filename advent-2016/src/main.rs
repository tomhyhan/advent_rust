use q1::run as run_q1;
use q2::run as run_q2;
use q3::run as run_q3;
use q4::run as run_q4;
use q5::run as run_q5;
use q6::run as run_q6;
use q7::run as run_q7;
use q8::Q8 as run_q8;

use std::{env, process};

pub mod common;
mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;

pub trait Runner {
    fn run(&mut self) {}
}

fn main() {
    let args = env::args();
    let config = common::Config::build(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let mut q8 = run_q8::new(3, 7);
    match config.quiz {
        1 => run_q1(),
        2 => run_q2(),
        3 => run_q3(),
        4 => run_q4(),
        5 => run_q5(),
        6 => run_q6(),
        7 => run_q7(),
        8 => q8.run(),
        _ => panic!("wrong quiz number"),
    }
}
