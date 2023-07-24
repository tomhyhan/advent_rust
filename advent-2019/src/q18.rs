use std::collections::{HashMap, HashSet, VecDeque};

use advent_2019::{Runner, get_file};
use itertools::Itertools;

struct World {
    map: HashMap<(i32,i32), char>,
    keys: HashMap<char,(i32,i32)>,
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
        let mut keys = HashMap::new();
        for (row, line) in content.lines().enumerate() {
            for (col, char) in line.chars().enumerate() {
                map.insert((row as i32,col as i32), char);
                if char == '@' {
                    starts.push((row as i32,col as i32));   
                    // starts.push((row as i32,col as i32));   
                }
                if char >= 'a' && char <= 'z' {
                    keys.insert(char, (row as i32,col as i32));
                }
            }
        }
        
        Self {map, keys, starts}
    }

    fn collect_all_keys(&self) {
        let mut visited = HashMap::new();
        let mut result = vec![];
        let mut queue = VecDeque::from([(self.starts.clone(),HashSet::new(), 0)]);
        while let Some((robots,keys,distance)) = queue.pop_front() {
            let mut sorted_keys = keys.iter().cloned().collect::<Vec<char>>(); 
            sorted_keys.sort();
            if visited.contains_key(&(robots.clone(),sorted_keys.clone())) {
                if distance >= *visited.get(&(robots.clone(),sorted_keys.clone())).unwrap() {
                    continue
                }
            }
            visited.insert((robots.clone(),sorted_keys), distance);

            if visited.len() % 10000 == 0 {
                println!("{:?} {:?}", distance, keys)
            }

            let mut key_locations = HashMap::new();
            let mut robot_queue = VecDeque::new();
            for i in 0..self.starts.len() {
                robot_queue.push_back((robots[i], i, 0))
            }
            // println!("robot_queue - {:?}", robot_queue);
            while let Some(((row,col), robot, d)) = robot_queue.pop_front() {

                if key_locations.contains_key(&(row,col)) {
                    continue
                }

                key_locations.insert((row,col), (robot, d));
                
                for dir in [(-1,0),(1,0),(0,-1),(0,1)] {
                    let (nrow,ncol) = (row+dir.0, col+dir.1);
                    let next = *self.map.get(&(nrow,ncol)).unwrap();
                    if next == '#' {
                        continue
                    }
                    if next>= 'A' && next <= 'Z' && !keys.contains(&next.to_lowercase().next().unwrap()) {
                        continue
                    }
                
                    // if robot == 0 {
                    //     println!("inner {:?}", ((nrow,ncol),robot,d+1));
                    // }
                    robot_queue.push_back(((nrow,ncol),robot,d+1))
                } 
            }   
            // println!("key_locations - {:?}", key_locations);
            for (key, (row,col)) in self.keys.iter() {
                if keys.contains(key) {
                    continue
                }
                if !key_locations.contains_key(&(*row,*col)) {
                    continue
                }
                let (robot, d) = key_locations.get(&(*row,*col)).unwrap();
                let mut new_robots = robots.clone();
                new_robots[*robot] = (*row,*col);
                let mut new_keys = keys.clone();
                new_keys.insert(*key);
                if new_keys.len() == self.keys.len() {
                    result.push(distance + d);
                    println!("{:?}", distance + d)
                }
                queue.push_back((new_robots,new_keys, distance + d));
            }
            // println!("{:?}", queue);
            // break
        }
        println!("{:?}", result.iter().min().unwrap());
    }

    
}

struct Robot {
    pos: (i32,i32),
    
}


// par1
// fn collect_all_keys(&self) {
//     let mut visited = HashSet::new();
//     let mut queue = VecDeque::from([(self.starts.0, self.starts.1,HashSet::new(), 0)]);
//     while let Some((row,col,mut keys,distance)) = queue.pop_front() {
//         let mut sorted_keys = keys.iter().cloned().collect::<Vec<_>>(); 
//         sorted_keys.sort();
//         if !visited.insert((row,col,sorted_keys)) {
//             continue
//         }
//         let current = *self.map.get(&(row,col)).unwrap();
//         if current >= 'a' && current <= 'z'{
//             keys.insert(current);
//         }
//         if keys == self.keys {
//             println!("distance - {:?}", distance);
//             break
//         }
//         for dir in [(-1,0),(1,0),(0,-1),(0,1)] {
//             let (nrow,ncol) = (row+dir.0, col+dir.1);
//             let next = *self.map.get(&(nrow,ncol)).unwrap();
//             if next == '#' {
//                 continue
//             }
//             if next>= 'A' && next <= 'Z' && !keys.contains(&next.to_lowercase().next().unwrap()) {
//                 continue
//             }
//             queue.push_back((nrow,ncol,keys.clone(),distance+1))
//         }
//     }
// }

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