use std::{collections::{HashMap, HashSet}, hash::{Hash, Hasher}};

use advent_2018::{Runner, get_file};
use itertools::Itertools;

pub struct Q13 {

}

#[derive(Debug)]
struct Mine {
    directions: [(i32,i32); 4],
    map: HashMap<(usize,usize),char>,
    carts: Vec<Cart>,
}

impl Mine {
    fn new() -> Self {
        let directions = [(-1,0),(0,1),(1,0),(0,-1)];
        let mut map = HashMap::new();
        let mut carts = vec![];
        let contents = get_file("src/input/q13.txt").unwrap();
        contents.lines().enumerate().for_each(|(row,line)|{
            line.chars().enumerate().for_each(|(col, c)|{
                match c {
                    '-'|'|'|'\\'|'/'|'+' => {map.insert((row,col), c);},
                    '^' => {
                        carts.push(Cart{direction:0,next_turn:0,col,row,crash:false});
                        map.insert((row,col), '|');
                    }
                    'v' => {
                        carts.push(Cart{direction:2,next_turn:0,col,row,crash:false});
                        map.insert((row,col), '|');
                    }
                    '>' => {
                        carts.push(Cart{direction:1,next_turn:0,col,row,crash:false});
                        map.insert((row,col), '-');
                    }
                    '<' => {
                        carts.push(Cart{direction:3,next_turn:0,col,row,crash:false});
                        map.insert((row,col), '-');
                    }
                    _ => ()
                }
            })
        });
        Self { directions, map, carts }
    }

    fn move_carts(&mut self) {
        let mut tick = 0;
        // reminder n e s w
        loop {
            self.carts.sort_by(|x,y|{
                (x.row, x.col).cmp(&(y.row, y.col))
            });

            // println!("{:?}", self.carts);
            if self.carts.len() <= 1 {
                println!("{:?}", self.carts);
                break
            }
            
            let mut positions = HashSet::new();
            let mut removed = HashSet::new();

            for cart in self.carts.iter() {
                if !positions.insert((cart.row, cart.col)) {
                    removed.insert((cart.row, cart.col));
                }
            }

            for cart in self.carts.iter_mut() {
                if removed.contains(&(cart.row, cart.col)) {
                    continue
                }

                positions.remove(&(cart.row, cart.col));

                let direction = &self.directions[cart.direction % 4];
                let next_row = (cart.row as i32 + direction.0) as usize;
                let next_col = (cart.col as i32 + direction.1) as usize;
                let next_move = self.map.get(&(next_row,next_col)).unwrap();
                
                match next_move {
                    '\\' => {
                        match cart.direction {
                            0 | 2 => cart.direction = (cart.direction + 3) % 4,
                            1 | 3 => cart.direction = (cart.direction + 1) % 4,
                            _ => panic!("unknonw direction")
                        }
                    },
                    '/' => {
                        match cart.direction {
                            0 | 2 => cart.direction = (cart.direction + 1) % 4,
                            1 | 3 => cart.direction = (cart.direction + 3) % 4,
                            _ => panic!("unknonw direction")
                        }
                    },
                    '+'=> {
                        match cart.next_turn {
                            0 => cart.direction = (cart.direction + 3) % 4,
                            1 => cart.direction=cart.direction,
                            2 => cart.direction = (cart.direction + 1) % 4,
                            _ => panic!("unknonw cart turn")
                        }
                        cart.next_turn = (cart.next_turn + 1) % 3;
                    }
                    _ => {
                    }
                }
                cart.row = next_row; 
                cart.col = next_col;

                if !positions.insert((cart.row, cart.col)) {
                    removed.insert((cart.row, cart.col));    
                }
            };

            // if !removed.is_empty() {
            self.carts = self.carts.iter().cloned().filter(|c| !removed.contains(&(c.row, c.col))).collect();
            // }
            tick += 1;
        }
    }

}

#[derive(Debug, Clone, Copy)]
struct Cart {
    row:usize,
    col:usize,
    direction: usize,
    next_turn: usize,
    crash:bool
}

impl Eq for Cart {
}

impl PartialEq for Cart {
    fn eq(&self, other: &Self) -> bool {
        self.row == other.row && self.col == other.col
    }
}

impl Hash for Cart {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.row.hash(state);
        self.col.hash(state);
    }
}

// n    e->  s    w <-  \ /  
// w e  s n  e w  n s

impl Q13 {
    pub fn new() -> Self {
        Q13 {}
    }

    fn part1(&mut self) {
        let mut mine = Mine::new();
        // println!("{:?}", mine);
        mine.move_carts();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q13 {
    fn run(&mut self) {
        self.part1()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q13() {
        assert_eq!(1, 1);
    }
}