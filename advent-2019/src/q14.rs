use std::{collections::{HashMap, VecDeque}, hash::{Hash, Hasher}};

use advent_2019::{Runner, get_file};

#[derive(Debug, PartialEq, Eq, Hash)]
struct Chemical {
    name: String,
    count: i64,
}

impl Chemical {
    fn new(name: &str, count: &str) -> Self {
        let name = name.to_string();
        let count = count.parse::<i64>().unwrap();
        Self {name,count}
    }
}


#[derive(Debug)]
struct Reaction {
    chemicals: HashMap<String, (i64, Vec<Chemical>)>,
    left_overs: HashMap<String, i64>
}

impl Reaction {
    fn new() -> Self {
        let content = get_file("src/input/q14.txt").unwrap();
        let mut chemicals = HashMap::new();
        let left_overs = HashMap::new();

        content.lines().for_each(|line| {
            let (left, right) = line.split_once(" => ").unwrap();
            let (count, chemical) = right.split_once(" ").unwrap();
            let needs: Vec<Chemical>= left.split(", ").map(|pat|{
                let (count, chemical) = pat.split_once(" ").unwrap();
                Chemical::new(chemical, count)
            }).collect();
            chemicals.insert(chemical.to_string(), (count.parse::<i64>().unwrap(), needs));
            
        });
        Self {chemicals, left_overs}
    }

    fn get_basic_chemicals(&mut self, fuel:i64) -> i64 {
        let mut queue = VecDeque::from([("FUEL".to_string(), fuel)]);
        let mut chemical_tracker = HashMap::new();
        chemical_tracker.insert("FUEL".to_string(), fuel);

        while let Some((produce, required_count)) = queue.pop_front() {
            let (produce_count, needed_chemicals) = self.chemicals.get(&produce).unwrap();
            if &produce == "ORE" {
                break
            }
            // check required count  
            let mut factor = required_count / produce_count; 
            let left = required_count % produce_count; 
            
            if left != 0 {
                factor += 1;
                *self.left_overs.entry(produce.clone()).or_insert(0) += produce_count - left
            }
            chemical_tracker.remove(&produce.clone());
            if required_count < 0 {
                factor = 0;
            }
            // HVMC
            // n 130
            for chemical in needed_chemicals {
                let l = *self.left_overs.get(&chemical.name).unwrap_or(&0);
                let next_required = chemical_tracker.get(&chemical.name).unwrap_or(&0) +  (factor * chemical.count) - l;
                // if chemical.name == "QPBHG" {
                //     println!("{} {}", required_count, produce_count);
                //     println!("{:?}, {:?}", factor, chemical.count);
                //     println!("{:?} {} {} {}", chemical.name,next_required ,factor * chemical.count, l);
                // }
                self.left_overs.remove(&chemical.name);
                *chemical_tracker.entry(chemical.name.clone()).or_insert(next_required) = next_required;
                if chemical.name == "ORE".to_string() {
                    continue
                };
                if let Some(chem) = queue.iter_mut().find(|(c,_)| *c == chemical.name) {
                    chem.1 = next_required
                } else {
                    queue.push_back((chemical.name.clone(), next_required));
                }
            }
        }
        // println!("r - {:?}",chemical_tracker.get("ORE").unwrap());
        *chemical_tracker.get("ORE").unwrap()
    }
}

pub struct Q14 {

}
//  16C 4A
//  15B 21C
//  6A, 8B
//  10A, 24B, 37C
//  45 + 64 + 56
impl Q14 {
    pub fn new() -> Self {
        Q14 {}
    }

    fn part1(&mut self) {
        let mut reaction = Reaction::new();
        // println!("{:?}", reaction);
        reaction.get_basic_chemicals(2);
    }

    fn part2(&mut self) {
        let mut left: i64= 0;
        let mut right: i64 = 1000000000000;
        let ore: i64 = 1000000000000;
        let mut current_ore = 0;
        while left < right {
            let fuel = (left + right) / 2;
            let mut reaction = Reaction::new();
            current_ore = reaction.get_basic_chemicals(fuel); 
            if current_ore > ore {
                println!("{:?}", current_ore);
                println!("{:?}", fuel);
                right = fuel - 1
            } else if current_ore < ore {
                println!("{:?}", current_ore);
                println!("{:?}", fuel);
                left = fuel + 1
            } else {
                println!("{:?}", current_ore);
                println!("{:?}", fuel);
            }
        }
        println!("{:?}", current_ore);
        let mut a = 1;
        let mut b = 2;

        loop {
            let mut reaction = Reaction::new();
            if  reaction.get_basic_chemicals(b) >= ore{
                break
            }; 
            a = b;
            b = b * 2;
        }
        println!("{a} {b}");
        while b - a >= 2 {
            let mid = a + (b - a) / 2 ;
            let mut reaction = Reaction::new();
            if  reaction.get_basic_chemicals(mid) > ore{
                b = mid
            } else {
                a = mid
            }
        }
        println!("{a}")
    }
}


impl Runner for Q14 {
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
    fn Q14() {
        assert_eq!(1, 1);
    }
}