use std::{
    borrow::BorrowMut,
    cell::{RefCell, RefMut},
    collections::HashMap, ptr::NonNull,
};

use crate::Runner;

pub struct Q19 {}

#[derive(Debug, Clone, PartialEq)]
struct Node {
    next : Option<NonNull<Node>>,
    val: i32
}

impl Q19 {
    pub fn new() -> Self {
        Q19 {}
    }

    fn part1(&self) {
        let elf_count : u32 = 3018458;
        let prev = elf_count.next_power_of_two() >> 1;
        println!("{}", ((elf_count - prev) * 2) + 1)
    }

    fn part2(&self) {
        let elf_count: u32 = 3018458;
        let mut survivor = 1;
        while survivor * 3 < elf_count {
            survivor *= 3;
        }
        if elf_count < 2 * survivor {
            survivor = elf_count - survivor;
        } else {
            survivor = 2 * elf_count - 3 * survivor;
        }

        println!("{survivor}");
    }   
}

impl Runner for Q19 {
    fn run(&mut self) -> () {
        // self.part1()
        self.part2()
    }
}
