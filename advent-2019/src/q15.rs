use std::collections::{HashMap, HashSet, VecDeque};
use lazy_static::lazy_static;
use advent_2019::Runner;

use crate::int_program::Program;

lazy_static! {
    static ref DIRECTIONS: HashMap<i64, (i64,i64)>=  HashMap::from([(1, (-1,0)), (2, (1,0)),(3, (0,-1)),(4, (0,1))]);
}

struct System {
    map: HashMap<(i64,i64), Area>,
    min_row: i64,
    max_row: i64,
    min_col: i64,
    max_col: i64
}

impl System {
    fn new() -> Self{
        let map = HashMap::new();
        let min_row = i64::MAX;
        let max_row = i64::MIN;
        let min_col = i64::MAX;
        let max_col = i64::MIN;
        Self {map,min_row,
            max_row,
            min_col,
            max_col,}
    }

    fn search_area(&mut self, program: Program) {
        let mut visited = HashSet::new();
        let mut queue = VecDeque::from([((0,0), program.clone(), 0)]);

        'outer: while let Some((droid_pos, current_program, distance)) = queue.pop_front() {
            if !visited.insert(droid_pos) {
                continue
            };

            for command in 1..5 {
                let mut new_program = current_program.clone();
                let dir = DIRECTIONS.get(&command).unwrap();
                let next_pos = (droid_pos.0 + dir.0, droid_pos.1 + dir.1); 

                self.min_row = self.min_row.min(next_pos.0);
                self.max_row = self.max_row.max(next_pos.0);
                self.min_col = self.min_col.min(next_pos.1);
                self.max_col = self.max_col.max(next_pos.1);
                let is_output = new_program.run_program(command);
                if let Some(output) = is_output {
                    match output {
                        0 => {
                            self.map.insert(next_pos, Area::Wall);
                            continue
                        },
                        2 => {
                            println!("oxygem distance - {:?}", distance + 1);
                            self.map.insert(next_pos, Area::Oxygen);
                            continue
                        },
                        1 => {
                            self.map.insert(next_pos, Area::Empty);
                            queue.push_back((next_pos, new_program, distance + 1))
                        }
                        _ => panic!("invalid output")
                    }
                }
            }
        }
    }

    fn fill_oxygen(&mut self) {
        let oxygen_location = self.get_oxygen_location();
        let mut time = 0;
        let mut queue = VecDeque::from([oxygen_location]); 
        while !queue.is_empty() {
            let mut current_queue = queue.clone();
            queue = VecDeque::new();
            while let Some(pos) = current_queue.pop_front() {
                self.map.insert(pos, Area::Oxygen);
                for dir in [(-1,0),(1,0),(0,-1),(0,1)] {
                    let new_pos = (pos.0 + dir.0, pos.1 + dir.1);
                    let next_area = self.map.get(&new_pos).unwrap();
                    if *next_area == Area::Empty {
                        queue.push_back(new_pos);
                    }
                }
            }
            time += 1;
        }
        println!("{:?}", time-1);
    }

    fn get_oxygen_location(&self) -> (i64,i64) {
        *self.map.iter().find(|(_, &area)| area == Area::Oxygen ).unwrap().0
    }

    fn draw_map(&self) {
        // let rowcol = self.get_row_col();
        for row in self.min_row..=self.max_row {
            let mut s = String::new();
            for col in self.min_col..=self.max_col {
                if let Some(c) = self.map.get(&(row,col)) {
                    match *c {
                        Area::Empty => s.push('.'),
                        Area::Wall => s.push('#'),
                        Area::Oxygen => s.push('O')
                    };
                } else {
                    s.push('.')
                }
            }
            println!("{:?}", s);
        }
    } 

}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Area {
    Wall,
    Empty,
    Oxygen
}

pub struct Q15 {

}

impl Q15 {
    pub fn new() -> Self {
        Q15 {}
    }

    fn part1(&mut self) {
        let mut system = System::new();
        let program = Program::new();
        system.search_area(program);
        system.draw_map();
        system.fill_oxygen();
        system.draw_map();
    }
    

    fn part2(&mut self) {
        
    }

}

impl Runner for Q15 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q15() {
        assert_eq!(1, 1);
    }
}