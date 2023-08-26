use std::collections::HashMap;
use regex::Regex;
use lazy_static::lazy_static;

use advent_2020::{Runner, get_file};

pub struct Q14 {

}
//  001 10X  
//  001 | 100 => 101 
//  101 & 101 => 101 

struct PortSystem {
    memory: HashMap<u64, u64>,
    instructions: Vec<String>
}

impl PortSystem {
    fn new() -> Self {
        let content = get_file("src/input/q14.txt").unwrap();
        let memory = HashMap::new();
        let instructions = content.lines().map(String::from).collect();
        Self {memory, instructions }
    }

    fn decode_address(&mut self) {
        let mut mask: Option<String> = None;
        for ins in self.instructions.iter() {
            let (key,value) = ins.split_once(" = ").unwrap();
            match key {
                "mask" => {
                    mask = Some(value.to_string())
                }
                _ => {
                    if let Some(cmask) = mask.clone() {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"\d+").unwrap(); 
                        }
                        let value = value.parse::<i64>().unwrap();
                        let address = RE.find(key).unwrap().as_str().parse::<u64>().unwrap();
                        // 42
                        let mut addresses = vec![];
                        for bit in cmask.chars().rev() {
                            match bit {
                                'X' => {}
                                '0' => {}
                                '1' => {}
                                _ => panic!("unknown bit pattern")
                            }
                        }
                    } else {
                        panic!("mask does not exist")
                    }
                }
            }
        }
    }
    
    fn sum_masked_values(&mut self) {
        let mut mask: Option<String> = None;
        for ins in self.instructions.iter() {
            let (key,value) = ins.split_once(" = ").unwrap();
            match key {
                "mask" => {
                    mask = Some(value.to_string())
                }
                _ => {
                    if let Some(cmask) = mask.clone() {
                        lazy_static! {
                            static ref RE: Regex = Regex::new(r"\d+").unwrap(); 
                        }
                        let location = RE.find(key).unwrap().as_str().parse::<u64>().unwrap();
                        let mask_unchange = cmask.replace("X", "0");
                        let binary = u64::from_str_radix(&mask_unchange, 2).unwrap();
                        let val = value.parse::<u64>().unwrap();
                        let m = binary | val;
                        let mask_change = cmask.replace("X", "1");
                        let binary = u64::from_str_radix(&mask_change, 2).unwrap();
                        self.memory.insert(location, binary & m);
                    } else {
                        panic!("mask does not exist")
                    }
                }
            }
        }
        println!("sum - {:?}", self.memory.values().sum::<u64>())
    }
}



impl Q14 {
    pub fn new() -> Self {
        Q14 {}
    }

    fn part1(&mut self) {
        let mut sys = PortSystem::new();
        sys.sum_masked_values()
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q14 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q14() {
        assert_eq!(1, 1);
    }
}