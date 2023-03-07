use std::{collections::{HashSet, HashMap}, cmp::min};
use itertools::Itertools;
use crate::{Runner, common::my_modules::get_file};
use std::cmp;
pub struct Q24 {
}

struct Roof {
    map: Vec<Vec<char>>
}

impl Roof {
    pub fn new() -> Self {
        let content = get_file("q24.txt").unwrap();
        
        let map = content.lines().map(|line| line.chars().collect()).collect();
        
        Roof {
            map
        }
    }

    fn find_location(&self, num: char) -> Option<(usize,usize)> {
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col] == num {
                    return Some((row,col))
                }
            }
        }
        None
    }

    fn find_numbers(&self) -> Vec<char> {
        let mut nums = vec![];
        for row in 0..self.map.len() {
            for col in 0..self.map[0].len() {
                if self.map[row][col].is_digit(10) {
                    nums.push(self.map[row][col]);
                }
            }
        };
        nums
    }

    fn find_neighbors(&self, location : (usize,usize)) -> Vec<(usize,usize)>{
        let mut neighbors = vec![];

        for direction in [(0,1),(0,-1),(1,0),(-1,0)] {
            let new_row = (location.0 as i32 + direction.0) as usize;
            let new_col = (location.1 as i32 + direction.1) as usize;
            if self.map[new_row][new_col] != '#' {
                neighbors.push((new_row,new_col))
            }
        };
        neighbors
    }
}

impl Q24 {
    pub fn new() -> Self {
        Q24 {}
    }

    fn part1(&self) {
        
        let roof = Roof::new();
        let nums = roof.find_numbers();
        let length = nums.len();

        // method 2: getting shortest distance (hashmap by precomputing the distance btw each node)
        let mut distance = HashMap::new();
        nums.iter().combinations(2).for_each(|comb| {
            let start = roof.find_location(*comb[0]).unwrap();
            let end = roof.find_location(*comb[1]).unwrap();
            let distance_btw = djikstra(&roof, start, end);
            distance.insert((*comb[0],*comb[1]), distance_btw);
            distance.insert((*comb[1],*comb[0]), distance_btw);
        });

        let posible_ways : Vec<_> = nums.into_iter().permutations(length).filter(|p| p[0] == '0').map(|mut perms| {
            perms.push('0');
            perms
        }).collect();

        let mut shortest_distance = i32::MAX;

        for perm in posible_ways {
            let mut total_distance = 0;
            for location in perm.windows(2) {
                total_distance += distance.get(&(location[0], location[1])).unwrap()
            }
            shortest_distance = cmp::min(shortest_distance, total_distance)
        }
        println!("{:?}",shortest_distance);

        // method 1: getting shortest distance (brute force)
        // let posible_ways : Vec<_> = nums.into_iter().permutations(length).filter(|p| p[0] == '0').map(|mut perms| {
        //     perms.push('0');
        //     perms
        // }).collect();

        // let mut shortest_distance = i32::MAX;
        // for perms in posible_ways.into_iter() {
        //     let mut total_distance = 0;
        //     for location_info in perms.windows(2) {
        //         let start = roof.find_location(location_info[0]).unwrap();
        //         let end = roof.find_location(location_info[1]).unwrap();
        //         total_distance += djikstra(&roof, start, end);
        //         if total_distance > shortest_distance {
        //             break
        //         }

        //     };
        //     // println!("{:?}", total_distance);
        //     shortest_distance = min(shortest_distance, total_distance);
        //     println!("{:?}",perms);
        //     println!("{:?}", shortest_distance);
        // }
        
    }
}

// dikstra 1: always get the min distance in our queue
fn djikstra(roof: &Roof, start : (usize,usize), end : (usize,usize)) -> i32 {
    let mut queue = HashSet::new();
    let mut dist = HashMap::new();
    let mut visited = HashSet::new();

    queue.insert(start);
    dist.insert(start, 0);

    loop {
        let current = queue
        .iter().map(|location| (location, dist.get(&location).unwrap()))
        .min_by(|x,y| x.1.cmp(y.1))
        .unwrap()
        .0
        .clone();

        if current == end {
            break
        } 

        if !queue.remove(&current) {
            panic!("BEEP!!");
        }

        for neighbor in roof.find_neighbors(current) {
            let n = 
            if queue.contains(&neighbor) {neighbor} 
            else if visited.insert(neighbor) {
                dist.insert(neighbor.clone(), i32::MAX);
                queue.insert(neighbor.clone());
                neighbor
            }
            else{ continue;};
        
            let current_distance = dist.get(&current).unwrap();
            let distance = dist.get(&n).unwrap();
            if *current_distance + 1 < *distance {
                dist.insert(n.clone(), *current_distance + 1);
            }
        }
    }

   *dist.get(&end).unwrap()

}

impl Runner for Q24 {
    fn run(&mut self) -> () {
        self.part1();
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn find_correct_location() {
        let roof = Roof::new();

        assert_eq!(roof.find_location('0').unwrap(), (27,177))
    }
}