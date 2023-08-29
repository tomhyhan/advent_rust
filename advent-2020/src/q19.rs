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
        for m in matches.split_whitespace() {
            println!("{:?}", self.get_messages(m.to_string()));
        }
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
                sub_message.extend(self.get_messages(m.to_string()));
            }
            for i in sub_message[1].chars() {

                // messages.push();
            }
        }
        // println!("messages - {:?}", messages);
        messages
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
        satellite.valid_messages("0".to_string());
    }

    fn part2(&mut self) {
        
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