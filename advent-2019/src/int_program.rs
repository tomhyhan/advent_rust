use advent_2019::get_file;


#[derive(Debug)]
pub struct Program {
    pub integers: Vec<i64>,
    a_pointer: usize,
    relative_base: usize
}

// 1219070632396864
impl Program {
    pub fn new() -> Self {
        let content = get_file("src/input/q13.txt").unwrap();
        let mut integers = vec![0;100000];
        content.split(",").enumerate().for_each(|(idx, num )| {
            integers[idx] = num.parse().unwrap()
        });
        let a_pointer = 0;
        let relative_base = 0;
        Program {
            integers,
            a_pointer,
            relative_base
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

    pub fn run_program(&mut self, mut output: i64) -> Option<i64> {
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
                    let idx = self.get_output_idx(self.a_pointer+1, param1);
                    self.integers[idx] = output;
                    self.a_pointer += 2;
                }
                4 => {
                    let input1 = self.param_mode_inputs(param1, 1);
                    // println!("{:?}", input1);
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