use std::{collections::HashMap, fmt::format};

use crate::Runner;
use md5::{self, Digest};
pub struct Q14 {}

impl Q14 {
    pub fn new() -> Self {
        Q14 {}
    }
}

struct Pad {
    cnt: usize,
    salt: String,
    index: usize,
    repeat: usize,
    keys: HashMap<String, String>,
}

impl Pad {
    fn new(salt: String, repeat: usize) -> Self {
        Pad {
            salt,
            index: 0,
            cnt: 0,
            repeat,
            keys: HashMap::new(),
        }
    }

    fn hash(&mut self, hash_string: &str) -> String {
        let mut hash = format!("{:?}", md5::compute(hash_string));
        for _ in 0..self.repeat {
            hash = format!("{:?}", md5::compute(hash));
        }
        hash
    }

    fn find_3repeating_chars(&self, hash: &String) -> Option<u8> {
        let hash_b = hash.as_bytes();

        for chars in hash_b.windows(3) {
            if chars[0] == chars[1] && chars[1] == chars[2] {
                return Some(chars[0]);
            }
        }
        None
    }

    fn find_5repeating_chars(&mut self, matching: u8) -> bool {
        for idx in self.index + 1..=self.index + 1000 {
            let unhashed = format!("{}{}", self.salt, idx);
            let hash = if self.keys.contains_key(&unhashed) {
                self.keys.get(&unhashed).unwrap().clone()
            } else {
                self.hash(&unhashed)
            };

            self.keys.insert(unhashed.clone(), hash.clone());
            for chars in hash.as_bytes().windows(5) {
                if chars.iter().all(|c| *c == matching) {
                    return true;
                }
            }
        }
        false
    }
}

impl Q14 {
    fn part1(&self) {
        let mut pad = Pad::new("ngcjuoqr".to_string(), 2016);

        while pad.cnt < 64 {
            let unhashed = format!("{}{}", pad.salt, pad.index);
            let hash = if pad.keys.contains_key(&unhashed) {
                pad.keys.get(&unhashed).unwrap().clone()
            } else {
                pad.hash(&unhashed)
            };

            pad.keys.insert(unhashed.clone(), hash.clone());
            let repeat_char = pad.find_3repeating_chars(&hash);
            if !repeat_char.is_none() {
                // println!("{hash:?} {}", pad.index);

                if pad.find_5repeating_chars(repeat_char.unwrap()) {
                    println!("{:?} {:?}", pad.index, hash);
                    pad.cnt += 1;
                }
            }
            pad.index += 1;
        }
    }
}

impl Runner for Q14 {
    fn run(&mut self) -> () {
        self.part1()
    }
}
