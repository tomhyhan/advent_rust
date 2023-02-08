use core::panic;
use std::{collections::HashMap, fs, hash::Hash};

#[derive(Debug)]
struct Register {
    a: usize,
    b: usize,
}

#[derive(Debug)]
enum Instructions {
    hlf { r: String },
    tpl { r: String },
    inc { r: String },
    jmp { offset: i32 },
    jie { r: String, offset: i32 },
    jio { r: String, offset: i32 },
}

#[derive(Debug)]
struct Address {
    info: HashMap<usize, Instructions>,
}

impl From<String> for Address {
    fn from(content: String) -> Self {
        let mut info = HashMap::new();
        content.lines().enumerate().for_each(|(address, line)| {
            let mut line = line.split(" ").map(|w| w.trim_end_matches(','));
            let ins = line.next().unwrap();
            match ins {
                "inc" => {
                    info.insert(
                        address,
                        Instructions::inc {
                            r: line.next().unwrap().to_string(),
                        },
                    );
                }
                "tpl" => {
                    info.insert(
                        address,
                        Instructions::tpl {
                            r: line.next().unwrap().to_string(),
                        },
                    );
                }
                "hlf" => {
                    info.insert(
                        address,
                        Instructions::hlf {
                            r: line.next().unwrap().to_string(),
                        },
                    );
                }
                "jio" => {
                    info.insert(
                        address,
                        Instructions::jio {
                            r: line.next().unwrap().to_string(),
                            offset: line.next().unwrap().parse::<i32>().unwrap(),
                        },
                    );
                }
                "jie" => {
                    info.insert(
                        address,
                        Instructions::jie {
                            r: line.next().unwrap().to_string(),
                            offset: line.next().unwrap().parse::<i32>().unwrap(),
                        },
                    );
                }
                "jmp" => {
                    info.insert(
                        address,
                        Instructions::jmp {
                            offset: line.next().unwrap().parse::<i32>().unwrap(),
                        },
                    );
                }
                _ => panic!("not a know instruction"),
            }
        });

        Address { info }
    }
}

fn complie(addresses: &Address) {
    let mut register = Register { a: 1, b: 0 };
    let mut pointer: i32 = 0;
    let max_address = &addresses.info.keys().max().unwrap().to_owned();
    // println!("{max_address:?}")
    while &(pointer as usize) <= max_address {
        // execute value of pointer
        let instruction = addresses.info.get(&(pointer as usize)).unwrap();
        println!("{instruction:?}");
        match instruction {
            Instructions::hlf { r } => {
                if r == &"a".to_string() {
                    register.a = register.a / 2;
                } else if let r = "b".to_string() {
                    register.b = register.b / 2;
                }
            }
            Instructions::tpl { r } => {
                if r == &"a".to_string() {
                    register.a = register.a * 3;
                } else if r == &"b".to_string() {
                    register.b = register.b * 3;
                }
            }
            Instructions::inc { r } => {
                if r == &"a".to_string() {
                    register.a += 1;
                } else if r == &"b".to_string() {
                    register.b += 1;
                }
            }
            Instructions::jmp { offset } => {
                pointer += offset;
                continue;
            }
            Instructions::jio { r, offset } => {
                if r == &"a".to_string() {
                    if register.a == 1 {
                        pointer += offset;
                        continue;
                    }
                } else if r == &"b".to_string() {
                    if register.b == 1 {
                        pointer += offset;
                        continue;
                    }
                }
            }
            Instructions::jie { r, offset } => {
                if r == &"a".to_string() {
                    if register.a % 2 == 0 {
                        pointer += offset;
                        continue;
                    }
                } else if r == &"b".to_string() {
                    if register.b % 2 == 0 {
                        pointer += offset;
                        continue;
                    }
                }
            }
            _ => panic!("unkown instructions"),
        };
        pointer += 1;
        // if pointer == 3 {
        //     break;
        // }
    }
    println!("{pointer:?}");
    println!("{register:?}");
}
fn main() {
    let path = "q23.txt";
    let content = fs::read_to_string(path).expect("fail to read a file");

    let addresses: Address = content.into();

    // println!("{:?}", instructions)

    complie(&addresses);
}
