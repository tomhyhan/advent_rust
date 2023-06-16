use advent_2018::{Runner, get_file};


struct Memory {
    ip: i32,
    program: Vec<Vec<String>>
}

impl Memory {
    fn new() -> Self {
        let content = get_file("src/input/q19.txt").unwrap();
        let mut content_iter = content.lines();
        let (_, mut ip)= content_iter.next().unwrap().split_once(" ").unwrap();   
        let ip = ip.parse::<i32>().unwrap();
        let mut program: Vec<Vec<String>> = vec![];
        while let Some(line) = content_iter.next() {
            program.push(line.split_whitespace().map(String::from).collect())
        }
        println!("{:?}", ip);
        println!("{:?}", program);
        Self {ip, program}
    }

    fn test(&self, mut register: Vec<i32>, instruction: Vec<usize>, result: Vec<i32>) -> i32 {

        0
    }

    fn execute(&self, ops: Ops) -> i32{
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


pub struct Q19 {

}

// setr/seti => absolute jump
// addr/addi => relative jumps

impl Q19 {
    pub fn new() -> Self {
        Q19 {}
    }

    fn part1(&mut self) {
        let mut memory = Memory::new();
        // register[instruction[3]] = num; 
        let mut register = vec![1,0,0,0,0,0];
        //  
        let mut instruction_ip = 0;
        let mut cnt = 0;
        while instruction_ip < memory.program.len() as i32 {
            println!("{:?}", instruction_ip);
            register[memory.ip as usize] = instruction_ip;
            let instruction = &memory.program[instruction_ip as usize];
            // println!("{:?}", instruction);
            let num;
            let register_a = instruction[1].parse::<usize>().unwrap();
            let register_b = instruction[2].parse::<usize>().unwrap();
            let register_c = instruction[3].parse::<usize>().unwrap();
    
            match instruction[0].as_str() {
                "bani" => {
                    num = memory.execute(Ops::Bani(register[register_a], register_b as i32));
                }
                "banr" => {
                    num = memory.execute(Ops::Banr(register[register_a], register[register_b]));
                }
                "muli" => {
                    num = memory.execute(Ops::Muli(register[register_a], register_b as i32));
                }
                "setr" => {
                    num = memory.execute(Ops::Setr(register[register_a]));
                }
                "eqrr" => {
                    num = memory.execute(Ops::Eqrr(register[register_a], register[register_b]));
                }
                "bori" => {
                    num = memory.execute(Ops::Bori(register[register_a], register[register_b]));
                }
                "gtir" => {
                    num = memory.execute(Ops::Gtir(register_a as i32, register[register_b]));
                }
                "mulr" => {
    
                    num = memory.execute(Ops::Mulr(register[register_a], register[register_b]));
                }
                "gtrr" => {
                    num = memory.execute(Ops::Gtrr(register[register_a], register[register_b]));
                }
                "seti" => {
                    num = memory.execute(Ops::Seti(register_a as i32));
                }
                "gtri" => {
                    num = memory.execute(Ops::Gtri(register[register_a], register_b as i32));
                }
                "eqri" => {
                    num = memory.execute( Ops::Eqri(register[register_a], register_b as i32));
                }
                "addi" => {
                    num = memory.execute(Ops::Addi(register[register_a], register_b as i32));
                }
                "borr" => {
                    num = memory.execute(Ops::Borr(register[register_a], register[register_b]));
                }
                "eqir" => {
                    num = memory.execute(Ops::Eqir(register_a as i32, register[register_b]));
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
            println!("{:?}", instruction);
            println!("{:?}", register);
            // println!("");
            // if  instruction_ip == 25 {
            // println!("{:?}", register);
            // }
            // if register == vec![0, 989, 989, 3, 989, 1] {
            if cnt == 40 {
                break
            }
            cnt += 1;
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
        println!("{:?}", register);
        println!("{:?}", 10551389 + 1 + 67)
    }


    fn part2(&mut self) {
        // 3
        // ["mulr", "5", "2", "1"]
        // [0, 1, 1, 3, 989, 1]
        // [0, 2, 2, 3, 989, 1] .. [0, 989, 989, 3, 989, 1]

        // [0, 1, 990, 3, 989, 1]
        // [0, 1, 990, 3, 989, 2]

        // // [0, 1, 990, 3, 989, 990] 
        
    }
    

}

fn find_divisors(num:i32) -> Vec<i32> {
    let mut divisors = vec![];
    let mut rest = vec![];
    for i in 1..=(num as f64).sqrt() as i32 {
        if num % i == 0 {
            divisors.push(i);
            rest.push(num / i);
        }
    }
    println!("{:?}", divisors.iter().chain(rest.iter().rev()).sum::<i32>());
    divisors.into_iter().chain(rest.into_iter().rev()).collect()
}

impl Runner for Q19 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    use super::find_divisors;

    #[test]
    fn Q19() {
        let r = find_divisors(10551389);
        assert_eq!(r, vec![1,2,3,4,6,12])
    }
}

// addi => ra + vb -> rc 
// addr => ra + rb -> rc
// seti => va -> rc
// setr => ra -> rc
// mulr => ra * rb -> rc
// muli => ra * vb -> rc
// eqrr => rc = 1 if r a == r b, else rc = 0
// gtrr => rc = 1 if ra > rb, else rc = 0

// [0,0,0,0,0,0] - 0
// [0,0,0,16,0,0] - 0
// [0,0,0,16,0,2] - 1
// [0,0,0,16,0,0] - 17
// [0,0,0,9,0,0] - 10
// [0,0,0,10,0,0] - 11
// 
