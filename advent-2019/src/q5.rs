use advent_2019::{Runner, get_file};


#[derive(Debug)]
struct Program {
    integers: Vec<i32>,
    a_pointer: usize
}

impl Program {
    fn new() -> Self {
        let content = get_file("src/input/q5.txt").unwrap();
        let integers: Vec<i32>= content.split(",").map(|num| num.parse().unwrap()).collect();
        let a_pointer = 0;
        Program {integers, a_pointer}
    }

    // refactoring
    fn param_mode_inputs(&self, param1: i32, param2: i32) {
        // do something
    }
    
    fn run_program(&mut self, mut output:i32) {
        
        loop {
            if self.integers[self.a_pointer] == 99 {
                break
            }
            println!("{:?}", self.integers[self.a_pointer]);
            println!("pointer - {:?}", self.a_pointer);
            let opcode = self.integers[self.a_pointer] %100;
            self.integers[self.a_pointer] /= 100;
            let param_1 = self.integers[self.a_pointer] %10;
            self.integers[self.a_pointer] /= 10;
            let param_2 = self.integers[self.a_pointer] % 10;
            self.integers[self.a_pointer] /= 10;
            println!("{:?} {} {}", opcode, param_1, param_2);
            match opcode  {
                1 => {
                    let input1;
                    let input2;
                    match (param_1, param_2) {
                        (0,1) => {
                            input1 = self.integers[self.integers[self.a_pointer+1] as usize];
                            input2 = self.integers[self.a_pointer + 2];    
                        },
                        (1,0) => {
                            input1 = self.integers[self.a_pointer + 1];
                            input2 = self.integers[self.integers[self.a_pointer+2] as usize]  
                        },
                        (1,1) => {
                            input1 = self.integers[self.a_pointer + 1];
                            input2 = self.integers[self.a_pointer + 2];    
                        },
                        _ => {
                            input1 = self.integers[self.integers[self.a_pointer+1] as usize];
                            input2 = self.integers[self.integers[self.a_pointer+2] as usize];    
                        }
                    }
                    let output_integer = self.integers[self.a_pointer+3];
                    self.integers[output_integer as usize] = input1 + input2;
                    self.a_pointer += 4;
                },
                2 => {
                    let input1;
                    let input2;
                    match (param_1, param_2) {
                        (0,1) => {
                            input1 = self.integers[self.integers[self.a_pointer+1] as usize];
                            input2 = self.integers[self.a_pointer + 2];    
                        },
                        (1,0) => {
                            input1 = self.integers[self.a_pointer + 1];
                            input2 = self.integers[self.integers[self.a_pointer+2] as usize]  
                        },
                        (1,1) => {
                            input1 = self.integers[self.a_pointer + 1];
                            input2 = self.integers[self.a_pointer + 2];    
                        },
                        _ => {
                            input1 = self.integers[self.integers[self.a_pointer+1] as usize];
                            input2 = self.integers[self.integers[self.a_pointer+2] as usize];    
                        }
                    }
                    let output_integer = self.integers[self.a_pointer+3];
                    self.integers[output_integer as usize] = input1 * input2;
                    self.a_pointer += 4;
                },
                3 => {
                    let i = self.integers[self.a_pointer+1] as usize;
                    self.integers[i] = output; 
                    self.a_pointer += 2;
                }
                4 => {
                    output = self.integers[self.integers[self.a_pointer+1] as usize];
                    println!("{:?}", output);
                    self.a_pointer += 2;
                }
                5 => {
                    let param1 = self.integers[self.a_pointer+1];
                    if param1 != 0 {
                        self.a_pointer = self.integers[self.a_pointer+2] as usize;
                    } else {
                        self.a_pointer += 3;
                    }
                }
                6 => {
                    let param1 = self.integers[self.a_pointer+1];
                    if param1 == 0 {
                        self.a_pointer = self.integers[self.a_pointer+2] as usize;
                    } else {
                        self.a_pointer += 3;
                    }
                }
                7 => {
                    let param1 = self.integers[self.a_pointer+1];
                    let param2 = self.integers[self.a_pointer+2];
                    if param1 < param2 {
                        self.a_pointer = self.integers[self.a_pointer+2] as usize;
                        let output_integer = self.integers[self.a_pointer+3];
                        self.integers[output_integer as usize] = 1
                    } else {
                        let output_integer = self.integers[self.a_pointer+3];
                        self.integers[output_integer as usize] = 0                    }
                    self.a_pointer += 4;
                }
                8 => {
                    let param1 = self.integers[self.a_pointer+1];
                    let param2 = self.integers[self.a_pointer+2];
                    if param1 == param2 {
                        self.a_pointer = self.integers[self.a_pointer+2] as usize;
                        let output_integer = self.integers[self.a_pointer+3];
                        self.integers[output_integer as usize] = 1
                    } else {
                        let output_integer = self.integers[self.a_pointer+3];
                        self.integers[output_integer as usize] = 0                    }
                    self.a_pointer += 4;
                }
                _ => {
                    println!("{:?}", opcode);
                    panic!("invalid opcode")
                }
            }
        } 

        println!("{:?}", output);
    }
    
}


pub struct Q5 {

}

impl Q5 {
    pub fn new() -> Self {
        Q5 {}
    }

    fn part1(&mut self) {
        let mut program = Program::new();
        program.run_program(1);

    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q5 {
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
    fn Q5() {
        assert_eq!(1, 1);
    }
}