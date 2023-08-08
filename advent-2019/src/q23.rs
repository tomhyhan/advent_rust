use std::{collections::{VecDeque, HashSet}, cell::RefCell};

use advent_2019::Runner;

use crate::int_program::{Program, ProgramASCII, ProgramInputs};

pub struct Q23 {

}

impl Q23 {
    pub fn new() -> Self {
        Q23 {}
    }

    fn part1(&mut self) {
        let mut programs = vec![];
        for id in 0..50 {
            programs.push(ProgramInputs::new(VecDeque::from(vec![id])));
        }
        let mut program = ProgramInputs::new(VecDeque::from(vec![0]));
        // println!("{:?}", program.run_program(0));
        let mut idle = 0;
        let mut nat = vec![];
        let mut seen = HashSet::new();
        loop {  
    
            for i in 0..programs.len() {
                let address = match programs[i].run_program(0) {
                    Some(val) => val,
                    None => {
                        if programs[i].idle >= 10 {
                            idle += 1;  
                        }                         
                        // println!("{i} {}", programs[i].idle);
                        continue
                    }
                };
                // println!("address: {address}");
                let X = programs[i].run_program(0).unwrap();
                let Y = programs[i].run_program(0).unwrap();
                if address == 255 {
                    nat.push((X,Y));
                    continue
                }
                programs[address as usize].inputs.push_back(X);
                programs[address as usize].inputs.push_back(Y);
            }
            if idle == 50 {

                let (x,y) = nat.last().unwrap(); 
                programs[0].inputs.push_back(*x);
                programs[0].inputs.push_back(*y);
                if !seen.insert(*y) {
                    println!("{:?}", y);
                    break
                }
                for program in programs.iter_mut() {
                    program.idle = 0;
                }
            }
            idle = 0;
        }
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q23 {
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
    fn Q23() {
        assert_eq!(1, 1);
    }
}