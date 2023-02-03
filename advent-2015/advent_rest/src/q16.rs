use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, fs, hash::Hash, ops::IndexMut};

#[derive(Debug)]

struct Sue {
    pub children: Option<i64>,
    pub cats: Option<i64>,
    pub samoyeds: Option<i64>,
    pub pomeranians: Option<i64>,
    pub akitas: Option<i64>,
    pub vizslas: Option<i64>,
    pub goldfish: Option<i64>,
    pub trees: Option<i64>,
    pub cars: Option<i64>,
    pub perfumes: Option<i64>,
}

impl Sue {
    fn chagne_value(&mut self, property: &str, value: i64) {
        match property {
            "children" => self.children = Some(value),
            "cats" => self.cats = Some(value),
            "samoyeds" => self.samoyeds = Some(value),
            "pomeranians" => self.pomeranians = Some(value),
            "akitas" => self.akitas = Some(value),
            "vizslas" => self.vizslas = Some(value),
            "goldfish" => self.goldfish = Some(value),
            "trees" => self.trees = Some(value),
            "cars" => self.cars = Some(value),
            "perfumes" => self.perfumes = Some(value),
            _ => panic!("no such key word exist"),
        }
    }

    fn matches(&self, other: &Sue) -> bool {
        let mut cnt = 0;

        match other {
            val if self.children == other.children => {
                println!("{val:?} is key1");
            }
            _ => (),
        }
        if !self.children.is_none() && self.children == other.children {
            cnt += 1
        }
        if !self.cats.is_none() && self.cats > other.cats {
            cnt += 1
        }
        if !self.samoyeds.is_none() && self.samoyeds == other.samoyeds {
            cnt += 1
        }
        if !self.pomeranians.is_none() && self.pomeranians < other.pomeranians {
            cnt += 1
        }
        if !self.akitas.is_none() && self.akitas == other.akitas {
            cnt += 1
        }
        if !self.vizslas.is_none() && self.vizslas == other.vizslas {
            cnt += 1
        }
        if !self.goldfish.is_none() && self.goldfish < other.goldfish {
            cnt += 1
        }
        if !self.trees.is_none() && self.trees > other.trees {
            cnt += 1
        }
        if !self.cars.is_none() && self.cars == other.cars {
            cnt += 1
        }
        if !self.perfumes.is_none() && self.perfumes == other.perfumes {
            cnt += 1
        }
        println!("{cnt:?}");
        if cnt >= 3 {
            true
        } else {
            false
        }
    }
}

impl Default for Sue {
    fn default() -> Sue {
        Sue {
            children: None,
            cats: None,
            samoyeds: None,
            pomeranians: None,
            akitas: None,
            vizslas: None,
            goldfish: None,
            trees: None,
            cars: None,
            perfumes: None,
        }
    }
}

#[derive(Debug)]
struct Sues {
    info: HashMap<String, Sue>,
}

impl From<String> for Sues {
    fn from(contents: String) -> Self {
        let mut sue_info = Sues {
            info: HashMap::new(),
        };

        contents
            .lines()
            .enumerate()
            .for_each(|(idx, line)| parse(idx, line, &mut sue_info));
        sue_info
    }
}

// Sue 1: goldfish: 6, trees: 9, akitas: 0
fn parse(idx: usize, line: &str, sue_info: &mut Sues) {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"(children|cats|samoyeds|pomeranians|akitas|vizslas|goldfish|trees|cars|perfumes)(?:: )\d+").unwrap();
    }
    let sue_collections: Vec<_> = RE.find_iter(line).map(|n| n.as_str()).collect();
    sue_info.info.insert(
        idx.to_string(),
        Sue {
            ..Default::default()
        },
    );

    sue_collections.iter().for_each(|c| {
        let s: Vec<&str> = c.split(": ").collect();
        sue_info
            .info
            .get_mut(&idx.to_string())
            .unwrap()
            .chagne_value(s[0], s[1].parse::<i64>().unwrap());
    });
    // println!("{:?}", sue_info.info[&idx.to_string()]);
}

pub fn run_code() {
    let path = "q16.txt";
    let contents = fs::read_to_string(path).expect("fail to read a file");

    let sues: Sues = contents.into();
    println!("{:?}", sues.info);

    let real_sue = Sue {
        children: Some(3),
        cats: Some(7),
        samoyeds: Some(2),
        pomeranians: Some(3),
        akitas: None,
        vizslas: None,
        goldfish: Some(5),
        trees: Some(3),
        cars: Some(2),
        perfumes: Some(1),
    };

    let result: Vec<_> = sues
        .info
        .iter()
        .filter(|(key, sue)| sue.matches(&real_sue))
        .collect();

    println!("{result:?}");
}
