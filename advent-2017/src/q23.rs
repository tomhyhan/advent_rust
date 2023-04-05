use std::collections::{HashMap, VecDeque};

use advent_2017::{Runner, get_file};

pub struct Q23 {

}

struct Memory {
    mul:i32
}

impl Memory {
    fn new() -> Self {
        Memory {
            mul:0
        }
    }

    // part 2
    // fn run(&mut self, program: &mut Program, other_program: &mut Program) -> bool {
    //     while program.pointer < program.codes.len() as i64 {
    //         let current_program = &program.codes[program.pointer as usize];
    //         match current_program {
    //             Op::Snd(x) => {
    //                 let x = get_value_or_reg(x, &mut program.registers);
    //                 program.send_value += 1;
    //                 other_program.queue.push_back(x);
    //             },
    //             Op::Set(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut program.registers);
    //                 let x = get_register(x);
    //                 program.registers.insert(x, y);
    //             },
    //             Op::Add(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut program.registers);
    //                 let x = get_register(x);
    //                 program.registers.entry(x).and_modify(|val| *val = *val + y).or_insert(y);
    //             },
    //             Op::Sub(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut program.registers);
    //                 let x = get_register(x);
    //                 program.registers.entry(x).and_modify(|val| *val = *val + y).or_insert(y);
    //             },
    //             Op::Mul(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut program.registers);
    //                 let x = get_register(x);
    //                 program.registers.entry(x).and_modify(|val| *val = *val * y).or_insert(0);
    //             },
    //             Op::Mod(x,y) => {
    //                 let y =  get_value_or_reg(y, &mut program.registers);
    //                 let x = get_register(x);
    //                 program.registers.entry(x).and_modify(|val| *val = *val % y).or_insert(0 % y);  
    //             },
    //             Op::Rcv(x) => {
    //                 let x = get_register(x);
                    
    //                 if program.queue.is_empty() && other_program.queue.is_empty() {
    //                     return false
    //                 } else if program.queue.is_empty() {
    //                     return true
    //                 } else {
    //                     let val = program.queue.pop_front().unwrap();
    //                     program.registers.insert(x, val);
    //                 }
                    
    //             },
    //             Op::Jgz(x,y )=> {
    //                 let x =  get_value_or_reg(x, &mut program.registers);
    //                 if x > 0 {
    //                     let y = get_value_or_reg(y, &mut program.registers);
    //                     program.pointer += y;
    //                     continue
    //                 }
    //             }
    //         }
    //         program.pointer += 1;    
    //     }
    //     false
    // }

    // part 1
    fn run(&mut self,  program: &mut Program) {
        let mut cnt = 0;
        while program.pointer < program.codes.len() as i64 {
            let current_program = &program.codes[program.pointer as usize];
            match current_program {
                Op::Snd(x) => {
                    let x = get_value_or_reg(x, &mut program.registers);
                    // program.sound = x;
                },
                Op::Set(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.insert(x, y);
                    if x == 'f' && y == 0 {
                       break
                    }
                },
                Op::Add(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.entry(x).and_modify(|val| *val = *val + y).or_insert(y);
                },
                Op::Sub(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.entry(x).and_modify(|val| *val = *val - y).or_insert(- y);
                },
                Op::Mul(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.entry(x).and_modify(|val| *val = *val * y).or_insert(0);
                    self.mul += 1;
                },
                Op::Mod(x,y) => {
                    let y =  get_value_or_reg(y, &mut program.registers);
                    let x = get_register(x);
                    program.registers.entry(x).and_modify(|val| *val = *val % y).or_insert(0 % y);  
                },
                Op::Rcv(x) => {
                    let x =  get_value_or_reg(x, &mut program.registers);
                    // if x != 0 {
                    //     println!("{:?}", program.sound);
                    //     break;
                    // }
                },
                Op::Jnz(x,y )=> {
                    let x =  get_value_or_reg(x, &mut program.registers);
                    if x != 0 {
                        let y = get_value_or_reg(y, &mut program.registers);
                        program.pointer += y;
                        continue
                    }
                }
            }
            program.pointer += 1;    
            if program.registers.contains_key(&'g') && *program.registers.get(&'g').unwrap() == 0{
                    println!("{:?}", program.registers);
                } 
            
            cnt  += 1;
            if cnt == 10  {
                break
            }
        }
    }
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
        let content = get_file("q23.txt").unwrap();
        let codes = content.lines().map(|line| {
            let op: Vec<_> = line.split_whitespace().collect();
            match op[0] {
                "snd" => Op::Snd(Instruction::parse(op[1])),
                "set" => Op::Set(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "add" => Op::Add(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "mul" => Op::Mul(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "mod" => Op::Mod(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "rcv" => Op::Rcv(Instruction::parse(op[1])),
                "jnz" => Op::Jnz(Instruction::parse(op[1]), Instruction::parse(op[2])),
                "sub" => Op::Sub(Instruction::parse(op[1]), Instruction::parse(op[2])),
                _ => panic!("wrong Op")
            }
        }).collect();
        let pointer = 0;
        let queue = VecDeque::new();
        let registers = HashMap::from([('a',id)]);
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
    Jnz(Instruction,Instruction),
    Sub(Instruction,Instruction)
}

impl Q23 {
    pub fn new() -> Self {
        Q23 {}
    }

    fn part1(&mut self) {
        let mut program0 = Program::new(1);
        // let mut program1 = Program::new(1);
        let mut memory = Memory::new();
        memory.run(&mut program0);
        
        // loop {
        //     let continue_running = memory.run(&mut program0, &mut program1);
        //     if !continue_running {
        //         break
        //     }
        //     let continue_running = memory.run(&mut program1, &mut program0);
        //     if !continue_running {
        //         break
        //     }
        // }
        println!("{:?}", memory.mul);
        println!("{:?}", program0.registers);
    }

    fn part2(&self) {
        // {'a': 1, 'b': 105700, 'f': 1, 'd': 2, 'c': 122700, 'e': 2, 'g': 2} => 105713 | 105726 ... 
        //   9: f to 1
        //  10: d to 2
        //  11: e to 2
        //  12: g to d => g to 2 | g to 2  |||| d becomes 3 | 4 | 5
        //  13: g x e =>  4 | 6 | 8 ... 3 | 6 | 9 ...
        //  14: g - b => -105696 | -105694
        //  15: jump 2
        //  16: f to 0
        //  17: inc e => 3 | 4 | 5 ...
        //  18: g => 3 | 4 | 5 ...
        //  19: g - b => -105697 | -105696 | -105695  
        //  20: jump - 8
        //  21: inc d => 3 | 4 | 5 | 6 ... 
        //  22: g => 3 | 4 | 5 | 6 ...
        //  23: g - b => -105697 | -105696 | ...
        //  24: jump -13
        //  25: jump 2 if f not 0
        //  26: h += 1
        //  27: g to b
        //  28: g - c => 105700 - 122700
        //  29: jump 2
        //  30: jump 3
        //  31: b - 17
        //  32: jump -23

        // along the way f becomes 0 when g is still - 70000 in 19
        // d = 1 on second iteration

        //  105717 => 325
        let mut h = 0;
        'outer: for val in (105700..=122700).step_by(17) {
            let root = ((val as f32).sqrt()) as i32;
            // find prime
            for n in 2..root {
                if val % n ==0 {
                    h += 1;
                    continue 'outer
                }
            }
        }
        println!("{:?}", h);
    }
}

impl Runner for Q23 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}