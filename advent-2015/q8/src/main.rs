use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn parse(line: &str) -> usize {
    let result = line.len();
    let x_matches = Regex::new(r"\\x[0-9a-fA-F]{2}").unwrap();
    let q_matches = Regex::new(r#"""#).unwrap();
    let e_matches = Regex::new(r"\\\\").unwrap();

    let new_line = line.trim_matches(|c| c == '"');

    // println!("{}", line);
    // println!("{}", line.trim_matches(|c| c == '"'));
    // println!("{}", new_line);
    // println!("")
    // println!("{new_line:?}")

    // matches
    //     .captures_iter(new_line)
    //     .for_each(|caps| println!("{:?}", caps.name("x").unwrap_or(caps.get(1).unwrap())))

    let x = x_matches.find_iter(new_line).count();
    let y = q_matches.find_iter(new_line).count();
    let z = e_matches.find_iter(new_line).count();
    let escapes = (x * 3) + y + z;
    println!("{result}");
    // println!("{}", in_memory + 2);
    println!("{}", result - escapes - 2);
    // println!();

    escapes + 2
}

fn main() {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("file to be read");

    let r: usize = context.split("\n").map(|line| parse(line)).sum();

    println!("{r}");
    println!("bvm\x28aa\\\\\"pywuhaniox\\z\\hbpdxd7mold");
}
