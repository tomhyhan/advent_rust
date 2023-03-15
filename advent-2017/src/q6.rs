use std::collections::HashSet;

use advent_2017::{Runner, get_file};


struct Memory {
    addresses: Vec<i32>
}

impl Memory {
    fn new() -> Self {
        let content = get_file("q6.txt").unwrap();
        let addresses = content.split_whitespace().map(|num| num.parse().unwrap()).collect();
        Memory { addresses } 
    }

}
pub struct Q6 {
}


fn loop_memory(memory: &mut Memory) {
    let mut cycle = 0;
    let mut state = HashSet::new();
    loop {
        let (mut pointer, &(mut allocation)) = find_largest_block(&memory.addresses);
        pointer = memory.addresses.len() - pointer - 1;
        memory.addresses[pointer] = 0;
        while allocation > 0 {
            pointer = (pointer + 1) % memory.addresses.len();
            memory.addresses[pointer] += 1;
            allocation -= 1
        }
        cycle += 1;
        if !state.insert(memory.addresses.clone()) {
            break
        }
    }

    println!("{cycle:?}");
}
impl Q6 {
    pub fn new() -> Self {
        Q6 {}
    }

    fn part1(&mut self) -> Memory{
        let mut memory = Memory::new();
        loop_memory(&mut memory);
        memory
    }

    fn part2(&self, mut memory: Memory) {
        loop_memory(&mut memory)
    }
}

fn find_largest_block(addresses: &Vec<i32>) -> (usize, &i32)  {
    addresses.iter().rev().enumerate().max_by(|x,y|x.1.cmp(y.1)).unwrap()
}

impl Runner for Q6 {
    fn run(&mut self) {
        let m = self.part1();
        self.part2(m);
    }
}