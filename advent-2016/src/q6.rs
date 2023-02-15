use std::str::FromStr;

use crate::common::my_modules::get_file;

struct Signals {
    messages: Vec<Vec<char>>,
    index: usize,
}

impl Signals {
    fn find_message(&mut self) -> String {
        self.into_iter()
            .map(|message| {
                message
                    .iter()
                    .min_by_key(|&&c| {
                        String::from_iter(message.clone())
                            .to_string()
                            .matches(c)
                            .count()
                    })
                    .unwrap()
                    .clone()
            })
            .collect()
    }
}

impl FromStr for Signals {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut messages = vec![];
        let first_line = s.lines().next().unwrap();
        for _ in first_line.chars() {
            messages.push(Vec::new())
        }

        s.lines().for_each(|line| add_messages(line, &mut messages));

        Ok(Signals { messages, index: 0 })
    }
}

impl Iterator for Signals {
    type Item = Vec<char>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index >= self.messages.len() {
            return None;
        }
        // let current =
        let item = Some(self.messages[self.index].clone());
        self.index += 1;
        item
    }
}

fn add_messages(line: &str, messages: &mut Vec<Vec<char>>) {
    line.chars()
        .enumerate()
        .for_each(|(idx, c)| messages[idx].push(c))
}

pub fn run() {
    let content = get_file("q6.txt").unwrap();
    let mut signals: Signals = content.parse().unwrap();
    println!("{:?}", signals.find_message());
}
