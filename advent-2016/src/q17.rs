use std::collections::HashSet;

use crate::Runner;

pub struct Q17 {
    vault: HashSet<(i32,i32)>,
    key: String
}

impl Q17 {
    pub fn new(key: String) -> Self {
        let mut vault = HashSet::new();
        vault.insert((1,1));
        Q17 {
            vault,
            key
        }
    }
}


impl Runner for Q17 {
    fn run(&mut self) -> () {
        println!("dfdf");
    }
}