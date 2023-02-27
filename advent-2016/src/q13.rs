use crate::Runner;

pub struct Q13 {}

impl Q13 {
    pub fn new() -> Self {
        Q13 {}
    }
}

impl Runner for Q13 {
    fn run(&mut self) -> () {
        println!("Hi there!");
        let mut min = i32::MAX;
        min = min.min(1);
    }
}
