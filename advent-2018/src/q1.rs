use std::collections::HashSet;

use advent_2018::{Runner, get_file};

pub struct Q1 {
    input: Vec<i32>
}

impl Q1 {
    pub fn new() -> Self {
        let content = get_file("src/input/q1.txt").unwrap();
        let input = content.lines().map(|num| num.parse::<i32>().unwrap()).collect();
        Q1 {input}
    }

    fn part1(&mut self) {
        let sum: i32 = self.input.iter().sum();
        println!("{:?}", sum)
    }

    fn part2(&mut self) {
        let mut deduplicate = HashSet::new();

        let mut frequency = 0;
        let mut idx = 0;
        loop {
            let num = self.input[idx];
            frequency += num;
            if !deduplicate.insert(frequency) {
                println!("{:?}", frequency);
                break
            }
            idx = (idx + 1) % self.input.len();
        }
    }

}

impl Runner for Q1 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q1() {
        assert_eq!(1, 1);
    }
}