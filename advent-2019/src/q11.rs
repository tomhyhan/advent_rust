use std::collections::HashMap;
use lazy_static::lazy_static;
use advent_2019::{Runner, get_file};

const DIRECTIONS: [(i64,i64);4] = [(-1,0),(0,1),(1,0),(0,-1)];

struct PaintingRobot {
    panel: HashMap<(i64,i64), char>,
    pos: (i64,i64),
    current_dir: usize,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64
}

impl PaintingRobot {
    fn new() -> Self{
        let mut panel = HashMap::new();        
        let pos = (0,0);
        let current_dir = 0;
        panel.insert(pos,'#');
        let min_row = i64::MAX;
        let max_row = i64::MIN;
        let min_col = i64::MAX;
        let max_col = i64::MIN;
        Self { panel, pos, current_dir, min_row,
            max_row,
            min_col,
            max_col }
    }

    fn get_input(&mut self) -> i64{
        let c = *self.panel.entry(self.pos).or_insert('.');
        match c {
            '.' => 0,
            '#' => 1,
            _ => panic!("invalid character")
        }
    }

    fn draw_current_position(&mut self, output:i64) {
        match output {
            0 => self.panel.insert(self.pos, '.'),
            1 => self.panel.insert(self.pos, '#'),
            _ => panic!("invalid output")
        };
    }

    fn change_direction(&mut self, output:i64) {
        match output {
            0 => {
                self.current_dir = (self.current_dir + 3) % 4
            },
            1 => {
                self.current_dir = (self.current_dir + 1) % 4
            },
            _ => panic!("invalid output")
        }
        self.move_robot()
    }
    
    fn move_robot(&mut self) {
        let (row,col) = DIRECTIONS[self.current_dir];
        self.min_row = self.min_row.min(self.pos.0);
        self.max_row = self.max_row.max(self.pos.0);
        self.min_col = self.min_col.min(self.pos.1);
        self.max_col = self.max_col.max(self.pos.1);
        self.pos.0 += row;
        self.pos.1 += col;
    }

    fn draw(&self) {
        for row in self.min_row..=self.max_row {
            let mut s = String::new();
            for col in self.min_col..=self.max_col {
                if let Some(c) = self.panel.get(&(row,col)) {
                    if *c == '#' {
                        s.push('#')
                    } else {
                        s.push(' ')
                    }
                } else {
                    s.push(' ')
                }
            }
            println!("{:?}", s);
        }
    }
}

pub struct Q11 {

}
// 0 - if black
// 1 - if white
// 0 - black left
// 1 - white right

#[derive(Debug)]
struct Program {
    integers: Vec<i64>,
    a_pointer: usize,
    relative_base: usize
}

// 1219070632396864
impl Program {
    fn new() -> Self {
        let content = get_file("src/input/q11.txt").unwrap();
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
    fn param_mode_inputs(&self, param: i64, pos: usize) -> i64 {
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

    fn get_output_idx(&self, pointer:usize, param: i64) -> usize {
        let mut idx = self.integers[pointer];
        if param == 2 {
            idx += self.relative_base as i64;
        }
        idx as usize
    }

    fn run_program(&mut self, mut output: i64) -> Option<i64> {
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


impl Q11 {
    pub fn new() -> Self {
        Q11 {}
    }

    fn part1(&mut self) {
        let mut program = Program::new();
        let mut painting_robot = PaintingRobot::new();
        loop {
            let input = painting_robot.get_input();
            let drawing_output = match program.run_program(input) {
                Some(value) => value,
                None => break
            };
            let direction_output = program.run_program(input).unwrap();
            painting_robot.draw_current_position(drawing_output);            
            painting_robot.change_direction(direction_output);            
        }
        // exclude last position
        println!("panels - {:?}", painting_robot.panel.iter().count() - 1);
        painting_robot.draw();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q11 {
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
    fn Q11() {
        assert_eq!(1, 1);
    }
}