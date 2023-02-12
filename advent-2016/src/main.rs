use q1::run as run_q1;
use q2::run as run_q2;
use q3::run as run_q3;

use std::{env, process};

pub mod common;
mod q1;
mod q2;
mod q3;

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
        _ => panic!("wrong quiz number"),
    }
}
