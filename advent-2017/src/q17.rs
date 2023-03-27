use std::collections::{HashMap, HashSet};

use advent_2017::Runner;


struct SpinLock {
    state: Vec<usize>,
    pointer: usize,
    steps: usize
}

impl SpinLock {
    fn new(steps: usize) -> Self {
        let state = vec![0];
        let pointer = 0;
        SpinLock { state, pointer, steps }
    }

    fn spin(&mut self, jump:usize) {
        self.pointer = ((self.pointer + self.steps) % jump) + 1;
        // part 2
        if self.pointer == 1 {
            println!("{:?}", jump);
        };
        // part 1
        // self.state.insert(self.pointer, jump);
    }
}

pub struct Q17 {

}

impl Q17 {
    pub fn new() -> Self {
        Q17 {}
    }

    fn part1(&mut self) {
        let mut spinlock = SpinLock::new(314);

        for jump in 1..50000000 {
            spinlock.spin(jump);
        };

    }
}

impl Runner for Q17 {
    fn run(&mut self) {
        self.part1();
    }
}