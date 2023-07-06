use advent_2019::{Runner, get_file};
use itertools::Itertools;

#[derive(Debug)]
struct OutputController {
    output1: i64,
    output2: i64,
    output_other: i64,
    is_output1_used: bool,
    is_output2_used: bool,
}

impl OutputController {
    fn new(output1: i64, output2:i64) -> Self {
        let is_output1_used = false; 
        let is_output2_used = false; 
        let output_other = 0;
        Self { output1, output2, is_output1_used, is_output2_used, output_other }
    }

    fn output1_is_used(&mut self) {
        self.is_output1_used = true
    }

    fn output2_is_used(&mut self) {
        self.is_output2_used = true
    }
}

#[derive(Debug)]
struct Program {
    integers: Vec<i64>,
    a_pointer: usize,
    output_controller : OutputController
}

impl Program {
    fn new(output_controller: OutputController) -> Self {
        let content = get_file("src/input/q7.txt").unwrap();
        let integers: Vec<i64> = content.split(",").map(|num| num.parse().unwrap()).collect();
        let a_pointer = 0;
        Program {
            integers,
            a_pointer,
            output_controller
        }
    }

    // refactoring
    fn param_mode_inputs(&self, param1: i64, param2: i64) -> (i64,i64) {
        let input1;
        let input2;
        match (param1, param2) {
            (0,1) => {
                input1 = self.integers[self.integers[self.a_pointer+1] as usize];
                input2 = self.integers[self.a_pointer + 2];    
            },
            (1,0) => {
                input1 = self.integers[self.a_pointer + 1];
                // this is sepcial case for opcode 4
                // too lazy to change code only for opcode 4
                if self.integers[self.a_pointer+2] < self.integers.len() as i64 {
                    input2 = self.integers[self.integers[self.a_pointer+2] as usize]  
                } else {
                    input2 = 0
                }
            },
            (1,1) => {
                input1 = self.integers[self.a_pointer + 1];
                input2 = self.integers[self.a_pointer + 2];    
            },
            _ => {
                input1 = self.integers[self.integers[self.a_pointer+1] as usize];
                if self.integers[self.a_pointer+2] < self.integers.len() as i64 {
                    input2 = self.integers[self.integers[self.a_pointer+2] as usize]  
                } else {
                    input2 = 0
                }
            }
        }    
        (input1, input2)
    }

    fn run_program(&mut self) -> (i64, bool) {
        loop {
            let tmp = self.integers[self.a_pointer];
            let opcode = self.integers[self.a_pointer] %100;
            self.integers[self.a_pointer] /= 100;
            let param1 = self.integers[self.a_pointer] %10;
            self.integers[self.a_pointer] /= 10;
            let param2 = self.integers[self.a_pointer] % 10;
            self.integers[self.a_pointer] /= 10;
            self.integers[self.a_pointer] = tmp;

            match opcode  {
                1 => {
                    let (input1, input2) = self.param_mode_inputs(param1, param2);
                    let output_integer = self.integers[self.a_pointer+3];
                    self.integers[output_integer as usize] = input1 + input2;
                    self.a_pointer += 4;
                }
                2 => {
                    let (input1, input2) = self.param_mode_inputs(param1, param2);
                    let output_integer = self.integers[self.a_pointer+3];
                    self.integers[output_integer as usize] = input1 * input2;
                    self.a_pointer += 4;
                }
                3 => {
                    let i = self.integers[self.a_pointer + 1] as usize;
                    if !self.output_controller.is_output1_used {
                        self.integers[i] = self.output_controller.output1;
                        self.output_controller.output1_is_used();
                    } else if !self.output_controller.is_output2_used{
                        self.integers[i] = self.output_controller.output2;
                        self.output_controller.output2_is_used();
                    } else {
                        self.integers[i] = self.output_controller.output_other;
                    }
                    
                    self.a_pointer += 2;
                }
                4 => {
                    let (input1, _) = self.param_mode_inputs(param1, param2);
                    self.output_controller.output_other = input1;
                    self.a_pointer += 2;
                    return (input1, false)
                }
                5 => {
                    let (input1, input2) = self.param_mode_inputs(param1, param2);
                    if input1 != 0 {
                        self.a_pointer = input2 as usize;
                    } else {
                        self.a_pointer += 3;
                    }
                }
                6 => {
                    let (input1, input2) = self.param_mode_inputs(param1, param2);
                    if input1 == 0 {
                        self.a_pointer = input2 as usize;
                    } else {
                        self.a_pointer += 3;
                    }
                }
                7 => {
                    let (input1, input2) = self.param_mode_inputs(param1, param2);
                    let output_integer = self.integers[self.a_pointer+3];
                    if input1 < input2 {
                        self.integers[output_integer as usize] = 1
                    } else {
                        self.integers[output_integer as usize] = 0
                    }
                    self.a_pointer += 4;
                }
                8 => {
                    let (input1, input2) = self.param_mode_inputs(param1, param2);
                    let output_integer = self.integers[self.a_pointer+3];
                    if input1 == input2 {
                        self.integers[output_integer as usize] = 1
                    } else {
                        self.integers[output_integer as usize] = 0
                    }
                    self.a_pointer += 4;
                }
                99 => {
                    return (self.output_controller.output_other, true)
                },
                _ => {
                    panic!("invalid opcode")
                }
            }
        }

        // println!("{:?}", output.output_other);
        
        // self.output_controller.output_other
    }
}

pub struct Q7 {

}

impl Q7 {
    pub fn new() -> Self {
        Q7 {}
    }

    fn part1(&mut self) {
        let mut max_output = 0;
        for sequence in (5..=9).permutations(5)  {
            let output = 0;
            let sequence = sequence;
            let program_a = Program::new(OutputController::new(sequence[0], output));
            let program_b = Program::new(OutputController::new(sequence[1], output));
            let program_c = Program::new(OutputController::new(sequence[2], output));
            let program_d = Program::new(OutputController::new(sequence[3], output));
            let program_e = Program::new(OutputController::new(sequence[4], output));
            let mut programs = [program_a, program_b, program_c, program_d, program_e];
            let mut i = 0;
            loop {
                let current_program = &mut programs[i%5];

                let (output, state) = current_program.run_program();
                programs[(i+1)%5].output_controller.output2 = output;
                programs[(i+1)%5].output_controller.output_other = output;

                if i % 5 == 4 && state {
                    println!("output - {:?}", output);
                    println!("i - {:?}", i);
                    max_output = max_output.max(output);
                    break
                }
                i += 1;
            }
            // max_output = max_output.max(output);
        }
        println!("{:?}", max_output);
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q7 {
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
    fn Q7() {
        assert_eq!(1, 1);
    }
}