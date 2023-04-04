use std::collections::HashMap;

use advent_2017::{Runner, get_file};

pub struct Q22 {

}

struct Grid {
    map: HashMap<(i32,i32), State>,
    virus: Virus,
    infection: i32,
}

#[derive(Debug, PartialEq, Eq)]
enum State{
    Clean,
    Weakened,
    Infected,
    Flagged
}

#[derive(Debug)]
enum Direction {
    Left,
    Right,
    Continue,
    Reverse
}

impl Grid {
    fn new() -> Self {
        let content = get_file("q22.txt").unwrap();
        let mut map = HashMap::new();

        let list_map: Vec<Vec<char>>  = content.lines().map(|line| line.chars().collect::<Vec<char>>()).collect();
    
        let mut virus = Virus {direction : (-1,0), pos: (0,0)}; 
        for row in 0..list_map.len() {
            for col in 0..list_map[0].len() {
                if list_map[row][col] == '#' {
                    map.insert((row as i32, col as i32), State::Infected);
                }
                if row == list_map.len() / 2 && col == list_map[0].len() / 2 {
                    virus.pos = (row as i32, col as i32);
                }
            }
        };
        let infection = 0;
        Grid {
            map,
            virus,
            infection 
        }
    }
    
    //  Clean -> Weakened -> Infected -> Flagged -> Clean
    fn bursts(&mut self, mut move_cnt: i32) {
        let directions = HashMap::from([((-1,0),((0,-1),(0,1))), ((0,1),((-1,0),(1,0))), ((1,0),((0,1),(0,-1))), ((0,-1),((1,0),(-1,0)))]);
        
        while move_cnt > 0 {
            match self.is_current_infected() {
                Direction::Right  => {
                    let next_direction = directions.get(&self.virus.direction).unwrap().1;
                    self.virus.pos.0 += next_direction.0;
                    self.virus.pos.1 += next_direction.1;
                    self.virus.direction = next_direction;
                },
                Direction::Left  => {
                    let next_direction = directions.get(&self.virus.direction).unwrap().0;
                    self.virus.pos.0 += next_direction.0;
                    self.virus.pos.1 += next_direction.1;
                    self.virus.direction = next_direction;
                },
                Direction::Continue  => {
                    self.virus.pos.0 += self.virus.direction.0;
                    self.virus.pos.1 += self.virus.direction.1;
                    
                },
                Direction::Reverse => {
                    self.virus.direction.0 = -self.virus.direction.0; 
                    self.virus.direction.1 = -self.virus.direction.1; 
                    self.virus.pos.0 += self.virus.direction.0;
                    self.virus.pos.1 += self.virus.direction.1;
                },
            }
            move_cnt -= 1;
        }
    }
    

    //  Clean -> Weakened -> Infected -> Flagged -> Clean
    fn is_current_infected(&mut self) -> Direction{
        let current = self.map.entry(self.virus.pos).or_insert(State::Clean);
        match *current {
            State::Clean => {
                *current = State::Weakened;
                Direction::Left
            },
            State::Flagged => {
                *current = State::Clean;
                Direction::Reverse
            },
            State::Infected => {
                *current = State::Flagged;
                Direction::Right
            },
            State::Weakened => {
                *current = State::Infected;
                self.infection += 1;
                Direction::Continue
            }
        }
    }

    
}

struct Virus {
    pos: (i32,i32),
    direction: (i32,i32)
}

impl Q22 {
    pub fn new() -> Self {
        Q22 {}
    }

    fn part1(&mut self) {
        let mut grid = Grid::new();
        println!("{:?}", grid.map);
        grid.bursts(10000000);
        println!("{:?}", grid.infection);
    }
}

impl Runner for Q22 {
    fn run(&mut self) {
        self.part1()
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_bursts()  {
        let mut grid = Grid::new();
        grid.bursts(100);
        assert_eq!(grid.infection, 26);
    }   

    // #[test]
    fn test_bursts1()  {
        let mut grid = Grid::new();
        grid.bursts(10000);
        assert_eq!(grid.infection, 5587);
    }   

    // #[test]
    fn reverse() {
        let mut x = (-1,0);
        x.0 = -x.0;
        x.1 = -x.1;
        println!("{:?}", x);
        assert_eq!(1,2)
    }
}