use std::collections::HashMap;

use crate::{common::my_modules::get_file, Runner};

#[derive(Debug)]
pub struct Q23 {
    computer: Computer,
}

#[derive(Debug)]
enum Instructions {
    Inc(Source),
    Dec(Source),
    Cpy(Source, Source),
    Jnz(Source, Source),
    Tgl(Source),
}

impl Instructions {
    fn parse(line: &str) -> Self {
        let ins: Vec<_> = line.split_whitespace().collect();
        match ins[0] {
            "inc" => Instructions::Inc(Source::parse(ins[1])),
            "dec" => Instructions::Dec(Source::parse(ins[1])),
            "cpy" => Instructions::Cpy(Source::parse(ins[1]), Source::parse(ins[2])),
            "jnz" => Instructions::Jnz(Source::parse(ins[1]), Source::parse(ins[2])),
            "tgl" => Instructions::Tgl(Source::parse(ins[1])),
            _ => panic!("wrong instructions"),
        }
    }
}

#[derive(Debug, Clone)]
enum Source {
    Reg(Register),
    Val(i32),
}

impl Source {
    fn parse(ins: &str) -> Self {
        if let Ok(reg) = Register::parse(ins) {
            Source::Reg(reg)
        } else {
            Source::Val(ins.parse().unwrap())
        }
    }
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Register {
    A,
    B,
    C,
    D,
}

impl Register {
    fn parse(ins: &str) -> Result<Self, ()> {
        match ins {
            "a" => Ok(Self::A),
            "b" => Ok(Self::B),
            "c" => Ok(Self::C),
            "d" => Ok(Self::D),
            _ => Err(()),
        }
    }
}

impl Q23 {
    pub fn new() -> Self {
        let pointer = 0;
        let instructions = get_file("Q23.txt")
            .unwrap()
            .lines()
            .map(|l| Instructions::parse(l))
            .collect();
        let register = HashMap::from([
            (Register::A, 7),
            (Register::B, 0),
            (Register::C, 0),
            (Register::D, 0),
        ]);
        Q23 {
            computer: Computer {
                instructions,
                pointer,
                register,
            },
        }
    }
}

#[derive(Debug)]
struct Computer {
    register: HashMap<Register, i32>,
    pointer: i32,
    instructions: Vec<Instructions>,
}

impl Computer {
    fn toggle(&mut self, toggle_p: usize) {
        match &self.instructions[toggle_p] {
            Instructions::Inc(src) => self.instructions[toggle_p] = Instructions::Dec(src.clone()),
            Instructions::Dec(src) => self.instructions[toggle_p] = Instructions::Inc(src.clone()),
            Instructions::Cpy(src, dest) => {
                self.instructions[toggle_p] = Instructions::Jnz(src.clone(), dest.clone())
            }
            Instructions::Jnz(src, dest) => {
                self.instructions[toggle_p] = Instructions::Cpy(src.clone(), dest.clone())
            }
            Instructions::Tgl(src) => self.instructions[toggle_p] = Instructions::Inc(src.clone()),
            _ => panic!("unknown instruction"),
        }
    }

    fn find_password(&mut self) {
        // println!("{:?}",self.instructions);

        let mut cnt = 0;
        while (self.pointer as usize) < self.instructions.len() {
            match &self.instructions[self.pointer as usize] {
                Instructions::Inc(Source::Reg(reg)) => {
                    *self.register.get_mut(reg).unwrap() += 1;
                }
                Instructions::Inc(Source::Val(_)) => {}
                Instructions::Dec(Source::Reg(reg)) => {
                    *self.register.get_mut(reg).unwrap() -= 1;
                }
                Instructions::Dec(Source::Val(_)) => {}
                Instructions::Cpy(Source::Val(val), Source::Reg(reg)) => {
                    self.register.insert(*reg, *val);
                }
                Instructions::Cpy(_, Source::Val(_)) => {}
                Instructions::Cpy(Source::Reg(reg_src), Source::Reg(reg_dest)) => {
                    let val = self.register.get(&reg_src).unwrap().clone();
                    self.register.insert(*reg_dest, val);
                }
                Instructions::Jnz(condition, jmp) => {
                    let condition = match condition {
                        Source::Reg(reg) => self.register.get(reg).unwrap(),
                        Source::Val(val) => val,
                    };

                    let jmp = match jmp {
                        Source::Reg(reg) => self.register.get(reg).unwrap(),
                        Source::Val(val) => val,
                    };

                    if *condition != 0 {
                        self.pointer += *jmp;
                        continue;
                    }
                }
                Instructions::Tgl(Source::Reg(reg)) => {
                    let reg_val = self.register.get(reg).unwrap().clone();
                    let new_pointer = self.pointer + reg_val;
                    if new_pointer >= 0 && new_pointer < self.instructions.len() as i32 {
                        self.toggle(new_pointer as usize)
                    }
                }
                _ => panic!("asdf"),
            }
            self.pointer += 1;
            if *self.register.get(&Register::D).unwrap() == 0 {
                println!("{:?}", self.register);
            }
            cnt += 1
        }

        println!("{:?}", self.register);
        let mut r = 12;
        for n in (1..=11).rev() {
            r *= n;
        }
        println!("{r:?}");
        println!("{:?}", r + 82 * 73);
        // println!("{:?}", self.instructions);
    }
}

impl Runner for Q23 {
    fn run(&mut self) -> () {
        self.computer.find_password();
    }
}
