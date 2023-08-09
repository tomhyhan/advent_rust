use std::collections::VecDeque;

use advent_2019::{Runner, get_file};

// pattern
// input
// keeps only one digit
// repeat pattern with position of output list

struct Frequency{
    input: Vec<i32>,
    base_pattern: VecDeque<i32>,
    offset:i32
}

impl Frequency {
    fn new() -> Self {
        let content = get_file("src/input/q16.txt").unwrap();
        let mut offset = "".to_string();
        let input = content.chars().enumerate().map(|(idx, c)| {
            if idx < 7 {
                offset.push(c)
            }
            c.to_digit(10).unwrap() as i32
        }).collect();
        let offset = offset.parse::<i32>().unwrap();
        let base_pattern = VecDeque::from([0,1,0,-1]);
        Self {input,base_pattern, offset}
    }
// 123123123123
// 234123412341
// 123123123123123123123123
// 122334411223344112233441
// 123123123123123123123123123123123123123123123123123123123123
// 11222333444111222333444111222333444111222333444111222333444
    fn transmission(&mut self, phase: usize) {
        // input len = 3 base pattern len = 4 lcm = 12 / 3 = 4 
        for _ in 0..phase {
            let mut next_input = vec![];    
            let mut repeat = 1;
            for position in 0..self.input.len() {
                let current_input = self.input.clone();
                let pattern_len =  self.base_pattern.len();
                // input should be changed here
                let mut i = 1;
                let mut input = vec![];
                for num in current_input.iter() {
                    let base_num = i / repeat % pattern_len;
                    input.push(num * self.base_pattern[base_num]);
                    i+= 1
                }
                next_input.push(input.iter().sum::<i32>().abs() % 10);
                repeat += 1
            }
            // println!("{:?}", next_input.iter().map(|num| char::from_digit(*num as u32,10).unwrap()).collect::<String>());
            self.input = next_input;
        }
    }

    fn heuristic_transmission(&mut self, phase: usize, input_repeated:usize) {
        self.input = self.input.repeat(input_repeated);
        let offset = self.offset as usize % self.input.len() ;
        self.input = self.input[offset..].to_vec();
        for i in 0..phase {
            let mut sum = self.input.iter().sum::<i32>();
            for j in 0..self.input.len() {
                let current = sum;
                sum -= self.input[j];
                self.input[j] = current % 10
            }
        }
        println!("{:?}", &self.input[0..8])
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
        frequency.heuristic_transmission(100, 1000);
        println!("{:?}", frequency.offset % frequency.input.len() as i32);
        // println!("{:?}", frequency.input);
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