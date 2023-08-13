use std::collections::HashSet;

use advent_2020::{Runner, get_file};

struct Questions {
    groups: Vec<Vec<HashSet<char>>>
}

impl Questions {
    fn new() -> Self{
        let content = get_file("src/input/q6.txt").unwrap();
        let answers: Vec<_> = content.split("\r\n\r\n").collect();
        let groups = answers.iter().map(|line|{
            let answer = line.lines().map(|l| l.chars().collect::<HashSet<char>>()).collect();
            answer
        }).collect(); 
        Self {groups}
    }
    // part2
    fn count_yes(&self)-> usize {
        self.groups.iter().cloned().map(|answers| {
            let intersection = answers.into_iter().reduce(|a,b| a.intersection(&b).cloned().collect()).unwrap();
            intersection.iter().count()
        }).sum::<usize>()
    }

    // part1
    // fn count_yes(&self)-> usize {
    //     self.groups.iter().map(|answers| {
    //         answers.chars().collect::<HashSet<char>>().iter().count()
    //     }).sum::<usize>()
    // }
}

pub struct Q6 {

}

impl Q6 {
    pub fn new() -> Self {
        Q6 {}
    }

    fn part1(&mut self) {
        let questions = Questions::new();
        println!("{:?}", questions.count_yes());

    }

    fn part2(&mut self) {
    }

}

impl Runner for Q6 {
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
    fn Q6() {
        assert_eq!(1, 1);
    }
}