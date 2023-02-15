use q1::run as run_q1;
use q2::run as run_q2;
use q3::run as run_q3;
use q4::run as run_q4;
use q5::run as run_q5;
use q6::run as run_q6;
use q7::run as run_q7;

use std::{env, process};

pub mod common;
mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;

fn main() {
    let args = env::args();
    let config = common::Config::build(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    match config.quiz {
        1 => run_q1(),
        2 => run_q2(),
        3 => run_q3(),
        4 => run_q4(),
        5 => run_q5(),
        6 => run_q6(),
        7 => run_q7(),
        _ => panic!("wrong quiz number"),
    }
}
