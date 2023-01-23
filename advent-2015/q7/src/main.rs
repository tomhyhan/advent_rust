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

fn parse(line: &str, curcuits: &mut HashMap<String, String>) {
    let commands: Vec<&str> = line.split("->").map(|op| op.trim()).collect();
    // println!("{commands:?}");
    let wire = commands[1].to_string();
    let operations = commands[0].to_string();

    curcuits.insert(wire, operations);
    // if operations.parse::<i32>().is_ok() {
    //     println!("line - {commands:?}");
    //     let current = Operations {
    //         isop: false,
    //         value: Some(operations.parse::<i32>().unwrap()),
    //         operands: vec![],
    //         operator: None,
    //     };
    // } else {

    //     let current = Operations
    // }
}

fn get_value(
    find: String,
    curcuits: &HashMap<String, String>,
    result: &mut HashMap<String, i32>,
) -> i32 {
    println!("{find:?}");
    if find.parse::<i32>().is_ok() {
        return find.parse::<i32>().unwrap();
    }
    let operations = curcuits.get(&find).unwrap();
    let res;

    if !result.contains_key(&find) {
        if operations.contains("AND") {
            let chars: Vec<String> = operations
                .split("AND")
                .map(|op| op.trim().to_string())
                .collect();
            res = get_value(chars[0].to_string(), &curcuits, result)
                & get_value(chars[1].to_string(), &curcuits, result)
        } else if operations.contains("OR") {
            let chars: Vec<String> = operations
                .split("OR")
                .map(|op| op.trim().to_string())
                .collect();
            res = get_value(chars[0].to_string(), &curcuits, result)
                | get_value(chars[1].to_string(), &curcuits, result);
        } else if operations.contains("LSHIFT") {
            let chars: Vec<String> = operations
                .split("LSHIFT")
                .map(|op| op.trim().to_string())
                .collect();
            res = get_value(chars[0].to_string(), &curcuits, result)
                << chars[1].parse::<i32>().unwrap();
        } else if operations.contains("RSHIFT") {
            let chars: Vec<String> = operations
                .split("RSHIFT")
                .map(|op| op.trim().to_string())
                .collect();
            res = get_value(chars[0].to_string(), &curcuits, result)
                >> chars[1].parse::<i32>().unwrap();
        } else if operations.contains("NOT") {
            let chars: Vec<String> = operations
                .split("NOT")
                .map(|op| op.trim().to_string())
                .collect();
            // println!("{chars:?}");
            res = !get_value(chars[1].to_string(), &curcuits, result) & 0xffff;
        } else {
            res = get_value(operations.to_string(), &curcuits, result);
        }
        result.insert(find.clone(), res);
    }
    *result.get(&find).unwrap()
}

fn main() {
    let mut curcuits: HashMap<String, String> = HashMap::new();
    let mut result: HashMap<String, i32> = HashMap::new();
    let path = "input.txt";
    fs::read_to_string(path)
        .expect("cannot read a file")
        .split("\n")
        .for_each(|line| parse(line, &mut curcuits));

    // println!("{}", 0xffff & !123);
    // d -> x or y , x -> 123  and y -> 456

    // println!("{}", isize::from_str_radix("1111111111111111", 2).unwrap());
    // println!("{}", format!("{:0>1$}", (!123), 5));

    // context.split("\n").into_iter().for_each(|line| {
    //     println!("{line:?}");
    // });

    // println!("{:?}", "NOT x".split("NOT").collect::<Vec<_>>());
    // println!("{:?}", "y RSHIFT 2".split("RSHIFT").collect::<Vec<_>>());
    let r = get_value("a".to_string(), &curcuits, &mut result);
    println!("{r:?}");
}
