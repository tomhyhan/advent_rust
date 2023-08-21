use std::collections::{HashMap, VecDeque};

use advent_2020::Runner;

struct MemoryGame {
    starting_list: Vec<i32>
}

impl MemoryGame {
    fn new(starting_list: Vec<i32>) -> Self {
        Self {
            starting_list
        }
    }
    // 0 3 6 0
    fn play(&self) {
        // let mut memory_list = vec![];
        let mut memo = HashMap::new();
        let mut turn = 1;
        let mut last_spoken = 0;
        while turn <= 30000000 {
            let current_idx = turn - 1;
            if turn <= self.starting_list.len() {
                let val = self.starting_list[current_idx];
                memo.insert(val, VecDeque::from([(turn)]));
                last_spoken = val;
                turn += 1;
                continue
            }
            let record = memo.get_mut(&last_spoken).unwrap();
            if record.len() < 2 {
                last_spoken = 0;
                memo.entry(last_spoken).or_insert(VecDeque::new()).push_back(turn);
            } else {
                let (current, prev) = (record[1], record[0]);
                last_spoken = (current - prev) as i32;
                memo.entry(last_spoken).or_insert(VecDeque::new()).push_back(turn);
            }            
            if memo.get_mut(&last_spoken).unwrap().len() > 2 {
                memo.get_mut(&last_spoken).unwrap().pop_front();
            } 
            turn += 1;
        }
        println!("{:?}", last_spoken);
    }
}

struct Number {
    location: usize,
    val:i32
}

pub struct Q15 {

}
impl Q15 {
    pub fn new() -> Self {
        Q15 {}
    }

    fn part1(&mut self) {
        let game = MemoryGame::new(vec![0,8,15,2,12,1,4]);
        game.play();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q15 {
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
    fn Q15() {
        assert_eq!(1, 1);
    }
}