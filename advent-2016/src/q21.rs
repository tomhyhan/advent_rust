use crate::Runner;

pub struct Q21 {}

impl Q21 {
    pub fn new() -> Self {
        Q21 {}
    }
}

enum Direction {
    left,
    right,
}

struct Scrambler {
    puzzle: Vec<char>,
}

impl Scrambler {
    fn new(puzzle_str: String) -> Self {
        Scrambler {
            puzzle: puzzle_str.chars().collect(),
        }
    }

    fn rotate(&mut self, direction: Direction, range: usize) {
        match direction {
            Direction::left => self.puzzle.rotate_left(range),
            Direction::right => self.puzzle.rotate_right(range),
        };
    }

    fn swap_position(&mut self, x: usize, y: usize) {
        self.puzzle.swap(x, y);
    }

    fn swap_letter(&mut self, x: char, y: char) {
        //  position
    }
}

impl Runner for Q21 {
    fn run(&mut self) -> () {
        println!("aasd");
    }
}
