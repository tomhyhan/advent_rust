use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}};

use crate::{common::my_modules::get_file, Runner};

#[derive(Debug)]
pub struct Q25 {
    computer: Computer,
}

#[derive(Debug, Clone)]
enum Instructions {
    Inc(Source),
    Dec(Source),
    Cpy(Source, Source),
    Jnz(Source, Source),
    Tgl(Source),
    Out(Source)
}

// impl PartialEq for Instructions {
//     fn eq(&self, other: &Self) -> bool {
//         true
//     }
// }

impl Instructions {
    fn parse(line: &str) -> Self {
        let ins: Vec<_> = line.split_whitespace().collect();
        match ins[0] {
            "inc" => Instructions::Inc(Source::parse(ins[1])),
            "dec" => Instructions::Dec(Source::parse(ins[1])),
            "cpy" => Instructions::Cpy(Source::parse(ins[1]), Source::parse(ins[2])),
            "jnz" => Instructions::Jnz(Source::parse(ins[1]), Source::parse(ins[2])),
            "tgl" => Instructions::Tgl(Source::parse(ins[1])),
            "out" => Instructions::Out(Source::parse(ins[1])),
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

impl Q25 {
    pub fn new() -> Self {
        let pointer = 0;
    let instructions = get_file("q25.txt")
            .unwrap()
            .lines()
            .map(|l| Instructions::parse(l))
            .collect();
        let register = HashMap::from([
            (Register::A, 0),
            (Register::B, 0),
            (Register::C, 0),
            (Register::D, 0),
        ]);
    Q25 {
            computer: Computer {
                instructions,
                pointer,
                register,
            },
        }
    }
}

#[derive(Debug, Clone)]
struct Computer {
    register: HashMap<Register, i32>,
    pointer: i32,
    instructions: Vec<Instructions>,
}

impl Hash for Computer {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.register.iter().for_each(|r| r.1.hash(state));
        self.pointer.hash(state);
    }
}

impl PartialEq  for Computer {
    fn eq(&self, other: &Self) -> bool {
        self.pointer == other.pointer && self.register == other.register
    }
}

impl Eq for Computer {
    
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
        let clock = 

        for c in 0..300 {
            self.register = HashMap::from([
                (Register::A, c),
                (Register::B, 0),
                (Register::C, 0),
                (Register::D, 0),
            ]);
            self.pointer = 0;
            let mut next_bit = 1; 
            let mut output = vec![];
            let mut bit_tracker = HashSet::new();
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
                    Instructions::Out(Source::Reg(reg)) => {
                        let reg_val = self.register.get(reg).unwrap().clone();
                        output.push(reg_val);

                        if next_bit == reg_val {
                            break
                        }
                        next_bit = reg_val;
                    }
                    _ => panic!("asdf"),
                }
                if !bit_tracker.insert(self.clone()) {
                    println!("{:?}", c);
                    break
                }
                // if output.len() >= 20 {
                // }
                self.pointer += 1;
            }
        };
        

    }
}

impl Runner for Q25 {
    fn run(&mut self) -> () {
        self.computer.find_password();
    }
}
