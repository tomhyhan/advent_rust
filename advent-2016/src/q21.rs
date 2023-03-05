use crate::{Runner, common::my_modules::get_file};

pub struct Q21 {
    instructions : Vec<String>
}

impl Q21 {
    pub fn new() -> Self {
        let content = get_file("q21.txt").unwrap();
        let instructions = content.lines().map(|line| line.to_string()).collect();
        Q21 {instructions}
    }

    fn part1(&self) {
        let mut scrambler = Scrambler::new("aefgbcdh".chars().collect(), true);

// .rev()
        self.instructions.iter().rev().for_each(|line| {
            let line : Vec<&str> = line.split_whitespace().collect();
            match line[0] {
                "swap" => {
                    if line[1] == "position" {
                        scrambler.swap_position(line[2].parse::<usize>().unwrap(), line[5].parse::<usize>().unwrap())
                    } else {
                        scrambler.swap_letter(line[2].chars().next().unwrap(), line[5].chars().next().unwrap());
                    }
                }
                "rotate" => {
                    match line[1] {
                        "right" => scrambler.rotate(Direction::right, line[2].parse::<usize>().unwrap()),
                        "left" => scrambler.rotate(Direction::left, line[2].parse::<usize>().unwrap()),
                        _ => scrambler.rotate_on_position(line[6].chars().next().unwrap())
                    }
                },
                "reverse" => {
                    scrambler.reverse(line[2].parse::<usize>().unwrap(), line[4].parse::<usize>().unwrap())
                },
                "move" => {
                    scrambler.move_char(line[2].parse::<usize>().unwrap(), line[5].parse::<usize>().unwrap())
                },
                _ => panic!("unknown instruction")
            }
            println!("{line:?}");
            println!("{:?}", scrambler.puzzle.iter().collect::<String>());
        });
        // afdgechb
    }
}

// "ebcda"
// "edcba"
// "abcde"
// "bcdea"
// "bdeac"
// "abdec"
// "ecabd"
// "decab"
enum Direction {
    left,
    right,
}

struct Scrambler {
    puzzle: Vec<char>,
    reverse_en : bool
}

impl Scrambler {
    fn new(puzzle_str: String, reverse_en: bool) -> Self {
        Scrambler {
            puzzle: puzzle_str.chars().collect(),
            reverse_en
        }
    }

    fn rotate(&mut self, direction: Direction, range: usize) {
        let range = range % self.puzzle.len();
        if self.reverse_en {
            match direction {
                Direction::left => self.puzzle.rotate_right(range),
                Direction::right => self.puzzle.rotate_left(range),
            };    
        } else {
            match direction {
                Direction::left => self.puzzle.rotate_left(range),
                Direction::right => self.puzzle.rotate_right(range),
            };
    
        }
    }

    fn rotate_on_position(&mut self, letter : char){
        let mut rotate = 0;
        let letter_pos = self.puzzle.iter().position(|&c| c == letter).unwrap();
        rotate += letter_pos + 1;
        if letter_pos >= 4{
            rotate += 1
        };
        // println!("{:?}", self.puzzle);
        // println!("{rotate}");cd ..
        // println!(" letter pos - {letter_pos}");
        if self.reverse_en {
            let original = if letter_pos == 0 {1} else if letter_pos < 4 {letter_pos - 1}  else {letter_pos}; 
            self.puzzle.rotate_left(original)
        } else {
            self.rotate(Direction::right, rotate);
        }
    }

    fn swap_position(&mut self, x: usize, y: usize) {
        self.puzzle.swap(x, y);
    }

    fn swap_letter(&mut self, x: char, y: char) {
        //  position
        let x = self.puzzle.iter().position(|&c| c == x).unwrap();
        let y = self.puzzle.iter().position(|&c| c == y).unwrap();

        self.puzzle.swap(x, y);
    }

    fn reverse(&mut self, mut left: usize, mut right: usize) {
        
        while left < right {
            self.puzzle.swap(left, right);
            left += 1;
            right -= 1;
        }
    }

    fn move_char(&mut self,x:usize,y:usize) {
        if self.reverse_en {
            let pop = self.puzzle.remove(y);
            self.puzzle.insert(x, pop);
        } else {
            let pop = self.puzzle.remove(x);
            self.puzzle.insert(y, pop);    
        }
    }

}

impl Runner for Q21 {
    fn run(&mut self) -> () {
        self.part1();
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn scrambler_works() {
        let mut scrambler = Scrambler::new("abcde".chars().collect(), false);
        scrambler.puzzle.swap(4,0);
        assert_eq!(scrambler.puzzle, "ebcda".chars().collect::<Vec<char>>());
        scrambler.swap_letter('d', 'b');
        assert_eq!(scrambler.puzzle, "edcba".chars().collect::<Vec<char>>());
        scrambler.reverse(0, 4);
        assert_eq!(scrambler.puzzle, "abcde".chars().collect::<Vec<char>>());
        scrambler.rotate(Direction::left, 1);
        assert_eq!(scrambler.puzzle, "bcdea".chars().collect::<Vec<char>>());
        scrambler.move_char(1, 4);
        assert_eq!(scrambler.puzzle, "bdeac".chars().collect::<Vec<char>>());
        scrambler.move_char(3, 0);
        assert_eq!(scrambler.puzzle, "abdec".chars().collect::<Vec<char>>());
        scrambler.rotate_on_position('b');
        assert_eq!(scrambler.puzzle, "ecabd".chars().collect::<Vec<char>>());
        scrambler.rotate_on_position('d');
        assert_eq!(scrambler.puzzle, "decab".chars().collect::<Vec<char>>());
    }
}