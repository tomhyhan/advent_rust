use std::collections::HashMap;
use std::fmt::format;
use std::fs;

enum Operators {
    AND,
    OR,
    LSHIFT,
    RSHIFT,
    COMPLEMENT,
}

struct Operations {
    isop: bool,
    value: Option<i32>,
    operands: Vec<i32>,
    operator: Option<Operators>,
}

fn parse(line: &str, curcuits: &mut HashMap<char, Operations>) {
    let commands: Vec<&str> = line.split("->").map(|op| op.trim()).collect();
    println!("{commands:?}");
    let operator = &commands[1];
    let operations = &commands[0];

    if operations.parse::<i32>().is_ok() {
        println!("line - {commands:?}");
        let current = Operations {
            isop: false,
            value: Some(operations.parse::<i32>().unwrap()),
            operands: vec![],
            operator: None,
        };
    } else {
        
        let current = Operations
    }
}
fn main() {
    let mut curcuits: HashMap<char, Operations> = HashMap::new();

    let path = "input.txt";
    fs::read_to_string(path)
        .expect("cannot read a file")
        .split("\n")
        .for_each(|line| parse(line, &mut curcuits));

    println!("{}", 0xffff & !123);
    // d -> x or y , x -> 123  and y -> 456

    // println!("{}", isize::from_str_radix("1111111111111111", 2).unwrap());
    // println!("{}", format!("{:0>1$}", (!123), 5));

    // context.split("\n").into_iter().for_each(|line| {
    //     println!("{line:?}");
    // });
}
