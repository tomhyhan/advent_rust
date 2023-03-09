use std::fs;
use std::env;
use advent_2017::Runner;
use q1::Q1;


mod q1;


fn copy_file(file_name: &str) {
    fs::copy("sample.rs", file_name).unwrap();
}


fn main() {
    let args : Vec<_>= env::args().collect();

    let mut q1 = Q1::new();

    match args[1].as_str() {
        "cp" => {copy_file(&args[2])},
        "1" => q1.run(),
        _ => panic!("invalid command")
    };
}
