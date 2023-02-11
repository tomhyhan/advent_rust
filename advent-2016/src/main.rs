use q1::run as run_q1;
use std::{env, process};

pub mod common;
mod q1;
fn main() {
    let args = env::args();
    let config = common::Config::build(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    match config.quiz {
        1 => run_q1(),
        _ => panic!("wrong quiz number"),
    }
    // let content = common::get_file("asdf").unwrap_or_else(|err| {
    //     println!("{err}");
    //     process::exit(1);
    // });

    // println!("{content}");
}
