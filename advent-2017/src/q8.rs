use std::collections::HashMap;

use advent_2017::{Runner, get_file};

pub struct Q8 {

}

struct Memory {
    addresses: Vec<String>,
    registers: HashMap<String, i32>
}

impl Memory {
    fn new() -> Self{
        let content = get_file("q8.txt").unwrap();
        let addresses: Vec<_> = content.lines().map(String::from).collect();
        let mut registers = HashMap::new();

        addresses.iter().for_each(|line| {
            let (register, _) = line.split_once(" ").unwrap();
            let register = register.to_string();
            registers.insert(register, 0);
        });

        Memory { addresses, registers }
    }
}

impl Q8 {
    pub fn new() -> Self {
        Q8 {}
    }

    fn part1(&mut self){
        let mut memory = Memory::new();
        let mut highest = 0;
        memory.addresses.iter().for_each(|address| {
            let ins: Vec<_> = address.split(" ").collect();
            let r_change = ins[0].to_string();
            let logic = ins[1];
            let val = ins[2].parse::<i32>().unwrap();
            let condition_r = ins[4].to_string();
            let condition_cmp = ins[5];
            let condition_val = ins[6].parse::<i32>().unwrap();

            // if !memory.registers.contains_key(&condition_r) {
            //     println!("not contained")
            // }

            // println!("{val:?}");
            let mut condition = false;
            match condition_cmp {
                ">" => {if *memory.registers.get(&condition_r).unwrap() > condition_val {condition = true};},
                "<" => {if *memory.registers.get(&condition_r).unwrap() < condition_val {condition = true};},
                "<=" => {if *memory.registers.get(&condition_r).unwrap() <= condition_val {condition = true};},
                ">=" => {if *memory.registers.get(&condition_r).unwrap() >= condition_val {condition = true};},
                "==" => {if *memory.registers.get(&condition_r).unwrap() == condition_val {condition = true};},
                "!=" => {if *memory.registers.get(&condition_r).unwrap() != condition_val {condition = true};},
                _ => panic!("unknown cmp operator!")
            }

            if condition {
                match logic {
                    "inc" => {
                        *memory.registers.get_mut(&r_change).unwrap() += val;
                        
                    },
                    "dec" => {
   
                            *memory.registers.get_mut(&r_change).unwrap() -= val;
                    },
                    _ => panic!("invalid logic unit")
                }
            }
            highest = highest.max(*memory.registers.get(&r_change).unwrap());
        });
        println!("{:#?}", memory.registers);
        println!("{:#?}", memory.registers.iter().max_by_key(|(_,v)|**v));
        println!("{highest:?}");
    }
}

impl Runner for Q8 {
    fn run(&mut self) {
        self.part1();
    }
}