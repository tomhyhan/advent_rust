use std::collections::{HashMap, VecDeque};

use advent_2017::{Runner, get_file};

pub struct Q18 {

}

struct Memory {

}

impl Memory {
    fn new() -> Self {
        Memory {}
    }

    // part 2
    fn run(&mut self, program: &mut Program, other_program: &mut Program) -> bool {
        while program.pointer < program.codes.len() as i64 {
            let current_program = &program.codes[program.pointer as usize];
            match current_program {
                Op::Snd(x) => {
                    let x = get_value_or_reg(x, &mut program.registers);
                    program.send_value += 1;
                    other_program.queue.push_back(x);
                },
                Op::Set(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.insert(x, y);
                },
                Op::Add(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.entry(x).and_modify(|val| *val = *val + y).or_insert(y);
                },
                Op::Mul(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.entry(x).and_modify(|val| *val = *val * y).or_insert(0);
                },
                Op::Mod(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.entry(x).and_modify(|val| *val = *val % y).or_insert(0 % y);  
                },
                Op::Rcv(x) => {
                    let x = get_register(x);
                    
                    if program.queue.is_empty() && other_program.queue.is_empty() {
                        return false
                    } else if program.queue.is_empty() {
                        return true
                    } else {
                        let val = program.queue.pop_front().unwrap();
                        program.registers.insert(x, val);
                    }
                    
                },
                Op::Jgz(x,y )=> {
                    let x =  get_value_or_reg(x, &mut program.registers);
                    if x > 0 {
                        let y = get_value_or_reg(y, &mut program.registers);
                        program.pointer += y;
                        continue
                    }
                }
            }
            program.pointer += 1;    
        }
        false
    }

    // part 1
    // fn run(&mut self, program: Program) {
    //     while self.pointer < program.codes.len() as i64 {
    //         let current_program = &program.codes[self.pointer as usize];
    //         match current_program {
    //             Op::Snd(x) => {
    //                 let x = get_value_or_reg(x, &mut self.registers);
    //                 self.sound = x;
    //             },
    //             Op::Set(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut self.registers);
    //                 let x = get_register(x);
    //                 self.registers.insert(x, y);
    //             },
    //             Op::Add(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut self.registers);
    //                 let x = get_register(x);
    //                 self.registers.entry(x).and_modify(|val| *val = *val + y).or_insert(y);
    //             },
    //             Op::Mul(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut self.registers);
    //                 let x = get_register(x);
    //                 self.registers.entry(x).and_modify(|val| *val = *val * y).or_insert(0);
    //             },
    //             Op::Mod(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut self.registers);
    //                 let x = get_register(x);
    //                 self.registers.entry(x).and_modify(|val| *val = *val % y).or_insert(0 % y);  
    //             },
    //             Op::Rcv(x) => {
    //                 let x =  get_value_or_reg(x, &mut self.registers);
    //                 if x != 0 {
    //                     println!("{:?}", self.sound);
    //                     break;
    //                 }
    //             },
    //             Op::Jgz(x,y )=> {
    //                 let x =  get_value_or_reg(x, &mut self.registers);
    //                 if x > 0 {
    //                     let y = get_value_or_reg(y, &mut self.registers);
    //                     self.pointer += y;
    //                     continue
    //                 }
    //             }
    //         }
    //         self.pointer += 1;    
    //     }
    // }
}

fn get_register(instruction: &Instruction) -> char {
    match instruction {
        Instruction::Register(reg) => *reg,
        _ => panic!("should be a register")
    }
}

fn get_value_or_reg(instruction: &Instruction, registers: &mut HashMap<char, i64>) -> i64 {
    match instruction {
        Instruction::Value(val) => *val,
        Instruction::Register(reg) => *registers.entry(*reg).or_insert(0)
    }
}

struct Program {
    codes: Vec<Op>,
    pointer: i64,
    queue: VecDeque<i64>,
    registers: HashMap<char, i64>,
    send_value: i64
}

impl Program {
    fn new(id:i64) -> Self {
        let content = get_file("q18.txt").unwrap();
        let codes = content.lines().map(|line| {
            let op: Vec<_> = line.split_whitespace().collect();
            match op[0] {
                "snd" => Op::Snd(Instruction::parse(op[1])),
                "set" => Op::Set(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "add" => Op::Add(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "mul" => Op::Mul(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "mod" => Op::Mod(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "rcv" => Op::Rcv(Instruction::parse(op[1])),
                "jgz" => Op::Jgz(Instruction::parse(op[1]), Instruction::parse(op[2])),
                _ => panic!("wrong Op")
            }
        }).collect();
        let pointer = 0;
        let queue = VecDeque::new();
        let registers = HashMap::from([('p',id)]);
        let send_value = 0;
        Program { codes, send_value ,pointer,queue,registers }
    }
}

#[derive(Debug)]
enum Instruction {
    Register(char),
    Value(i64),
}

impl Instruction {
    fn parse(op: &str) -> Instruction{
        if op.parse::<i64>().is_ok() {
            Instruction::Value(op.parse::<i64>().unwrap())
        } else {
            Instruction::Register(op.chars().next().unwrap())
        }
    }
}

#[derive(Debug)]
enum Op {
    Snd(Instruction),
    Set(Instruction,Instruction),
    Add(Instruction,Instruction),
    Mul(Instruction,Instruction),
    Mod(Instruction,Instruction),
    Rcv(Instruction),
    Jgz(Instruction,Instruction)
}

impl Q18 {
    pub fn new() -> Self {
        Q18 {}
    }

    fn part1(&mut self) {
        let mut program0 = Program::new(0);
        let mut program1 = Program::new(1);
        let mut memory = Memory::new();

        loop {
            let continue_running = memory.run(&mut program0, &mut program1);
            if !continue_running {
                break
            }
            let continue_running = memory.run(&mut program1, &mut program0);
            if !continue_running {
                break
            }
        }
        println!("{:?}", program1.send_value);
    }
}

impl Runner for Q18 {
    fn run(&mut self) {
        self.part1();
    }
}