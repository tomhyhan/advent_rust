use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let (query, path) = parse_config(&args);
    let contents = fs::read_to_string(path).expect("fail to read a file");

    println!("{contents:?}");
}

fn parse_config(args: &[String]) -> (&str, &str) {
    (&args[1], &args[2])
}
