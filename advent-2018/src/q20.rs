use std::collections::HashMap;

use advent_2018::{Runner, get_file};

//  problem
//  1. draw the map
//  2. find steps to get to the furthest room
//  requirements
//  1. input - Vec<char>
//  2. map (output) - HashMap
//  3. furthest_room - i32
//  design 
//  1. 

//  ^ENWWW(NEEE|SSE(EE|N))NNN$
//  ^ENWWW(NEEE|SSE)NNN$

struct Construction {
    map: HashMap<(i32,i32), i32>,
    regex: Vec<char>
}

impl Construction {
    fn new() -> Self {
        let content = get_file("src/input/q20.txt").unwrap();
        let regex = content.chars().collect();
        let mut map = HashMap::new();
        map.insert((0,0), 0);
        Self {map, regex}
    }

    fn draw(&mut self) {
        let mut current_pos = (0,0);
        let mut stack: Vec<(i32,i32)> = Vec::new();
        let dirs = HashMap::from([('N',(-1,0)),('S',(1,0)),('W',(0,-1)),('E',(0,1))]);
        for c in self.regex.iter() {
            match c {
                '^' => continue,
                'N' | 'S' | 'E' | 'W' => {
                    let (row, col) = current_pos;            
                    let (dir_row, dir_col) = dirs.get(c).unwrap();
                    current_pos = (row + *dir_row, col + *dir_col);
                    let current_steps = *self.map.get(&(row,col)).unwrap();
                    if let Some(step) = self.map.get_mut(&current_pos) {
                        if *step > current_steps + 1 {
                            *step = current_steps + 1;
                        } 
                    } else {
                        self.map.insert(current_pos, current_steps + 1);
                    }
                },
                '(' => {
                    stack.push(current_pos);
                },
                '|' => {
                    current_pos = *stack.last().unwrap();
                },
                ')' => {
                    stack.pop();
                },
                '$' => return,
                _ => panic!("unknonw char")
            }
        }
    }
}


pub struct Q20 {

}

impl Q20 {
    pub fn new() -> Self {
        Q20 {}
    }

    fn part1(&mut self) {
        let mut con = Construction::new();
        con.draw();
        // part1
        println!("{:?}", con.map.iter().max_by_key(|(k,v)|*v).unwrap());
        // part2
        println!("{:?}", con.map.iter().filter(|(k,&v)|v >= 1000).count());
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q20 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q20() {
        assert_eq!(1, 1);
    }
}