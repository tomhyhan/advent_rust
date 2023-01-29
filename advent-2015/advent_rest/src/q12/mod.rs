use std::fs;

// .map(|line| line.to_string())

fn parse() -> Vec<String> {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("cannot read a file");
    context.lines().map(|line| line.to_string()).collect()
}

pub fn run_code() {
    parse().iter().for_each(|line| println!("{line}"));
}
