// again with Hashmap
use advent_2018::{Runner, get_file};
use regex:: Regex;

pub struct Q16 {

}

impl Q16 {
    pub fn new() -> Self {
        Q16 {}
    }

    fn part1(&mut self) {
        let samples = Samples::new();
        let mut count = 0;
        
        samples.data.iter().for_each(|sample| {
            let memory = Memory::new();
            let register = &sample[0];
            let instruction = &sample[1];
            let result = &sample[2];
            count += memory.test(register.clone(), instruction.iter().map(|num| *num as usize).collect(), result.clone());
        });
        println!("count {:?}", count);
    }

    fn part2(&mut self) {
        let samples = Samples::new();

        let memory = Memory::new();
        // register[instruction[3]] = num; 
        let mut register = vec![0,0,0,0];
        //  
        samples.tests.iter().skip(1).for_each(|instruction| {
            let num;
            match instruction[0] {
                0 => {
                    num = memory.execute(Ops::Bani(register[instruction[1]], instruction[2] as i32));
                }
                1 => {
                    num = memory.execute(Ops::Banr(register[instruction[1]], register[instruction[2]]));
                }
                2 => {
                    num = memory.execute(Ops::Muli(register[instruction[1]], instruction[2] as i32));
                }
                3 => {
                    num = memory.execute(Ops::Setr(register[instruction[1]]));
                }
                4 => {
                    num = memory.execute(Ops::Eqrr(register[instruction[1]], register[instruction[2]]));
                }
                5 => {
                    num = memory.execute(Ops::Banr(register[instruction[1]], register[instruction[2]]));
                }
                6 => {
                    num = memory.execute(Ops::Gtir(instruction[1] as i32, register[instruction[2]]));
                }
                7 => {

                    num = memory.execute(Ops::Mulr(register[instruction[1]], register[instruction[2]]));
                }
                8 => {
                    num = memory.execute(Ops::Gtrr(register[instruction[1]], register[instruction[2]]));
                }
                9 => {
                    num = memory.execute(Ops::Seti(instruction[1] as i32));
                }
                10 => {
                    num = memory.execute(Ops::Gtri(register[instruction[1]], instruction[2] as i32));
                }
                11 => {
                    num = memory.execute( Ops::Eqri(register[instruction[1]], instruction[2] as i32));
                }
                12 => {
                    num = memory.execute(Ops::Addi(register[instruction[1]], instruction[2] as i32));
                }
                13 => {
                    num = memory.execute(Ops::Borr(register[instruction[1]], register[instruction[2]]));
                }
                14 => {
                    num = memory.execute(Ops::Eqir(instruction[1] as i32, register[instruction[2]]));
                }
                15 => {
                    num = memory.execute(Ops::Addr(register[instruction[1]], register[instruction[2]]));
                }
                _ => panic!("wrong")
            }
            register[instruction[3]] = num; 
        });
        println!("{:?}", register);
    }
}

struct Samples {
    data: Vec<Vec<Vec<i32>>>,
    tests: Vec<Vec<usize>>
}

impl Samples {
    fn new() -> Self{
        let content = get_file("src/input/q16.txt").unwrap();
        let (content, testing) = content.split_once("\r\n\r\n\r\n").unwrap();
        let info : Vec<_>= content.split("\r\n\r\n").collect();
        let mut data = vec![];
        info.iter().for_each(|sample| {
            let re = Regex::new(r"\d+").unwrap();
            let mut d = vec![];
            sample.lines().for_each(|text| {
                let mut s = vec![];
                for captured in re.captures_iter(text) {
                    let number = captured[0].parse::<i32>().unwrap();
                    s.push(number);
                }
                d.push(s)
            });
            data.push(d);
        });
        let tests: Vec<_> = testing.lines().map(|line| line.split_whitespace().map(|num| num.parse::<usize>().unwrap()).collect()).collect();
        Self { data , tests}
    }
}

struct Memory {
}

// Before: [3, 2, 1, 1]
// 9 2 1 2
// After:  [3, 2, 2, 1]

impl Memory {
    fn new() -> Self {
        Self {}
    }
    fn test(&self, mut register: Vec<i32>, instruction: Vec<usize>, result: Vec<i32>) -> i32 {
        
        let ops = vec![
            Ops::Addr(register[instruction[1]], register[instruction[2]]), 
            Ops::Addi(register[instruction[1]], instruction[2] as i32),
            Ops::Mulr(register[instruction[1]], register[instruction[2]]),
            Ops::Muli(register[instruction[1]], instruction[2] as i32),
            Ops::Banr(register[instruction[1]], register[instruction[2]]),
            Ops::Bani(register[instruction[1]], instruction[2] as i32),
            Ops::Borr(register[instruction[1]], register[instruction[2]]),
            Ops::Bori(register[instruction[1]], instruction[2] as i32),
            Ops::Setr(register[instruction[1]]),
            Ops::Seti(instruction[1] as i32),
            Ops::Gtir(instruction[1] as i32, register[instruction[2]]),
            Ops::Gtri(register[instruction[1]], instruction[2] as i32),
            Ops::Gtrr(register[instruction[1]], register[instruction[2]]),
            Ops::Eqir(instruction[1] as i32, register[instruction[2]]),
            Ops::Eqri(register[instruction[1]], instruction[2] as i32),
            Ops::Eqrr(register[instruction[1]], register[instruction[2]]),
        ];

        let mut matches = 0;
        let mut matching_ops = vec![];
        ops.iter().for_each(|op| {
            let num = self.execute(op.clone());
            register[instruction[3]] = num; 
            if result == register {
                matches += 1;
                matching_ops.push(op.clone());
            }
        });

        if instruction[0] == 15 {
            println!("matching_ops - {:?}", matching_ops);
            println!("instruction - {:?}", instruction);
            println!("");
            1
        } else {
            0
        }
    }

    fn execute(&self, ops: Ops) -> i32{
        //  Eqri - 11, Gtrr - 8, Gtri - 10, Eqrr - 5, Eqir - 14
        //  Banr -  1, Bani - 0, Bori -  4,  Gtir* - 6, Setr* - 3
        //  Seti - 9,  Borr - 13, Addr - 15, Muli - 2, Addi - 12
        //  Mulr - 7
        match ops {
            Ops::Addr(reg1, reg2) => reg1 + reg2,
            Ops::Addi(reg1,val2) => reg1 + val2,
            Ops::Mulr(reg1, reg2) => reg1 * reg2,
            Ops::Muli(reg1, val2) => reg1 * val2,
            Ops::Banr(reg1, reg2) => reg1 & reg2,
            Ops::Bani(reg1, val2) => reg1 & val2,
            Ops::Borr(reg1, reg2) => reg1 | reg2,
            Ops::Bori(reg1, val2) => reg1 | val2,
            Ops::Setr(reg1) => reg1,
            Ops::Seti(val1) => val1,
            Ops::Gtir(val1,reg2) => {
                if val1 > reg2 {1} else {0}
            },
            Ops::Gtri(reg1,val2) => {
                if reg1 > val2 {1} else {0}
            },
            Ops::Gtrr(reg1,reg2) => {
                if reg1 > reg2 {1} else {0}
            },
            Ops::Eqir(val1,reg2) => {
                if val1 == reg2 {1} else {0}
            },
            Ops::Eqri(reg1,val2) => {
                if reg1 == val2 {1} else {0}
            },
            Ops::Eqrr(reg1,reg2) => {
                if reg1 == reg2 {1} else {0}
            }, 
            _ => panic!("wrong ops")
        }
    }

}

#[derive(Debug, Clone, Copy)]
enum Ops {
    Addr(i32,i32),
    Addi(i32,i32),
    Mulr(i32,i32),
    Muli(i32,i32),
    Banr(i32,i32),
    Bani(i32,i32),
    Borr(i32,i32),
    Bori(i32,i32),
    Setr(i32),
    Seti(i32),
    Gtir(i32,i32),
    Gtri(i32,i32),
    Gtrr(i32,i32),
    Eqir(i32,i32),
    Eqri(i32,i32),
    Eqrr(i32,i32),
}



impl Runner for Q16 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q16() {
        assert_eq!(1, 1);
    }
}