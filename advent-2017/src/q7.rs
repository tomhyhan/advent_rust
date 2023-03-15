use std::{collections::HashMap, hash::Hash};

use advent_2017::{Runner, get_file};

pub struct Q7 {
}

struct Disks {
    list: HashMap<String, Disk>
}

impl Disks {
    fn new() -> Self {
        let mut list = HashMap::new();
        let content = get_file("q7.txt").unwrap();

        content.lines().for_each(|line| parse(&mut list, line));
        Disks { list }
    }
}

#[derive(Debug)]
struct Disk {
    weight: i32,
    holding: Vec<String>
}

fn parse(list: &mut HashMap<String, Disk>, line: &str) {
    let (name, rest) = line.split_once(" ").unwrap();
    if line.split_once(" -> ").is_some() {
        let (weight, holding) = rest.split_once(" -> ").unwrap();
        let weight = weight.trim_matches(|c| c=='(' || c ==')').parse::<i32>().unwrap();
        let holding : Vec<String>= holding.split(", ").map(String::from).collect();
        let disk = Disk {
            holding,
            weight
        };
        list.insert(name.to_string(), disk);
    } else {
        let weight = rest.trim_matches(|c| c=='(' || c ==')').parse::<i32>().unwrap();
        let disk = Disk {
            weight,
            holding: Vec::new()
        };
        list.insert(name.to_string(), disk);
    }
}


impl Q7 {
    pub fn new() -> Self {
        Q7 {}
    }

    fn part1(&mut self) -> String{
        let disks = Disks::new();

        for (name, disk) in disks.list.iter() {
            if disk.holding.is_empty() {
                continue
            }

            let cnt = count_disks(disk, &disks);
            if cnt as usize == disks.list.iter().len() {
                return name.clone();
            }
        }

        panic!("no bottom disk exist");
    }

    fn part2(&mut self, bottom:String) {
        let disks = Disks::new();

        let mut weights = HashMap::new();
        let mut one_program = HashMap::new();
        find_branch_weights(&disks, bottom.clone(), &mut weights, &mut one_program);
        println!("{one_program:?}");
    }
}

fn find_branch_weights(disks: &Disks, name:String, weights: &mut HashMap<String, i32>, one_program: &mut HashMap<String, i32>) -> i32{
    let current_disk = disks.list.get(&name).unwrap();

    if one_program.iter().len() > 0 {
        return -1
    }
    if current_disk.holding.is_empty() {
        return current_disk.weight;
    }

    let mut ws = vec![];
    for disk in current_disk.holding.iter(){
        ws.push(find_branch_weights(&disks, disk.clone(), weights, one_program));
    }

    for i in 0..ws.len() {
        let f =  ws.iter().enumerate().find(|(idx, &p)| {
            *idx != i  && p == ws[i]
        });
        if f.is_none() && one_program.is_empty() {
            println!("{ws:?}");
            one_program.insert(current_disk.holding[i].clone(), ws[i]);
            return -1
        }
    }


    let total = ws.iter().sum::<i32>() + current_disk.weight;
    weights.insert(name, total);
    println!("{total}");
    total
}


fn count_disks(disk: &Disk, disks: &Disks) -> i32 {
    let mut cnt = 0 ;

    cnt += 1;

    for disk in disk.holding.iter() {
        cnt += count_disks(disks.list.get(disk).unwrap(), &disks);
    };

    cnt
}


impl Runner for Q7 {
    fn run(&mut self) {
        let name = self.part1();
        println!("{name}");
        self.part2(name)
    }
}