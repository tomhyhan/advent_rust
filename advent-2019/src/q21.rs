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
