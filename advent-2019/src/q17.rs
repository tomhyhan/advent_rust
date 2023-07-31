// very bad code 
use std::{collections::HashMap, str::Chars};

use advent_2019::{Runner, get_file};

pub struct Q17 {

}

struct System {
    map: HashMap<(i64,i64), char>,
    program: Program,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64
}

impl System {
    fn new(program: Program) -> Self{
        let mut map = HashMap::new();
        let min_row = i64::MAX;
        let max_row = i64::MIN;
        let min_col = i64::MAX;
        let max_col = i64::MIN;
        
        Self {map,
            program,
            min_row,
            max_row,
            min_col,
            max_col,}
    }

    // fn set_map(&mut self) {
    //     let mut row = 0;
    //     let mut col = 0;        
    //     while let Some(output) = self.program.run_program(1) {
    //         self.min_row = self.min_row.min(row);
    //         self.max_row = self.max_row.max(row);
    //         self.min_col = self.min_col.min(col);
    //         self.max_col = self.max_col.max(col);
    //         match output {
    //             46 => {
    //                 self.map.insert((row, col), '.');
    //                 col += 1;
    //             }
    //             35 => {
    //                 self.map.insert((row, col), '#');
    //                 col += 1;
    //             }
    //             10 => {
    //                 row +=1;
    //                 col=0
    //             },
    //             94 => {
    //                 self.map.insert((row, col), '^');
    //                 col += 1;
    //             },
    //             _ => {
    //                 println!("{:?}", output);
    //                 panic!("unknown output!!")
    //             }
    //         }
    //     }
    // }
// 77
// 97
// 105
    fn move_robot(&self) {
        let ((mut row,mut col), _) = self.map.iter().find(|(_,&v)|v == '^').unwrap();
        let directions = [(-1,0),(0,1),(1,0),(0,-1)];
        let mut dir = 0;
        let mut paths = vec![];
        loop {
            let (nrow,ncol) = (row + directions[dir].0, col + directions[dir].1);
            if let Some(new_path) = self.map.get(&(nrow,ncol)) {
                if *new_path == '#' {
                    row = nrow;
                    col = ncol;
                    paths.push('1');
                    continue
                }
            } 
            let left = (dir + 3) % 4;
            let right = (dir + 1) % 4;
            let (lrow,lcol,rrow,rcol) = (row + directions[left].0, col + directions[left].1, row + directions[right].0, col + directions[right].1);
            if let Some(new_path) = self.map.get(&(lrow,lcol)) {
                if *new_path == '#' {
                    row = lrow;
                    col = lcol;
                    dir = left;
                    paths.push('L');
                    paths.push('1');
                    continue
                }
            } 
            if let Some(new_path) = self.map.get(&(rrow,rcol)) {
                if *new_path == '#' {
                    row = rrow;
                    col = rcol;
                    dir = right;
                    paths.push('R');
                    paths.push('1');
                    continue
                }
            } 
            break
        }
        let mut string_path = "".to_string();
        let mut moves = 1;
        paths.iter().for_each(|c|{
            match c {
                'L' | 'R' => {
                    if moves > 1 {
                        string_path.push_str(&moves.to_string());
                        string_path.push_str(&" ".to_string());
                        string_path.push_str(&*c.to_string());
                    } else {
                        string_path.push_str(&*c.to_string());
                    }
                    moves = 0;
                }, 
                '1' => moves += 1,
                _ => panic!("err")
            }
        });
        string_path.push(char::from_digit(moves, 10).unwrap());
        println!("{string_path:?}")
        // "L10 R8 L6 R6 L8 L8 R8 L10 R8 L6 R6 R8 L6 L10 L10 L10 R8 L6 R6 L8 L8 R8 R8 L6 L10 L10 L8 L8 R8 R8 L6 L10 L10 L8 L8 R8"
        // "A B A C A B C B C B"
        // A B A C A B C B C B
        // L10 R8 L6 R6
        // L8 L8 R8
        // R8 L6 L10 L10
        // A,B,A,C,A,B,C,B,C,B\nL,10,R,8,L,6,R,6\nL,8,L,8,R,8\nR,8,L,6,L,10,L,10\nn\n
    }

    fn collect_dust(&self, mut program: Program) {
        while let Some(out) = program.run_program(0) {
            println!("{:?}", out);
        }
    }
    
    fn draw_debug(&self) {
        // let rowcol = self.get_row_col();
        for row in self.min_row..=self.max_row {
            let mut s = String::new();
            for col in self.min_col..=self.max_col {
                if let Some(c) = self.map.get(&(row,col)) {
                    match *c {
                        '.' => s.push('.'),
                        '#' => s.push('#'),
                        '^' => s.push('^'),
                        _ => panic!("unknown char")
                    };
                } else {
                    s.push('@')
                }
            }
            println!("{:?}", s);
        }
    } 

    fn sum_of_alignment_parameter(&self) -> i64 {
        let mut total = 0;
        for ((row,col), path) in self.map.iter() {
            if *path == '#' && self.neighbor_is_scaffold(*row,*col) {
                total += row * col;
            }
        }
        total
    }
    
    fn neighbor_is_scaffold(&self,row: i64, col: i64)-> bool {
        for (nrow,ncol) in [(row+1,col),(row-1,col),(row,col+1),(row,col-1)] {
            if let Some(npath) =self.map.get(&(nrow,ncol)) {
                if *npath != '#' {
                    return false
                }
            } else {
                return false
            }
        }
        true
    }
}



impl Q17 {
    pub fn new() -> Self {
        Q17 {}
    }

    fn part1(&mut self) {
        // let program = Program::new();
        // let mut system = System::new(program);
        // system.set_map();
        // system.draw_debug();
        // println!("total - {:?}", system.sum_of_alignment_parameter());
    }

    fn part2(&mut self) {
        let program = Program::new();
        let system = System::new(program);
        let mut program = Program::new();
        program.integers[0] = 2;
        system.collect_dust(program);
    }

}

impl Runner for Q17 {
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
    fn Q17() {
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
        let content = get_file("src/input/q17.txt").unwrap();
        let mut integers = vec![0;100000];
        content.split(",").enumerate().for_each(|(idx, num )| {
            integers[idx] = num.parse().unwrap()
        });
        let a_pointer = 0;
        let relative_base = 0;
        
        let str = "A,B,A,C,A,B,C,B,C,B\nL,10,R,8,L,6,R,6\nL,8,L,8,R,8\nR,8,L,6,L,10,L,10\nn\n";
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