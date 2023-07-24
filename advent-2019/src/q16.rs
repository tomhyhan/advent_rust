use std::collections::VecDeque;

use advent_2019::{Runner, get_file};

// pattern
// input
// keeps only one digit
// repeat pattern with position of output list

struct Frequency{
    input: Vec<i32>,
    base_pattern: VecDeque<i32>,
}

impl Frequency {
    fn new() -> Self {
        let content = get_file("src/input/q16.txt").unwrap();
        let input = content.chars().map(|c| c.to_digit(10).unwrap() as i32).collect();
        let base_pattern = VecDeque::from([0,1,0,-1]);
        Self {input,base_pattern}
    }
// 123123123123
// 234123412341
// 123123123123123123123123
// 122334411223344112233441
// 123123123123123123123123123123123123123123123123123123123123
// 11222333444111222333444111222333444111222333444111222333444
    fn transmission(&mut self, phase: usize, input_repeated: usize) {
        // input len = 3 base pattern len = 4 lcm = 12 / 3 = 4 
 
        for _ in 0..phase {
            let mut next_input = vec![];    
            for position in 0..self.input.len() {
                let mut current_input = self.input.clone();
                let repeat = position + 1;
                let pattern_len =  self.base_pattern.len();
                // input should be changed here
                let r = pattern_len * repeat / gcd((pattern_len * repeat) as i32, current_input.len() as i32) as usize ;
                current_input = current_input.repeat(r);
                
                let mut i = 1;
                let mut input = vec![];
                for num in current_input.iter() {
                    let base_num = (i / repeat) % pattern_len;
                    input.push(num * self.base_pattern[base_num]);
                    i+= 1
                }
                let cycle = input_repeated / r;
                let left = (input_repeated % r) as usize; 
                let left_some = input[0..left].iter().cloned().sum::<i32>();
                let input_sum: i32 = input.iter().sum::<i32>() * cycle as i32 + left_some;
                next_input.push(input_sum.abs() % 10);
            }
            println!("{:?}", next_input);
            self.input = next_input;
        }
        
    }
}
// 4810
// 8, 6, 0, 0
// 2, 4, 6, 0, 4, 2
// 10 4 | 4  10 | 
fn gcd(x:i32,y:i32) -> i32 {
    if x == 0 {y} else {gcd(y%x,x)}    
}

pub struct Q16 {
    
}

impl Q16 {
    pub fn new() -> Self {
        Q16 {}
    }

    fn part1(&mut self) {
        // let mut frequency = Frequency::new();
        // frequency.transmission(100, 10000);
        
        // println!("{:?}", frequency.input);
        // let x = vec![1,2,3];
    }
    

    fn part2(&mut self) {
        let mut frequency = Frequency::new();
        frequency.transmission(100, 10000);
        println!("{:?}", frequency.input);
        println!("{:?}", frequency.input[0..8].iter().cloned().map(|num| num.to_string()).collect::<String>().parse::<i32>().unwrap() % frequency.input.len() as i32);
        println!("{:?}", gcd(3, 8));
        
    }
// .iter().cloned().map(|num| num.to_string()).collect::<String>()
}

impl Runner for Q16 {
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
    fn Q16() {
        assert_eq!(1, 1);
    }
}