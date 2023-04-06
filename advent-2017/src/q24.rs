use std::{collections::HashSet, hash::{Hash, Hasher}};

use advent_2017::{Runner, get_file};

pub struct Q24 {

}

struct Bridge {
    components: Vec<Magnetic>
}

fn strongest(components: Vec<Magnetic>, start: i32) -> i32{
    let mut strong = 0;

    for i in 0..components.len() {
        if let Some(next) = components[i].find_next(start) {
            let mut new_components = components.clone();
            new_components.remove(i);
            let length = components[i].strength() + strongest(new_components, next);
            strong = strong.max(length)
        } 
    };
    strong
}


impl Bridge {
    fn new() -> Self {
        let content = get_file("q24.txt").unwrap();
        let mut components = vec![];
        content.lines().for_each(|line| {
            let (left, right) = line.split_once('/').unwrap();
            let one = Port {pins: left.parse::<i32>().unwrap(), used: false};
            let two = Port {pins: right.parse::<i32>().unwrap(), used: false};
            components.push(Magnetic {one,two})
        });
        Bridge { components }
    }

    fn find_zero_port(&self) -> Vec<Magnetic>{
        self.components.iter().cloned().filter(|p| p.one.pins == 0 || p.two.pins == 0).collect()
    }
    
    // method 2
    fn find_strongest_bridge(&mut self) -> i32 { 
        let strongest = strongest(self.components.clone(), 0);
        strongest
    }

    // method 1
    // fn find_strongest_bridge(&mut self) {
    //     let mut start_components = self.find_zero_port();

    //     for start_component in start_components.iter_mut() {
    //         if start_component.one.pins == 0 { start_component.one.used = true} else {
    //             start_component.two.used = true}

    //         let visited = HashSet::new();
    //         let str = self.find_strength(&start_component);
    //         let mut max = 0;
    //         let mut longest_bridge = 0;

    //         let mut stack = vec![(start_component.clone(),self.components.clone(), visited.clone(), str)];

    //         while let Some((mut magnatic, mut components, mut visited, str)) = stack.pop() {
                
    //             if visited.contains(&magnatic) {
    //                 continue
    //             };

    //             visited.insert(magnatic.clone());

    //             if visited.len() == 40 {
    //                 println!("{:?}",str)
    //             }
    //             let pins = self.find_unused_port(&mut magnatic);

    //             for magnet in components.iter_mut() {
    //                 if magnet.one.pins == 0 || magnet.two.pins == 0 {
    //                     continue
    //                 }
    //                 if visited.contains(&magnet) {
    //                     continue
    //                 };
    //                 if self.find_matching_pairs(magnet, pins) {
    //                     let new_str = self.find_strength(magnet);
    //                     let str = new_str + str;
    //                     max = max.max(str); 
    //                     stack.push((magnet.clone(), self.components.clone(), visited.clone(), str))
    //                 } else {
    //                     longest_bridge = longest_bridge.max(visited.len());
    //                 }
    //             }
    //         }
    //         println!("longest_bridge - {:?}", longest_bridge);
    //         println!("{:?}", max);
    //     }
    // }

    fn find_matching_pairs(&self, magnet: &mut Magnetic, pins: i32) -> bool{
        if magnet.one.pins == pins {
            magnet.one.used = true;
            return true
        } else if magnet.two.pins == pins {
            magnet.two.used = true;
            return true
        };
        false
    }

    fn find_strength(&self, magnetic: &Magnetic) -> i32 {
        magnetic.one.pins + magnetic.two.pins 
    }

    fn find_unused_port(&self, magnet: &mut Magnetic ) -> i32{
        if magnet.one.used {
            magnet.two.used = true;
            magnet.two.pins
        } else {
            magnet.one.used = true;
            magnet.one.pins 
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Magnetic {
    one: Port,
    two: Port
}

impl Magnetic {
    fn find_next(&self, value: i32) -> Option<i32> {
        if self.one.pins == value {
            return Some(self.two.pins)
        } else if self.two.pins == value {
            return Some(self.one.pins)
        } else {
            None
        }
    }
    fn strength(&self) -> i32{
        self.one.pins + self.two.pins
    }
}

impl PartialEq for Magnetic {
    fn eq(&self, other: &Self) -> bool {
        self.one.pins == other.one.pins && self.two.pins == other.two.pins
    }
}

impl Hash for Magnetic {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.one.pins.hash(state);
        self.two.pins.hash(state);
    }
}


#[derive(Debug, Clone, PartialEq, Eq)]
struct Port {
    pins: i32,
    used: bool
}

impl Hash for Port {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.pins.hash(state);
    }
}


impl Q24 {
    pub fn new() -> Self {
        Q24 {}
    }

    fn part1(&mut self) {
        let mut bridge = Bridge::new();
        // println!("{:?}", bridge.components)
        let r = bridge.find_strongest_bridge();
        println!("{:?}", r);
    }
}

impl Runner for Q24 {
    fn run(&mut self) {
        self.part1()
    }
}