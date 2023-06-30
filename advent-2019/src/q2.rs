use advent_2019::{Runner, get_file};

pub struct Q2 {

}

#[derive(Debug)]
struct Program {
    integers: Vec<usize>
}

impl Program {
    fn new() -> Self {
        let content = get_file("src/input/q2.txt").unwrap();
        let integers: Vec<usize>= content.split(",").map(|num| num.parse().unwrap()).collect();
        
        Program {integers}
    }

    fn run_program(&mut self) -> usize{
        for i in (0..self.integers.len()).step_by(4) {
            if self.integers[i] == 99 {
                break
            }
            let input1 = self.integers[self.integers[i+1]];
            let input2 = self.integers[self.integers[i+2]];
            match self.integers[i] {
                1 => {
                    let output_integer = self.integers[i+3];
                    self.integers[output_integer] = input1 + input2;
                },
                2 => {
                    let output_integer = self.integers[i+3];
                    self.integers[output_integer] = input1 * input2;
                },
                _ => panic!("invalid opcode")
            }
        }
        self.integers[0]
    }
    
}

impl Q2 {
    pub fn new() -> Self {
        Q2 {}
    }

    fn part1(&mut self) {
        let mut program = Program::new();
        program.integers[1] = 12;
        program.integers[2] = 2;
        program.run_program();
        // 5434661
        println!("{:?}", program.integers);
    }

    fn part2(&mut self) {
        for noun in 0..100 {
            for verb in 0..100 {
                let mut program = Program::new();
                program.integers[1] = noun;
                program.integers[2] = verb;
                if program.run_program() == 19690720 {
                    println!("{:?}", noun * 100 + verb)
                }
            }
        }
    }

}

impl Runner for Q2 {
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
    fn Q2() {
        assert_eq!(1, 1);
    }
}