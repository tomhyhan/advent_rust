use std::collections::HashSet;

use advent_2020::{Runner, get_file};

struct Expense {
    report: Vec<i32>
}

impl Expense {
    fn new() -> Self {
        let content = get_file("src/input/q1.txt").unwrap();
        let report = content.lines().map(|line|line.parse::<i32>().unwrap()).collect();
        Self {report}
    }

    fn find_target(&self, target:i32) -> i32 {
        let mut matches = HashSet::new();
        for num in self.report.iter() {
            if matches.contains(num) {
                return *num * (target-*num)
            } else{
                matches.insert(target- *num);
            }
        }
        -1
    }

    fn find_target_from_two(&self, target:i32, array: &[i32]) -> Option<(i32,i32)> {
        let mut matches = HashSet::new();
        for num in array.iter() {
            if matches.contains(num) {
                return Some((*num, (target-*num)))
            } else{
                matches.insert(target- *num);
            }
        }
        None
    }
    
    fn find_target_from_three(&self, target:i32) -> i32 {
        for i in 0..self.report.len() {
            let num1 = self.report[i]; 
            let sum = target - num1;
            if let Some((num2, num3)) = self.find_target_from_two(sum, &self.report[i+1..]){
                return num1 * num2 * num3
            }
        }
        0
    }
}

pub struct Q1 {

}

impl Q1 {
    pub fn new() -> Self {
        Q1 {}
    }

    fn part1(&mut self) {
        let expense = Expense::new();
        println!("expense - {:?}", expense.find_target(2020));
    }

    fn part2(&mut self) {
        let expense = Expense::new();
        println!("{:?}", expense.find_target_from_three(2020))
    }

}

impl Runner for Q1 {
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
    fn Q1() {
        assert_eq!(1, 1);
    }
}