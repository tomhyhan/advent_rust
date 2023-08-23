use std::env;
use advent_2020::{copy_file, Runner, copy_txt};

use q1::Q1;
use q2::Q2;
use q3::Q3;
use q4::Q4;
use q5::Q5;
use q6::Q6;
use q7::Q7;
use q8::Q8;
use q9::Q9;
use q10::Q10;
use q11::Q11;
use q15::Q15;
use q16::Q16;
use q17::Q17;
use q18::Q18;


mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;
mod q7;
mod q8;
mod q9;
mod q10;
mod q11;
mod q15;
mod q16;
mod q17;
mod q18;

fn main() {
    let args: Vec<_> = env::args().collect();

    let mut q1 = Q1::new();
    let mut q2 = Q2::new();
    let mut q3 = Q3::new();
    let mut q4 = Q4::new();
    let mut q5 = Q5::new();
    let mut q6 = Q6::new();
    let mut q7 = Q7::new();
    let mut q8 = Q8::new();
    let mut q9 = Q9::new();
    let mut q10 = Q10::new();
    let mut q11 = Q11::new();
    let mut q15 = Q15::new();
    let mut q16 = Q16::new();
    let mut q17 = Q17::new();
    let mut q18 = Q18::new();


    match args[1].as_str() {
        "cp" => copy_file(args[2].as_str()).unwrap(),
        "txt" => copy_txt(args[2].as_str()).unwrap(),
        "1" => q1.run(),
        "2" => q2.run(),
        "3" => q3.run(),
        "4" => q4.run(),
        "5" => q5.run(),
        "6" => q6.run(),
        "7" => q7.run(),
        "8" => q8.run(),
        "9" => q9.run(),
        "10" => q10.run(),
        "11" => q11.run(),
        "15" => q15.run(),
        "16" => q16.run(),
        "17" => q17.run(),
        "18" => q18.run(),
        _ => panic!("invalid instruction!"),
    }
}
