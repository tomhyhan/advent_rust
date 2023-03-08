use q1::run as run_q1;
use q10::Q10;
use q11::Q11;
use q12::Q12;
use q13::Q13;
use q14::Q14;
use q15::Q15;
use q16::Q16;
use q17::Q17;
use q18::Q18;
use q19::Q19;
use q2::run as run_q2;
use q20::Q20;
use q21::Q21;
use q22::Q22;
use q23::Q23;
use q24::Q24;
use q25::Q25;
use q3::run as run_q3;
use q4::run as run_q4;
use q5::run as run_q5;
use q6::run as run_q6;
use q7::run as run_q7;
use q8::Q8;
use q9::Q9;

use std::{env, process};

pub mod common;
mod q1;
mod q10;
mod q11;
mod q12;
mod q13;
mod q14;
mod q15;
mod q16;
mod q17;
mod q18;
mod q19;
mod q2;
mod q20;
mod q21;
mod q22;
mod q23;
mod q24;
mod q25;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod q9;

pub trait Runner {
    fn run(&mut self) -> ();
}

fn main() {
    let args = env::args();
    let config = common::Config::build(args).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    let mut q8 = Q8::new(3, 7);
    let mut q9 = Q9::new();
    let mut q10 = Q10::new();
    let mut q11 = Q11::new();
    let mut q12 = Q12::new();
    let mut q13 = Q13::new();
    let mut q14 = Q14::new();
    let mut q15 = Q15::new();
    let mut q16 = Q16::new();
    let mut q17 = Q17::new();
    let mut q18 = Q18::new();
    let mut q19 = Q19::new();
    let mut q20 = Q20::new();
    let mut q21 = Q21::new();
    let mut q22 = Q22::new();
    let mut q23 = Q23::new();
    let mut q24 = Q24::new();
    let mut q25 = Q25::new();
    
    match config.quiz {
        1 => run_q1(),
        2 => run_q2(),
        3 => run_q3(),
        4 => run_q4(),
        5 => run_q5(),
        6 => run_q6(),
        7 => run_q7(),
        8 => q8.run(),
        9 => q9.run(),
        10 => q10.run(),
        11 => q11.run(),
        12 => q12.run(),
        13 => q13.run(),
        14 => q14.run(),
        15 => q15.run(),
        16 => q16.run(),
        17 => q17.run(),
        18 => q18.run(),
        19 => q19.run(),
        20 => q20.run(),
        21 => q21.run(),
        22 => q22.run(),
        23 => q23.run(),
        24 => q24.run(),
        25 => q25.run(),
        _ => panic!("wrong quiz number"),
    }
}
