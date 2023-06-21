// again
use std::collections::HashSet;

use advent_2018::{Runner, get_file};


struct Memory {
    ip: i64,
    program: Vec<Vec<String>>
}

impl Memory {
    fn new() -> Self {
        let content = get_file("src/input/q21.txt").unwrap();
        let mut content_iter = content.lines();
        let (_, mut ip)= content_iter.next().unwrap().split_once(" ").unwrap();   
        let ip = ip.parse::<i64>().unwrap();
        let mut program: Vec<Vec<String>> = vec![];
        while let Some(line) = content_iter.next() {
            program.push(line.split_whitespace().map(String::from).collect())
        }
        println!("{:?}", ip);
        println!("{:?}", program);
        Self {ip, program}
    }

    fn test(&self, mut register: Vec<i64>, instruction: Vec<usize>, result: Vec<i64>) -> i64 {
        // String { format!("u8: {}", 3) };
        0
    }

    fn execute(&self, ops: Ops) -> i64{
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
    Addr(i64,i64),
    Addi(i64,i64),
    Mulr(i64,i64),
    Muli(i64,i64),
    Banr(i64,i64),
    Bani(i64,i64),
    Borr(i64,i64),
    Bori(i64,i64),
    Setr(i64),
    Seti(i64),
    Gtir(i64,i64),
    Gtri(i64,i64),
    Gtrr(i64,i64),
    Eqir(i64,i64),
    Eqri(i64,i64),
    Eqrr(i64,i64),
}


pub struct Q21 {

}

// setr/seti => absolute jump
// addr/addi => relative jumps

impl Q21 {
    pub fn new() -> Self {
        Q21 {}
    }

    fn part1(&mut self) {
        let mut memory = Memory::new();
        // register[instruction[3]] = num; 
        let mut register = vec![0,0,0,0,0,0];
        //  
        let mut instruction_ip = 0;
        let mut cnt = 0;
        let mut seen = HashSet::new();
        let mut last = 0;
        while instruction_ip < memory.program.len() as i64 {
            // println!("{:?}", instruction_ip);
            register[memory.ip as usize] = instruction_ip;
            let instruction = &memory.program[instruction_ip as usize];
            // println!("{:?}", instruction);
            let num;
            let register_a = instruction[1].parse::<usize>().unwrap();
            let register_b = instruction[2].parse::<usize>().unwrap();
            let register_c = instruction[3].parse::<usize>().unwrap();
            // println!("{:?}", register_b);
            // println!("{:?}", register_a);
            match instruction[0].as_str() {
                "bani" => {
                    num = memory.execute(Ops::Bani(register[register_a], register_b as i64));
                }
                "banr" => {
                    num = memory.execute(Ops::Banr(register[register_a], register[register_b]));
                }
                "muli" => {
                    num = memory.execute(Ops::Muli(register[register_a], register_b as i64));
                }
                "setr" => {
                    num = memory.execute(Ops::Setr(register[register_a]));
                }
                "eqrr" => {
                    num = memory.execute(Ops::Eqrr(register[register_a], register[register_b]));
                }
                "bori" => {
                    num = memory.execute(Ops::Bori(register[register_a], register_b as i64));
                }
                "gtir" => {
                    num = memory.execute(Ops::Gtir(register_a as i64, register[register_b]));
                }
                "mulr" => {
    
                    num = memory.execute(Ops::Mulr(register[register_a], register[register_b]));
                }
                "gtrr" => {
                    num = memory.execute(Ops::Gtrr(register[register_a], register[register_b]));
                }
                "seti" => {
                    num = memory.execute(Ops::Seti(register_a as i64));
                }
                "gtri" => {
                    num = memory.execute(Ops::Gtri(register[register_a], register_b as i64));
                }
                "eqri" => {
                    num = memory.execute( Ops::Eqri(register[register_a], register_b as i64));
                }
                "addi" => {
                    num = memory.execute(Ops::Addi(register[register_a], register_b as i64));
                }
                "borr" => {
                    num = memory.execute(Ops::Borr(register[register_a], register[register_b]));
                }
                "eqir" => {
                    num = memory.execute(Ops::Eqir(register_a as i64, register[register_b]));
                }
                "addr" => {
                    num = memory.execute(Ops::Addr(register[register_a], register[register_b]));
                }
                _ => panic!("wrong")
            }
            register[register_c] = num; 
            instruction_ip = register[memory.ip as usize] + 1;
            // println!("{:?}", register);
            // println!("");
            // println!("{:?}", instruction_ip);
            // println!("{:?}", instruction);
            // println!("{:?}", register);
            // println!("");
            // if  instruction_ip == 25 {
            // println!("{:?}", register);
            // }
            // if register == vec![0, 989, 989, 3, 989, 1] {
            // println!("cnt {:?}", cnt);
            if instruction_ip == 28{
                cnt += 1;
                // println!("{:?}", instruction);
                // println!("{:?}", register);
                if seen.get(&register[4]).is_some() {
                    break
                }
                seen.insert(register[4]);
                // println!("{:?}", register[4]);
                last = register[4];
            }
            // if cnt == 110 {
            //     break
            // }
   
            // if register[5] == 3 {
            //     cnt += 1;
            //     if cnt == 5 {
            //         break

            //     }
            // }
        }
        // println!("{:?}", 174504 / 989);
        // [1056, 1, 990, 256, 989, 990]
        // [0, 560, 560, 3, 989, 1]
        // [67, 0, 884, 2, 989, 73]
        // println!("{:?}", register);
        // println!("{:?}", 65536 + 256);
        // println!("{:?}", 9566170 / 256);
        // println!("{:?}", 255 & 256);
        println!("{:?}", last);
        println!("{:?}", seen)
        // 9566170
        // 9566170
        // 65536
    }


    fn part2(&mut self) {
        let mut d : i64 = 0;
        let mut s = HashSet::new();
        loop {
            let mut e = d | 65536;
            d = 4332021;
            loop {
                let c = e & 255;
                d += c;
                d &= 16777215;
                d *= 65899;
                d &= 16777215;
                if 256 > e {
                    if !s.contains(&d){
                        println!("{d}");
                    }
                    s.insert(d);
                    break
                }
                e = e / 256
            }
        }
        
    }
    

}

fn find_divisors(num:i64) -> Vec<i64> {
    let mut divisors = vec![];
    let mut rest = vec![];
    for i in 1..=(num as f64).sqrt() as i64 {
        if num % i == 0 {
            divisors.push(i);
            rest.push(num / i);
        }
    }
    println!("{:?}", divisors.iter().chain(rest.iter().rev()).sum::<i64>());
    divisors.into_iter().chain(rest.into_iter().rev()).collect()
}

impl Runner for Q21 {
    fn run(&mut self) {
        // self.part1();
        println!("{:?}",0xFF);
        println!("{:?}",0xFFFFFF);
        self.part2();
    }
}


#[cfg(test)]
mod test{
    use super::find_divisors;

    #[test]
    fn Q21() {
        let r = find_divisors(10551389);
        assert_eq!(r, vec![1,2,3,4,6,12])
    }
}



// 9110405
// 4332021
// 11521639

// if the factor of 256 get larger than 65536
// goto 10 -> bani