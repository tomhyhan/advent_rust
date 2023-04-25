use std::collections::{HashMap, HashSet};

use advent_2018::{Runner, get_file};

pub struct Q12 {

}

#[derive(Debug)]
struct Plants {
    state: HashMap<i32, char>,
    patterns: HashMap<String, char>
}

impl Plants {
    fn new() -> Self {
        let mut state = HashMap::new();
        let mut patterns = HashMap::new();
        let contents = get_file("src/input/q12.txt").unwrap();
        let (init_state, init_pattern) = contents.split_once("\r\n\r\n").unwrap();
        
        let (_, s) = init_state.split_once(": ").unwrap();
        s.chars().enumerate().for_each(|(idx, c)|{
            state.insert(idx as i32, c);
        });

        init_pattern.lines().for_each(|line| {
            let (pattern, outcome) = line.split_once(" => ").unwrap();
            patterns.insert(pattern.to_string(), outcome.chars().next().unwrap());
        });
        Plants {patterns, state}
    }

    fn next_generation(&mut self) {
        let mut next_state = HashMap::new();
        let current_state = self.state.clone();
        let start = *current_state.keys().min().unwrap() as i32; 
        let end = *current_state.keys().max().unwrap() as i32; 
        for pot_num in start-2..=end+2 {
            let mut current_pattern = String::new();
            for range in pot_num-2..=pot_num+2 {
                let c_pot = self.state.entry(range).or_insert('.');
                next_state.entry(range).or_insert('.');
                current_pattern.push(*c_pot);
            }
            if self.patterns.contains_key(&current_pattern) {
                let pot = self.patterns.get(&current_pattern).unwrap();
                next_state.insert(pot_num,*pot);
            } else {
                next_state.insert(pot_num, '.');
            }
        }
        self.state = next_state
    }    
}

impl Q12 {
    pub fn new() -> Self {
        Q12 {}
    }

    fn part1(&mut self) {
        let mut plants = Plants::new();
        for i in 0..20 {
            // let mut display = "".to_string();
            // for i in -200..200 {
            //     display.push(*plants.state.get(&i).or_else(||{Some(&'.')}).unwrap());
            // }
            // println!("{:?}", display);
            plants.next_generation();
            let sum = plants.state.iter().filter(|(_,&v)| v=='#').map(|(k,_)| k).sum::<i32>();
            println!("{:?}", sum);
            if i == 200{
                println!("{sum}");
                break
            }
        }
        println!("{:?}", 5468 + (50000000000i64 - 119 - 1) * 42)
        // 6066 42
    }
    

    fn part2(&mut self) {
        
    }

}

impl Runner for Q12 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q12() {
        assert_eq!(1, 1);
    }
}