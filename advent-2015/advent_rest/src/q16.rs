use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs, hash::Hash};

struct Sue {
    children: Option<i64>,
    cats: Option<i64>,
    samoyeds: Option<i64>,
    pomeranians: Option<i64>,
    akitas: Option<i64>,
    vizslas: Option<i64>,
    goldfish: Option<i64>,
    trees: Option<i64>,
    cars: Option<i64>,
    perfumes: Option<i64>,
}

struct Sues {
    info: HashMap<String, Sue>,
}

impl From<String> for Sues {
    fn from(contents: String) -> Self {
        let mut sue_info = Sues {
            info: HashMap::new(),
        };

        contents.lines().for_each(|line| parse(line, &mut sue_info));
        sue_info
    }
}

// Sue 1: goldfish: 6, trees: 9, akitas: 0
fn parse(line: &str, sue_info: &mut Sues) {}

pub fn run_code() {
    let path = "q16.txt";
    let contents = fs::read_to_string(path).expect("fail to read a file");

    let sues: Sues = contents.into();
}
