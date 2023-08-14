use std::collections::{HashMap, HashSet};

use advent_2020::{Runner, get_file};


struct BootLoader {
    instructions: Vec<Op>,
    pointer: i32,
    registers: HashMap<String, i32> 
}

impl BootLoader {
    fn new() -> Self {
        let content = get_file("src/input/q8.txt").unwrap();
        let instructions = content.lines().map(|line|{
            let (op, value) = line.split_once(" ").unwrap();
            let value = value.parse::<i32>().unwrap();
            Op::parse(op, value)
        }).collect();
        let pointer = 0;
        let registers = HashMap::from([("acc".to_string(), 0)]);
        Self { instructions, pointer, registers }
    }

    fn operate(&mut self) {
        let current_ins = &self.instructions[self.pointer as usize];
        match current_ins {
            Op::Acc(val) => {
                *self.registers.get_mut("acc").unwrap() += *val;
            },
            Op::Jmp(val) => {
                self.pointer += *val;
                return
            },
            Op::Nop(_) => {}
        }
        self.pointer += 1;
    }

    fn run(&mut self) -> HashSet<i32> {
        let mut seen = HashSet::new();
        while self.pointer < self.instructions.len() as i32 {
            if !seen.insert(self.pointer) {
                break;
            }
            self.operate();
        }
        seen
    }

    fn does_loop_out(&mut self) -> bool {
        let mut seen = HashSet::new();
        while self.pointer < self.instructions.len() as i32 {
            if !seen.insert(self.pointer) {
                break;
            }
            self.operate();
        }
        if self.pointer >= self.instructions.len() as i32 {true} else {false}
    }
}


#[derive(Clone, Copy, Debug)]
enum Op {
    Acc(i32),
    Jmp(i32),
    Nop(i32),
}

impl Op {
    fn parse( op: &str, value: i32) -> Op {
        match op {
            "acc" => Op::Acc(value),
            "jmp" => Op::Jmp(value),
            "nop" => Op::Nop(value),
            _ => panic!("unknown instruction!")
        }

    }
}
pub struct Q8 {

}

impl Q8 {
    pub fn new() -> Self {
        Q8 {}
    }

    fn part1(&mut self) {
        let mut bootloader = BootLoader::new();
        bootloader.run();
    }

    fn part2(&mut self) {
        let mut bootloader = BootLoader::new();
        let p_sets = bootloader.run();

        for pt in p_sets.into_iter() {
            let mut bootloader = BootLoader::new();
            bootloader.instructions[pt as usize]= if let Op::Jmp(val) = bootloader.instructions[pt as usize] {
                Op::Nop(val)
            } else if let Op::Nop(val) = bootloader.instructions[pt as usize] {
                Op::Jmp(val)
            } else { bootloader.instructions[pt as usize]};
            if bootloader.does_loop_out() {
                println!("pt - {:?}", pt);
                println!("{:?}", bootloader.registers);
            }
        }
    }

}

impl Runner for Q8 {
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
    fn Q8() {
        assert_eq!(1, 1);
    }
}