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
                        let value = value.parse::<u64>().unwrap();
                        let address = RE.find(key).unwrap().as_str().parse::<u64>().unwrap();
                        let mut addresses = vec![];
                        self.change_address(address,cmask.chars().collect(), &mut addresses, 0, 0 );
                        for address in addresses.iter() {
                            self.memory.insert(*address, value);
                        }
                    } else {
                        panic!("mask does not exist")
                    }
                }
            }
        }
        println!("sum - {:?}", self.memory.values().sum::<u64>())
    }
    
    fn change_address(&self, address: u64, mask: Vec<char>, addresses: &mut Vec<u64>, idx: usize, addr_so_far: u64) {
        if let Some(c_mask) = mask.last() {
            match c_mask {
                '0' => {
                    self.change_address(address, mask[..mask.len()-1].to_vec(), addresses, idx + 1, addr_so_far + (address & (1 << idx)))
                }
                '1' => {
                    self.change_address(address, mask[..mask.len()-1].to_vec(), addresses, idx + 1, addr_so_far + (1 << idx))
                }
                'X' => {
                    self.change_address(address, mask[..mask.len()-1].to_vec(), addresses, idx + 1, addr_so_far + (0 << idx));
                    self.change_address(address, mask[..mask.len()-1].to_vec(), addresses, idx + 1, addr_so_far + (1 << idx));
                }
                _ => unreachable!()
            }
        } else {
            addresses.push(addr_so_far)
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
        let mut sys = PortSystem::new();
        sys.decode_address(); 
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