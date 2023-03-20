use core::panic;

use advent_2017::{Runner, get_file};

pub struct Q9 {

}

struct Stream {
    blocks: Vec<char>
}

impl Stream {
    fn new() -> Self {
        let content = get_file("q9.txt").unwrap();
        let blocks : Vec<char> = content.chars().collect();;
        Stream { blocks }
    }
}

enum State {
    Group,
    Skip,
    Garbage
}

impl Q9 {
    pub fn new() -> Self {
        Q9 {}
    }

    // method 2
    fn part1(&mut self) {
        let stream = Stream::new();

        let mut state = State::Group;
        let mut total = 0;
        let mut garbage_cnt = 0;
        let mut layer = 0;

        for c in stream.blocks.iter() {
            match state {
                State::Group =>  {
                    if *c == '{' {
                        layer += 1
                    } else if *c == '}' {
                        total += layer;
                        layer -= 1;
                    } else if *c == '<' {
                        state = State::Garbage
                    }
                },
                State::Skip => {state = State::Garbage},
                State::Garbage => {
                    if *c == '!' {
                        state = State::Garbage
                    } else if *c == '>' {
                        state = State::Group
                    } else {
                        garbage_cnt += 1;
                    }
                }
            }
        }

        println!("{:?}", total);
        println!("{:?}", garbage_cnt);
    }

    // method 1
    // fn part1(&mut self){
    //     let stream = Stream::new();

    //     let mut layer = 0;
    //     let mut is_garbage = false;
    //     let mut idx = 0;
    //     let mut total = 0;
    //     let mut garbage_cnt = 0;

    //     while idx < stream.blocks.len() {
    //         let current_char = stream.blocks[idx];
    //         match current_char {
    //             '{' => {
    //                 if is_garbage {
    //                     idx += 1;
    //                     garbage_cnt += 1;
    //                     continue
    //                 };
    //                 layer += 1
    //             },
    //             '}' => {
    //                 if is_garbage {
    //                     idx += 1; 
    //                     garbage_cnt+= 1;
    //                     continue;
    //                 };
    //                 total += layer;
    //                 layer -= 1
    //             },
    //             '<' => {
    //                 if is_garbage {
    //                     garbage_cnt += 1
    //                 } else {
    //                     is_garbage = true
    //                 }
    //             }
    //             '>' => {
    //                 is_garbage = false
    //             }
    //             '!' => {
    //                 idx += 1;
    //             }
    //             _ => {
    //                 if is_garbage {
    //                     garbage_cnt += 1
    //                 }
    //             }
    //         }
    //         idx += 1
    //     }

    //     println!("{:?}", total);
    //     println!("{:?}", garbage_cnt);
    // }   
}

impl Runner for Q9 {
    fn run(&mut self) {
        self.part1();
    }
}