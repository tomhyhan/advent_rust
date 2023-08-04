use std::{collections::{HashMap, VecDeque}, io};

use advent_2019::Runner;

use crate::int_program::ProgramASCII;

fn generate_powerset(items: Vec<&str>) -> Vec<Vec<String>> {
    let mut sets = vec![vec![]];
    for item in items.iter() {
        let len = sets.len();
        for i in 0..len {
            let mut new_set = sets[i].clone();
            new_set.push(item.to_string());
            sets.push(new_set);
        }
    }
    sets
}

fn add_command_to_program(program: &mut ProgramASCII, command:&str, item: &str) {
    let s  = format!("{}{}",command, item);
    let mut str_chars:VecDeque<char> = s.trim_end().chars().collect();
    str_chars.push_back('\n');
    program.str_chars.extend(str_chars);
}
pub struct Q25 {

}

impl Q25 {
    pub fn new() -> Self {
        Q25 {}
    }

    fn part1(&mut self) {
        let mut program_acsii = ProgramASCII::new("north\ntake sand\nnorth\nnorth\ntake astrolabe\nsouth\nsouth\nwest\nwest\ntake mutex\neast\neast\nsouth\neast\ntake klein bottle\neast\ntake semiconductor\nwest\nnorth\nnorth\nnorth\ntake dehydrated water\nsouth\nsouth\nsouth\nwest\nwest\nnorth\ntake shell\nsouth\nsouth\nwest\ntake ornament\nwest\nsouth\n".to_string());
        
        let items = vec!["mutex", "ornament", "sand", "astrolabe", "klein bottle", "semiconductor", "dehydrated water", "shell"];

        for item in items.iter() {
            add_command_to_program(&mut program_acsii, "drop ", item)
        }
        
        let command_sets = generate_powerset(items);
        for commands in command_sets.iter() {
            for command in commands.iter() {
                add_command_to_program(&mut program_acsii, "take ", command)
            }    
            add_command_to_program(&mut program_acsii, "", "south");
            for command in commands.iter() {
                add_command_to_program(&mut program_acsii, "drop ", command)
            }  
        }

        let mut msg = "".to_string();
        while let Some(output) = program_acsii.run_program(0) {
            let c = output as u8 as char;
            if c == '\n' {
                if !msg.is_empty() {
                    println!("{:?}", msg);
                } 
                if msg == "Command?".to_string() {
                        // let mut command = "".to_string();
                        // io::stdin().read_line(&mut command).expect("err");
                        // let mut str_chars:VecDeque<char> = command.trim_end().chars().collect();
                        // str_chars.push_back('\n');
                        // program_acsii.str_chars.extend(str_chars);
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