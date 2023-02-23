use crate::{common::my_modules::get_file, Runner};

#[derive(Debug)]
pub struct Q12 {
    computer: Computer,
}

impl Q12 {
    pub fn new() -> Self {
        let registers = Registers {
            c: 1,
            ..Registers::default()
        };
        let pointer = 0;
        let instructions = get_file("q12.txt")
            .unwrap()
            .lines()
            .map(|l| l.to_string())
            .collect();

        Q12 {
            computer: Computer {
                instructions,
                pointer,
                registers,
            },
        }
    }
}

enum Sample {
    A,
    B,
}
#[derive(Default, Debug)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
    d: i32,
}

impl Registers {
    fn copy(&mut self, value: &str, to: &str) {
        // printjln!("{value:?}");
        let value = if value.parse::<i32>().is_ok() {
            value.parse::<i32>().unwrap()
        } else {
            match value {
                "a" => self.a,
                "b" => self.b,
                "c" => self.c,
                "d" => self.d,
                _ => panic!("not known register"),
            }
        };
        match to {
            "a" => self.a = value.clone(),
            "b" => self.b = value.clone(),
            "c" => self.c = value.clone(),
            "d" => self.d = value.clone(),
            _ => panic!("not known register"),
        }
    }

    fn inc(&mut self, to: &str) {
        match to {
            "a" => self.a += 1,
            "b" => self.b += 1,
            "c" => self.c += 1,
            "d" => self.d += 1,
            _ => panic!("not known register"),
        }
    }

    fn dec(&mut self, to: &str) {
        match to {
            "a" => self.a -= 1,
            "b" => self.b -= 1,
            "c" => self.c -= 1,
            "d" => self.d -= 1,
            _ => panic!("not known register"),
        }
    }

    fn jump(&mut self, condition: &str, jump: &str) -> i32 {
        let jump = jump.parse::<i32>().unwrap();
        // println!("{condition:?}");
        match condition {
            "a" => {
                if self.a != 0 {
                    return jump;
                }
            }
            "b" => {
                if self.b != 0 {
                    return jump;
                }
            }
            "c" => {
                if self.c != 0 {
                    return jump;
                }
            }
            "d" => {
                if self.d != 0 {
                    return jump;
                }
            }
            _ => {
                if condition.parse::<i32>().unwrap() != 0 {
                    return jump;
                }
            }
        }
        0
    }
}

#[derive(Debug)]
struct Computer {
    registers: Registers,
    pointer: i32,
    instructions: Vec<String>,
}

impl Computer {
    fn find_password(&mut self) {
        println!("{}", Sample::A as usize);
        // while (self.pointer as usize) < self.instructions.len() {
        //     let space = &self.instructions[self.pointer as usize];
        //     let (address, instruction) = space.split_once(" ").unwrap();

        //     match address {
        //         "cpy" => {
        //             let (value, to) = instruction.split_once(" ").unwrap();
        //             self.registers.copy(value, to);
        //         }
        //         "inc" => self.registers.inc(instruction),
        //         "dec" => self.registers.dec(instruction),
        //         "jnz" => {
        //             let (condition, jump) = instruction.split_once(" ").unwrap();
        //             let jump = self.registers.jump(condition, jump);
        //             if jump != 0 {
        //                 // println!("{:?}", self.pointer);
        //                 // println!("{:?}", self.registers);
        //                 self.pointer += jump;
        //                 continue;
        //             }
        //         }
        //         _ => panic!("unknown command!"),
        //     }
        //     // println!("{:?}", self.pointer);
        //     self.pointer += 1;
        // }
        // println!("{:?}", self.registers);
    }
}

impl Runner for Q12 {
    fn run(&mut self) -> () {
        self.computer.find_password();
    }
}
