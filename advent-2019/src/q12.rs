use std::{ops::Add, cmp::Ordering, collections::{HashMap, HashSet}, hash::{Hash, Hasher}};

use advent_2019::{Runner, get_file};
use lazy_static::lazy_static;
use regex::Regex;

#[derive(Debug, Clone, Hash)]
struct Jupiter {
    moons: Vec<Moon>
}

impl Jupiter {
    fn new() -> Self {
        let content = get_file("src/input/q12.txt").unwrap();
        let moons: Vec<_>= content.lines().map(|line| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"-?\d+").unwrap();
            }
            let nums: Vec<_>= RE.find_iter(line).collect();
            let (x,y,z) = (nums[0].as_str().parse::<i64>().unwrap(),
                            nums[1].as_str().parse::<i64>().unwrap(),
                            nums[2].as_str().parse::<i64>().unwrap());
            // println!("{:?}", (x,y,z))
            Moon::new(x,y,z)
        }).collect();
        Self {moons}
    }

    // sample
    // <x=-1, y=0, z=2>
    // <x=2, y=-10, z=-7>
    // <x=4, y=-8, z=8>
    // <x=3, y=5, z=-1>
    // 2772
    fn move_moons(&mut self, final_steps:i64, to_find: Option<char>) -> i64 {
        let mut total_energy = 0;
        let mut previous = HashSet::new();
        let mut steps = 0;
        loop {
            if let Some(c) = to_find {
                let mut vec = vec![];
                for moon in self.moons.iter_mut() {
                    vec.push(*moon.pos.get(&c).unwrap());
                    vec.push(*moon.velocity.get(&c).unwrap());
                }
    
                if !previous.insert(vec.clone()){
                    println!("{:?}", vec);
                    println!("{:?}", steps);
                    return steps                
                }
            }
            // 2772 
            // 18 28 44
            for i in 0..self.moons.len() {
                for j in 0..self.moons.len() {
                    if i == j {
                        continue
                    }
                    let other_moon = self.moons[j].clone();
                    let moon = &mut self.moons[i];
                    moon.apply_gravity(other_moon);
                }
            }
    
            for moon in self.moons.iter_mut() {
                moon.apply_velocity();
                total_energy += moon.calculate_total_energy()
            }
            if steps == final_steps - 1 {
                break
            }
            total_energy = 0;
            steps += 1;
        }
        
        total_energy
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Moon {
    pos: HashMap<char,i64>,
    velocity: HashMap<char,i64>,
}

impl Hash for Moon {
    fn hash<H: Hasher>(&self, state: &mut H) {
        for c in ['x','y','z'] {
            self.pos.get(&c).unwrap().hash(state);
            self.velocity.get(&c).unwrap().hash(state);
        }
    }
}

impl Moon {
    fn new(x:i64,y:i64,z:i64) -> Self {
        let pos = HashMap::from([('x',x),('y',y),('z',z)]);
        let velocity = HashMap::from([('x',0),('y',0),('z',0)]);
        Self {pos, velocity}
    }
    
    fn apply_gravity(&mut self, other: Moon) {
        for c in ['x','y','z'] {
            match self.pos.get(&c).unwrap().cmp(other.pos.get(&c).unwrap()) {
                Ordering::Less => { *self.velocity.get_mut(&c).unwrap() +=1;}
                Ordering::Greater => {*self.velocity.get_mut(&c).unwrap() -=1}
                Ordering::Equal => {}
            }
        }
    }

    fn apply_velocity(&mut self) {
        for c in ['x','y','z'] {
            *self.pos.get_mut(&c).unwrap() += *self.velocity.get(&c).unwrap() 
        }
    }

    fn calculate_total_energy(&self) -> i64{
        let potential_enery = self.pos.iter().fold(0,|mut acc,  (_,v)| {acc += v.abs(); acc});
        let kinetic_enery = self.velocity.iter().fold(0,|mut acc,  (_,v)| {acc += v.abs(); acc});
        potential_enery * kinetic_enery
    }
}

//  10 4  => 2 10 => 0 2 => y==0: x
//  4 10 =>  10%4=2 4 => 0 2 => 

fn gcd(x:i64,y:i64) -> i64 {
    if y == 0 {x} else { gcd(y,x%y)}
}

// fn gcd(x: usize, y: usize) -> usize {
//     let mut x = x;
//     let mut y = y;
//     while y != 0 {
//         let t = y;
//         y = x % y;
//         x = t;
//     }
//     x
// }
pub struct Q12 {

}

impl Q12 {
    pub fn new() -> Self {
        Q12 {}
    }

    fn part1(&mut self) {
        let mut jupiter = Jupiter::new();
        let t = jupiter.move_moons(100, None);
        println!("{t}");
    }

    fn part2(&mut self) {
        let mut jupiter = Jupiter::new();
        let x = jupiter.move_moons(9999999, Some('x'));
        let mut jupiter = Jupiter::new();
        let y = jupiter.move_moons(9999999, Some('y'));
        let mut jupiter = Jupiter::new();
        let z = jupiter.move_moons(9999999, Some('z'));
        println!("{x} {y} {z}");
        
        // lcm should be better, but around the same
        let gcd = gcd(x,y) * gcd(y,z);
        println!("{:?}", x  * y * z / gcd);
        // println!("{:?}", ((x / gcd) * (y / gcd) * (z / gcd)))
    }

}

impl Runner for Q12 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    use super::*;
    // #[test]
    fn total_energy_after_100steps() {
        let mut jupiter = Jupiter::new();
        let total = jupiter.move_moons(100, None);
        assert_eq!(total, 1940)
    }

    #[test]
    fn individual_vec_iteration() {
        let mut jupiter = Jupiter::new();
        let x = jupiter.move_moons(100, Some('x'));
        let mut jupiter = Jupiter::new();
        let y = jupiter.move_moons(100, Some('y'));
        let mut jupiter = Jupiter::new();
        let z = jupiter.move_moons(100, Some('z'));

        println!("{x} {y} {z}");
        
        assert_eq!(1, 1)
    }

    #[test]
    fn test_gcd() {
        let x = gcd(10, 4);
        assert_eq!(x, 2);
        let x = gcd(4, 10);
        assert_eq!(x, 2);
        let x = gcd(3, 33);
        assert_eq!(x, 3);
        let x = gcd(78, 48);
        assert_eq!(x, 6);

    }
}