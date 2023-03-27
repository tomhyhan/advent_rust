use std::collections::HashSet;

use advent_2017::{Runner, get_file};

pub struct Q16 {
    
}

struct Program {
    instructions: Vec<Dance>
}

impl Program {
    fn new() -> Self {
        let content = get_file("q16.txt").unwrap();
        let instructions = content.split(',').map(|ins|{
            match &ins[0..1] {
                "s" => Dance::Spin(ins[1..].parse::<i32>().unwrap()),
                "x" => {
                    let (left, right) = ins[1..].split_once("/").unwrap();
                    Dance::Exchange(left.parse::<i32>().unwrap(), right.parse::<i32>().unwrap())
                },
                "p" => {
                    let (left, right) = ins[1..].split_once("/").unwrap();
                    Dance::Partner(left.chars().next().unwrap(), right.chars().next().unwrap())
                },
                _ => panic!("unknown instruction")
            }
        }).collect();
        Program { instructions }
    }
}

#[derive(Debug)]
enum Dance {
    Spin(i32),
    Exchange(i32,i32),
    Partner(char,char),
}

impl Dance {
    fn moves(&self, programs: &mut Vec<char>)  {
        match self {
            Dance::Spin(x) => {
                programs.rotate_right(*x as usize);
            },
            Dance::Exchange(a, b) => {
                programs.swap(*a as usize,*b as usize);
            },
            Dance::Partner(a, b) => {
                let a = programs.iter().position(|p| p == a ).unwrap();
                let b = programs.iter().position(|p| p == b ).unwrap();
                programs.swap(a, b);
            },
        }
    }
}

impl Q16 {
    pub fn new() -> Self {
        Q16 { }
    }

    fn part1(&mut self) -> String {
        let program = Program::new();
        // println!("{:?}", program.instructions);
        let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
        
        program.instructions.iter().for_each(|dance| {
            dance.moves(&mut programs)
        });

        programs.iter().collect()
    }

    fn part2(&self) {
        let program = Program::new();
        let mut programs: Vec<char> = "abcdefghijklmnop".chars().collect();
        let mut cnt = 0;
        let mut ps = HashSet::new();
        loop {
            program.instructions.iter().for_each(|dance| {
                dance.moves(&mut programs)
            });
            if !ps.insert(programs.iter().collect::<String>()) {    
                println!("{cnt}");
                break
            }
            if cnt == 15 {
                println!("{:?}", programs.iter().collect::<String>());
                break
            }
            cnt += 1;
        };
        println!("{}", 1000000000 % 48)
    }
}

impl Runner for Q16 {
    fn run(&mut self) {
        // ekmpglfdjicbhano
        // println!("{:?}", self.part1());
        self.part2();
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn dance_program() {
        let program = Program::new();
        let mut programs: Vec<char> = "abcde".chars().collect();
        program.instructions.iter().for_each(|dance| {
            dance.moves(&mut programs)
        });

        assert_eq!(programs, vec!['b', 'a', 'e', 'd', 'c'])
    }
}