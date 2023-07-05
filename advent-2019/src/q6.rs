use std::{collections::HashMap};

use advent_2019::{Runner, get_file};


// 1 2 3 2 3 4 4 5 5 6 7
struct Map {
    orbits: HashMap<String, Vec<String>>,
    parents: HashMap<String, String>
}

impl Map {
    fn new() -> Self {
        let content = get_file("src/input/q6.txt").unwrap();
        let orbits = content.lines().fold(HashMap::new(), |mut acc, orbit| {
            let (parent,child) = orbit.split_once(')').unwrap();
            acc.entry(parent.to_string()).or_insert(vec![]).push(child.to_string());
            acc
        });
        let parents = content.lines().fold(HashMap::new(), |mut acc, orbit| {
            let (parent,child) = orbit.split_once(')').unwrap();
            *acc.entry(child.to_string()).or_insert("".to_string()) = parent.to_string();
            acc
        });
        Self {orbits, parents}
    }

    fn count_orbits(&mut self, start: String) {
        println!("{:?}", Map::dfs(&start, 0, &self.orbits));
    }

    fn min_traverse_distance(&self, start:String) {
        let you = Map::dfs_to_target(&start, 0, &self.orbits, &"YOU".to_string());
        let san = Map::dfs_to_target(&start, 0, &self.orbits, &"SAN".to_string());

        let mut distance = 0;
        distance += (you as i32 - san as i32).abs() as usize;
        
        let (mut current, target, target_depth,mut seek_depth) = if san > you {
            ("SAN".to_string(),"YOU".to_string(),you, san)
        } else {
            ("YOU".to_string(),"SAN".to_string(),san, you)
        };
        
        while target_depth != seek_depth {
            current = self.find_parent(&mut current); 
            seek_depth -= 1;
        }

        let mut target_parent = self.parents.get(&target).unwrap();
        let mut current_parent = self.parents.get(&current).unwrap();
        while target_parent != current_parent {
            target_parent = self.parents.get(target_parent).unwrap();
            current_parent = self.parents.get(current_parent).unwrap();
            distance += 2;
        }

        println!("{:?}", distance);
    }

    fn find_parent(&self, current: &mut String) -> String {
        self.parents.get(current).unwrap().clone()
    }

    fn dfs_to_target(orbit:&String, depth: usize, orbits: &HashMap<String, Vec<String>>, target: &String) -> usize {
        if orbit == target {
            return depth
        }
        if !orbits.contains_key(orbit) {
            return 0
        }

        let mut num_of_orbits = 0;
        for child_orbit in orbits.get(orbit).unwrap()  {
            num_of_orbits += Map::dfs_to_target(&child_orbit, depth + 1, orbits, target);
        }
        num_of_orbits
    }

    fn dfs(orbit:&String, depth: usize, orbits: &HashMap<String, Vec<String>>) -> usize{
        if !orbits.contains_key(orbit) {
            return depth
        }
        let mut num_of_orbits = depth;

        for child_orbit in orbits.get(orbit).unwrap()  {
            num_of_orbits += Map::dfs(&child_orbit, depth + 1, orbits);
        }
        num_of_orbits
    }
}


pub struct Q6 {

}

impl Q6 {
    pub fn new() -> Self {
        Q6 {}
    }

    fn part1(&mut self) {
        let mut map = Map::new();
        map.count_orbits("COM".to_string());
    }

    fn part2(&mut self) {
        let mut map = Map::new();
        map.min_traverse_distance("COM".to_string())
    }

}

impl Runner for Q6 {
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
    fn Q6() {
        assert_eq!(1, 1);
    }
}