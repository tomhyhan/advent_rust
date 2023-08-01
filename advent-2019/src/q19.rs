use std::collections::{VecDeque, HashMap, HashSet};

use advent_2019::Runner;

use crate::int_program::{Program, ProgramInputs};

pub struct Q19 {

}

impl Q19 {
    pub fn new() -> Self {
        Q19 {}
    }

    fn part1(&mut self) {
        let mut beams = 0;
        for row in 0..20 {
            let mut s = "".to_string();
            for col in 0..20 {
                let queue = VecDeque::from([row,col]);
                let mut program = ProgramInputs::new(queue);
                let out = program.run_program(0).unwrap(); 
                if out == 1 {
                    beams += 1;
                    s.push('#')
                } else {
                    s.push('.')
                }
            }
            println!("{s}")
        }
        println!("{:?}", beams);
    }

    fn part2(&mut self) {
        let (mut row, mut col) = (5,6);
        let mut start_col = col;
        loop {
            if check_output(row+99,col) && check_output(row,col+99)  {
                println!("found - {row} {col}: {}", row * 10000 + col);
                break
            }
            let out = check_output(row, col+99);
            // println!("{row} {col}");
            // 4660462 10220918
            if out {
                col += 1;
            } else {
                row += 1;
                start_col = if check_output(row, start_col) {start_col} else {start_col+1};
                col = start_col;
            }
        }
    }

}

fn check_output(row:i64, col:i64) -> bool {
    let queue = VecDeque::from([row,col]);
    let mut program = ProgramInputs::new(queue);
    let out = program.run_program(0).unwrap(); 
    if out == 1 {true} else {false}
}

impl Runner for Q19 {
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
    fn Q19() {
        assert_eq!(1, 1);
    }
}
// 918 1022: 10220918
// 1022 918 9181022