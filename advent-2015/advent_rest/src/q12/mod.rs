use regex::Regex;
use serde_json::{json, Result, Value};
use std::{collections::HashMap, fs};

// .map(|line| line.to_string())

//  -> Vec<String>
fn parse() -> Vec<String> {
    let path = "q12.txt";
    let context = fs::read_to_string(path).expect("cannot read a file");
    context.lines().map(|line| line.to_string()).collect()
}

fn deserialize(line: &str) -> Value {
    let data = line;

    let v: Value = serde_json::from_str(data).unwrap();
    // println!("{v:?}");

    v
}

// {"a":[-1,1]} and [-1,{"a":1}]
// [[[3]]] and {"a":{"b":4},"c":-1}
// [1,2,3] and {"a":2,"b":4}

fn get_sum(line: &Value) -> i64 {
    let mut result = 0;

    if line.is_object() {
        println!("this is object");
        // line.as_object().unwrap().iter().for_each(|(key, val)| {
        //     if val.is_number() {
        //         result += val.as_i64().unwrap();
        //     } else if val.is_string() {
        //         if val.as_str().unwrap() == "red" {
        //             return;
        //         }
        //     } else {
        //         result += get_sum(val)
        //     }
        // })
        for (key, val) in line.as_object().unwrap() {
            // println!("{key:?}: {value:?}")
            if val.is_number() {
                result += val.as_i64().unwrap();
            } else if val.is_string() {
                if val.as_str().unwrap() == "red" {
                    return 0;
                }
            } else {
                result += get_sum(val)
            }
        }
    } else if line.is_array() {
        println!("this is array");
        for val in line.as_array().unwrap() {
            if val.is_number() {
                result += val.as_i64().unwrap();
            } else {
                result += get_sum(val)
            }
        }
    }

    println!("{result}");
    result
}

pub fn run_code() {
    let ds: Vec<_> = parse().iter().map(|line| deserialize(line)).collect();
    // Some JSON input data as a &str. Maybe this comes from the user.
    println!("{ds:?}");

    let result: Vec<_> = ds.iter().map(|d| get_sum(d)).collect();
    println!("{result:?}");
    // 51173
}
