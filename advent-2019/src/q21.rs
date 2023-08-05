use std::str::Chars;

use advent_2019::{Runner, get_file};

use crate::int_program::ProgramASCII;


pub struct Q21 {

}
// NOT X Y sets Y to true if X is false; otherwise, it sets Y to false.
// jumps if there is not hall else not jump
impl Q21 {
    pub fn new() -> Self {
        Q21 {}
    }
    // #####..#.########
    fn part1(&mut self) {
        // let instruction = "NOT D J\nWALK\n".to_string();
        // let instruction = "OR C J\nOR A J\nOR B J\nWALK\n".to_string();
        // #####.###########
        // #####...#########
        // #####..#.########
        // #####.#..########
        let instruction = "NOT B J\nNOT C T\nOR T J\nAND D J\nAND H J\nNOT A T\nOR T J\nRUN\n".to_string();
        let mut program_acsii = ProgramASCII::new(instruction);
        let mut msg = "".to_string();
        while let Some(output) = program_acsii.run_program(0) {
            let c = output as u8 as char;
            if c == '\n' {
                if !msg.is_empty() {
                    println!("{:?}", msg);
                    msg.clear();
                }
            } else {
                msg.push(c);
            }
            // println!("{:?}", c)
        }
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q21 {
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
    fn Q21() {
        assert_eq!(1, 1);
    }
}
#[derive(Debug, Clone)]
pub struct Program {
    pub integers: Vec<i64>,
    a_pointer: usize,
    relative_base: usize,
    str_chars: Chars<'static>
}

// 1219070632396864
impl Program {
    pub fn new() -> Self {
        let content = get_file("src/input/q21.txt").unwrap();
        let mut integers = vec![0;100000];
        content.split(",").enumerate().for_each(|(idx, num )| {
            integers[idx] = num.parse().unwrap()
        });
        let a_pointer = 0;
        let relative_base = 0;
        
        let str = "NOT C J\nAND D J\nNOT A T'\nOR T J\nWALK";
        let mut str_chars = str.chars();

        Program {
            integers,
            a_pointer,
            relative_base,
            str_chars
        }
    }
    
    // refactoring
    pub fn param_mode_inputs(&self, param: i64, pos: usize) -> i64 {
        let input;
        match param {
            1 => {input = self.integers[self.a_pointer + pos]}
            2 => {
                input= self.integers[(self.integers[self.a_pointer + pos] + self.relative_base as i64) as usize] }
            0 => {input = self.integers[self.integers[self.a_pointer + pos] as usize]}
            _ => panic!("unknown")
        }    
        input
    }

    pub fn get_output_idx(&self, pointer:usize, param: i64) -> usize {
        let mut idx = self.integers[pointer];
        if param == 2 {
            idx += self.relative_base as i64;
        }
        idx as usize
    }

    pub fn run_program(&mut self, mut output: i64)  -> Option<i64>{
        // further refactoring is possible
        
        loop {
            let tmp = self.integers[self.a_pointer];
            let opcode = self.integers[self.a_pointer] % 100;
            self.integers[self.a_pointer] /= 100; 
            let param1 = self.integers[self.a_pointer] % 10;
            self.integers[self.a_pointer] /= 10;
            let param2 = self.integers[self.a_pointer] % 10;
            self.integers[self.a_pointer] /= 10;
            let param3 = self.integers[self.a_pointer] % 10;
            self.integers[self.a_pointer] /= 10;

            self.integers[self.a_pointer] = tmp;

            match opcode  {
                1 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    let input2 = self.param_mode_inputs(param2, 2);
                    
                    let output_integer = self.get_output_idx(self.a_pointer+3, param3);
                    self.integers[output_integer as usize] = input1 + input2;
                    self.a_pointer += 4;
                }
                2 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    let input2 = self.param_mode_inputs(param2, 2);
                    let output_integer = self.get_output_idx(self.a_pointer+3, param3);
                    self.integers[output_integer as usize] = input1 * input2;
                    self.a_pointer += 4;
                }
                3 => {
                    println!("{:?}", "here");
                    let idx = self.get_output_idx(self.a_pointer+1, param1);
                    self.integers[idx] = (self.str_chars.next().unwrap() as u32) as i64;
                    self.a_pointer += 2;
                }
                4 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    output = input1;
                    self.a_pointer += 2;
                    return Some(output)
                }
                5 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    let input2 = self.param_mode_inputs(param2, 2);
                    if input1 != 0 {
                        self.a_pointer = input2 as usize;
                    } else {
                        self.a_pointer += 3;
                    }
                }
                6 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    let input2 = self.param_mode_inputs(param2, 2);
                    if input1 == 0 {
                        self.a_pointer = input2 as usize;
                    } else {
                        self.a_pointer += 3;
                    }
                }
                7 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    let input2 = self.param_mode_inputs(param2, 2);
                    let output_integer = self.get_output_idx(self.a_pointer+3, param3);                  
                    if input1 < input2 {
                        self.integers[output_integer as usize] = 1
                    } else {
                        self.integers[output_integer as usize] = 0
                    }
                    self.a_pointer += 4;
                }
                8 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    let input2 = self.param_mode_inputs(param2, 2);
                    let output_integer = self.get_output_idx(self.a_pointer+3, param3);         
                    if input1 == input2 {
                        self.integers[output_integer as usize] = 1
                    } else {
                        self.integers[output_integer as usize] = 0
                    }
                    self.a_pointer += 4;
                }
                9 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    self.relative_base = (self.relative_base as i64 + input1) as usize;
                    self.a_pointer += 2;
                }
                99 => return None,
                _ => {
                    panic!("invalid opcode")
                }
            }
        }

        panic!("used up all the values")
    }
}