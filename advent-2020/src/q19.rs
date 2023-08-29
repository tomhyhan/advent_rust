use std::collections::{HashMap, HashSet};

use advent_2020::{Runner, get_file};

struct Satellite {
    rules: HashMap<String, String>,
    valids: HashSet<String>
}

impl Satellite {
    fn new() -> Self {
        let content = get_file("src/input/q19.txt").unwrap();
        let (up, down) = content.split_once("\r\n\r\n").unwrap();
        let rules = up.lines().map(|line|{
            let (key, value) = line.split_once(": ").unwrap();
            (key.to_string(), value.trim_matches('\"').to_string())
        }).collect();
        let valids = down.lines().map(|line| line.to_string()).collect();
        Self {rules, valids}
    }

    fn valid_messages(&self, start: String) {
        let matches = self.rules.get(&start).unwrap();
        let mut comb = vec![vec!["".to_string()]];
        for m in matches.split_whitespace() {
            let current_comb = self.get_messages(m.to_string());
            comb.push(current_comb);
            let c = comb.clone();
            comb = vec![self.combine_vec(c)]; 
        }
        println!("{:?}", comb);
        let mut valids = 0;
        for c in comb[0].iter() {
            if self.valids.contains(c) {
                valids += 1;
            }
        }
        println!("valids - {:?}", valids);
    }

    fn get_messages(&self, rule: String) -> Vec<String>{
        // println!("rule - {:?}", rule);
        let matches = self.rules.get(&rule).unwrap();
        if matches == "a" || matches == "b" {
            return vec![matches.to_string()]
        }
        let mut messages = vec![];

        for sub_rules in matches.split(" | ") {
            let mut sub_message = vec![];
            for m in sub_rules.split_whitespace() {
                sub_message.push(self.get_messages(m.to_string()));
            }
            // println!("sub - {:?} {}", sub_message, rule);
            let comb = self.combine_vec(sub_message);
            messages.extend(comb);
        }
        // println!("messages - {:?}", messages);
        messages
    }

    fn combine_vec(&self, vec: Vec<Vec<String>>) -> Vec<String>{
        let mut comb = vec![];
        if vec.len() == 1 {
            return vec[0].clone()
        }
        for v1 in vec[0].iter() {
            for v2 in vec[1].iter() {
                comb.push(format!("{v1}{v2}"));
            }
        }
        comb
    }
}



pub struct Q19 {

}

impl Q19 {
    pub fn new() -> Self {
        Q19 {}
    }

    fn part1(&mut self) {
        let satellite = Satellite::new();
        // satellite.valid_messages("0".to_string());
    }

    fn part2(&mut self) {
        let satellite = Satellite::new();
        println!("{:?}", satellite.get_messages("42".to_string()));
        println!("{:?}", satellite.get_messages("31".to_string()));
    }

}

impl Runner for Q19 {
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
    fn Q19() {
        assert_eq!(1, 1);
    }
}