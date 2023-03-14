use advent_2017::{Runner, get_file};

pub struct Q5 {

}

struct Memory {
    instructions: Vec<Op>,
    pointer: i32
}

impl Memory {
    fn new() -> Self {
        let content = get_file("q5.txt").unwrap();
        let instructions = content.lines().map(|num|Op::Jump(num.to_string().parse().unwrap())).collect();
        Memory { instructions, pointer: 0 }
    }
}

enum Op {
    Jump(i32)
}

impl Q5 {
    pub fn new() -> Self {
        Q5 {}
    }

    fn part1(&mut self){
        let mut memory = Memory::new();
        let mut steps = 0;
        while memory.pointer >= 0 && memory.pointer < memory.instructions.len().try_into().unwrap() {
            match memory.instructions[memory.pointer as usize] {
                Op::Jump(ref mut val) => {
                    memory.pointer += *val;
                    if *val >= 3 {
                        *val -= 1;
                    } else {
                        *val += 1;
                    }
                }
            }
            steps += 1;
        }
        println!("{:?}", steps);
    }
}

impl Runner for Q5 {
    fn run(&mut self) {
        self.part1();
    }
}