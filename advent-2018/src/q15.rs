use std::collections::{HashMap, VecDeque, HashSet};

use advent_2018::{Runner, get_file};
use regex::internal::Char;

pub struct Q15 {

}

pub struct Game {
    map: HashMap<(i32, i32), Tile>,
    min_row: i32,
    max_row: i32,
    min_col: i32,
    max_col: i32,
}

impl Game {
    fn new() -> Self {
        let contents = get_file("src/input/q15.txt").unwrap();
        let mut map = HashMap::new();
        let mut min_row = i32::MAX;
        let mut max_row = i32::MIN;
        let mut min_col = i32::MAX;
        let mut max_col = i32::MIN;
        
        contents.lines().enumerate().for_each(|(row,line)| {
            line.chars().enumerate().for_each(|(col, c)| {
                let row = row as i32;
                let col = col as i32;
                match c {
                    '#' => map.insert((row,col), Tile::Wall),
                    '.' => map.insert((row,col), Tile::Open),
                    'G' => map.insert((row,col), Tile::Goblin(CharInfo::new(row,col))),
                    'E' => map.insert((row,col), Tile::Elf(CharInfo::new(row,col))),
                    _ => panic!("not a member of a map")
                };
                min_row = min_row.min(row);  
                max_row = max_row.max(row); 
                min_col = min_col.min(col); 
                max_col = max_col.max(col); 
                
            })
        });
        Self {map,max_col,max_row,min_col,min_row}
    }

    fn combat(&mut self) {
        let mut rounds = 0;

        loop {
            for row in self.min_row..=self.max_row {
                for col in self.min_col..=self.max_col {
                    let current_tile = self.map.get_mut(&(row,col)).cloned().unwrap();
                    match current_tile {
                        Tile::Elf(info) => {
                            if self.check_enemy(row,col,&info) {
                                continue
                            }
                            let mut reachable = self.find_reachable(row,col,'G');
                            if reachable.is_empty() {
                                continue
                            };
                            reachable.sort();
                            let togo = reachable[0].clone(); 
                            self.find_next_step(row,col, togo);
                        },
                        Tile::Goblin(info) => {
                            self.find_reachable(row,col,'E');
                        },
                        _ => {}
                    }
                }
            }
            rounds += 1;   
            break         
        }
    }
    
    fn check_enemy(&mut self, row: i32, col: i32, info: &CharInfo) -> bool {
        let mut enemies = vec![];
        for dir in [(1,0),(0,1),(-1,0),(0,-1)] {
            let new_row = row + dir.0;
            let new_col = col + dir.1;
            let neighbor = self.map.get(&(new_row,new_col)).unwrap();
            if let Tile::Goblin(info) = neighbor {
                enemies.push((new_row,new_col,info.clone()));
            }
        }
        if enemies.is_empty() {
            false    
        } else {
            // find enemy with lowest health
            // enemies.iter().min_by(|e1,e2| e1.2. )
            true
        }
    }

    fn find_next_step(&self, row:i32, col:i32, togo: (i32,i32,i32)) {
        // let mut neighbors = vec![];
        for dir in [(1,0),(0,1),(-1,0),(0,-1)] {
            let new_row = row + dir.0;
            let new_col = col + dir.1;
            let neighbor = self.map.get(&(new_row,new_col)).unwrap();
        }
    }

    fn find_reachable(&self,row:i32, col:i32, to_find: char) ->  Vec<(i32,i32,i32)> {
        let mut queue = VecDeque::from([(row,col)]);
        let mut reachable:Vec<(i32,i32,i32)> = Vec::new();
        let mut seen: HashSet<(i32,i32)> = HashSet::new();

        while let Some((c_row,c_col)) = queue.pop_front() {
            if !seen.insert((c_row,c_col)) {
                continue
            }

            for dir in [(1,0),(0,1),(-1,0),(0,-1)] {
                let new_row = c_row + dir.0;
                let new_col = c_col  + dir.1;
                let neighbor = self.map.get(&(new_row,new_col)).unwrap();
                match to_find {
                    'G' => {
                        if let Tile::Goblin(_) = neighbor {
                            let distance = (row - c_row).abs() + (col - c_col).abs();  
                            reachable.push((c_row,c_col, distance));
                        } else if let Tile::Open = neighbor {
                            queue.push_back((new_row,new_col));
                        }
                    },  
                    'E' => {
                        if let Tile::Elf(_) = neighbor {
                            let distance = (row - c_row).abs() + (col - c_col).abs();  
                            reachable.push((c_row,c_col,distance));
                        } else if let Tile::Open = neighbor {
                            queue.push_back((new_row,new_col));
                        }
                    },
                    _ => panic!("unknown character to find!")
                }
            }
        }
        if reachable.is_empty() {
            return vec![]
        }
        let min_val = reachable.iter().map(|r| r.2).min().unwrap();
        reachable.iter().filter(|&r| r.2 == min_val).cloned().collect()
    }
}


#[derive(Debug,Clone, Copy)]
enum Tile {
    Wall,
    Open,
    Goblin(CharInfo),
    Elf(CharInfo),
}

#[derive(Debug,Clone, Copy)]
struct CharInfo {
    row: i32,
    col:i32,
    hp: i32,
    attack: i32
}

impl CharInfo {
    fn new(row: i32,col: i32) -> Self {
        Self {row,col,hp:200,attack:3}
    }
}

impl Q15 {
    pub fn new() -> Self {
        Q15 {}
    }

    fn part1(&mut self) {
        let mut game = Game::new();
        game.combat();

    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q15 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q15() {
        assert_eq!(1, 1);
    }
}