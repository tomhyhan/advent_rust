use std::{collections::{HashMap, VecDeque}, io};

use advent_2019::Runner;

use crate::int_program::ProgramASCII;



pub struct Q25 {

}

impl Q25 {
    pub fn new() -> Self {
        Q25 {}
    }
    // 3 notrh 2 east 2 west
    fn part1(&mut self) {
        // let s = "".to_string();
        // north - kitchen/sand
        // north/west - arcade/None
        // north/north - Hallway/escape pod
        // north\nwest\nwest\n - Storage/mutex
        // north\nnorth\nnorth - Navigation/astrolabe
        
        // west -  None
        // west/north - Observatory/shell
        // west/south - Sick Bay/giant electromagnet
        
        // east  - scienceLab/kleinbottle
        
            let mut program_acsii = ProgramASCII::new("north take sand north north take astrolabe".to_string());

            let mut msg = "".to_string();
            while let Some(output) = program_acsii.run_program(0) {
                let c = output as u8 as char;
                if c == '\n' {
                    if !msg.is_empty() {
                        println!("{:?}", msg);
                    } 
                    if msg == "Command?".to_string() {
                        let mut command = "".to_string();
                        io::stdin().read_line(&mut command).expect("err");
                        let mut str_chars:VecDeque<char> = command.trim_end().chars().collect();
                        str_chars.push_back('\n');
                        program_acsii.str_chars.extend(str_chars);
                    }
                    msg.clear()
                } else {
                    msg.push(c);
                }
            }
            
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q25 {
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
    fn Q25() {
        assert_eq!(1, 1);
    }
}