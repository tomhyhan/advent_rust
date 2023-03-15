use std::fs;
use std::env;
use advent_2017::Runner;
use q1::Q1;
use q2::Q2;
use q3::Q3;
use q4::Q4;
use q5::Q5;
use q6::Q6;

mod q1;
mod q2;
mod q3;
mod q4;
mod q5;
mod q6;

fn copy_file(file_name: &str) {
    fs::copy("sample.rs", format!("./src/{file_name}.rs")).unwrap();
}

fn main() {
    let args : Vec<_>= env::args().collect();

    let mut q1 = Q1::new();
    let mut q2 = Q2::new();
    let mut q3 = Q3::new();
    let mut q4 = Q4::new();
    let mut q5 = Q5::new();
    let mut q6 = Q6::new();

    match args[1].as_str() {
        "cp" => {copy_file(&args[2])},
        "1" => q1.run(),
        "2" => q2.run(),
        "3" => q3.run(),
        "4" => q4.run(),
        "5" => q5.run(),
        "6" => q6.run(),
        _ => panic!("invalid command")
    };
}
