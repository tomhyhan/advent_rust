use std::collections::{HashMap, HashSet, VecDeque};

use advent_2019::{Runner, get_file};
use itertools::Itertools;

struct World {
    map: HashMap<(i32,i32), char>,
    keys: HashSet<char>,
    starts: Vec<(i32,i32)>    
}
// search area
// iterate over known nodes
//  8 * 8 * 
impl World {
    fn new() -> Self {
        let content = get_file("src/input/q18.txt").unwrap();
        let mut map = HashMap::new();
        let mut starts= vec![];
        let mut keys = HashSet::new();
        for (row, line) in content.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                map.insert((row as i32,col as i32), char);
                if char == '@' {
                    starts.push((row as i32,col as i32));   
                }
                if char >= 'a' && char <= 'z' {
                    keys.insert(char);
                }
            }
        }
        
        Self {map, keys, starts}
    }

    fn collect_all_keys(&self) {
        loop {
            for start in self.starts.iter() {

            }
        }
    }

    
}

struct Robot {
    pos: (i32,i32),
    
}


// let mut queue = VecDeque::from([(self.start.0,self.start.1,HashSet::new(),0)]);
//         println!("{:?} {:?}", self.keys, self.start);
//         let mut visited = HashSet::new();
//         while let Some((row,col,mut keys,distance)) = queue.pop_front() {
//             let mut seen = keys.iter().cloned().collect::<Vec<char>>();
//             seen.sort();
//             if !visited.insert((row,col,seen)) {
//                 continue
//             }
//             let current = self.map.get(&(row,col)).unwrap();
//             if *current >= 'a' && *current <= 'z'  {
//                 keys.insert(*current);
//             }
//             if keys == self.keys {
//                 println!("distance - {distance}");
//                 break
//             }
//             for dir in [(-1,0),(1,0),(0,-1),(0,1)] {
//                 let (nrow, ncol) = (row + dir.0, col + dir.1);
//                 let next = *self.map.get(&(row,col)).unwrap();
//                 if next == '#' {
//                     continue
//                 }
//                 if next >= 'A' && next <= 'Z' && !keys.contains(&next.to_lowercase().next().unwrap()) {
//                     continue
//                 }
//                 queue.push_back((nrow,ncol,keys.clone(),distance+1));
//             }
//         }   

pub struct Q18 {

}

impl Q18 {
    pub fn new() -> Self {
        Q18 {}
    }

    fn part1(&mut self) {
        let world = World::new();
        world.collect_all_keys();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q18 {
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
    fn Q18() {
        assert_eq!(1, 1);
    }
}