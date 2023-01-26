use eval::eval;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;

fn parse(line: &str) -> usize {
    let result = line.len();
    // \\[x][0-9a-f]{2}
    let x_matches = Regex::new(r"\\x[a-f0-9]{2}").unwrap();
    let q_matches = Regex::new(r#"""#).unwrap();
    let e_matches = Regex::new(r"\\\\").unwrap();

    let new_line = line;

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
    // println!("{result}");
    // println!("{}", in_memory + 2);
    // println!("{x}");
    // println!("{y}");
    println!("{}", escapes);
    // println!();

    escapes
}

fn decode(line: &str) -> usize {
    let mut idx = 0;

    let b_line = line.as_bytes();
    let mut result = 0;

    while idx < line.len() - 1 {
        match b_line[idx] {
            b'\"' => {
                idx += 1;
                continue;
            }
            b'\\' => {
                if b_line[idx + 1] == b'\"' || b_line[idx + 1] == b'\\' {
                    result += 1;
                    idx += 2;
                } else if b_line[idx + 1] == b'x' {
                    result += 3;
                    idx += 4;
                }
            }
            _ => idx += 1,
        }
    }
    println! {"{}", result + 2};
    result + 2
}

fn encode(line: &str) -> usize {
    let mut idx = 0;

    let b_line = line.as_bytes();
    let mut result: usize = 0;

    while idx < line.len() - 1 {
        match b_line[idx] {
            b'\"' => {
                idx += 1;
                continue;
            }
            b'\\' => {
                if b_line[idx + 1] == b'\"' || b_line[idx + 1] == b'\\' {
                    result += 2;
                    idx += 2;
                } else if b_line[idx + 1] == b'x' {
                    result += 1;
                    idx += 4;
                }
            }
            _ => idx += 1,
        }
    }
    println! {"{}", line.len() + 4 + result};
    result + 4
}

fn main() {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("file to be read");

    let r: usize = context
        .split("\n")
        .enumerate()
        .map(|(idx, line)| {
            println!("----{idx}----");
            parse(line)
        })
        .sum();

    println!("{r}");
}
