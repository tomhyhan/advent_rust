use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet, VecDeque},
    hash::Hash,
    thread::current,
};

use crate::Runner;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
enum Radiation {
    Chip(String),
    Generator(String),
}

impl Radiation {
    fn is_chip(&self) -> bool {
        match self {
            Radiation::Chip(s) => true,
            Radiation::Generator(s) => false,
        }
    }

    fn get_val(&self) -> String {
        match self {
            Radiation::Chip(s) => s.to_string(),
            Radiation::Generator(s) => s.to_string(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Q11 {
    floors: Vec<Vec<Radiation>>,
}

impl Q11 {
    pub fn new() -> Self {
        let mut floors = Vec::from([vec![], vec![], vec![], vec![]]);

        floors[0].append(&mut vec![
            Radiation::Chip("H".to_string()),
            Radiation::Chip("L".to_string()),
        ]);
        floors[1].append(&mut vec![Radiation::Generator("H".to_string())]);
        floors[2].append(&mut vec![Radiation::Generator("L".to_string())]);

        Q11 { floors }
    }

    fn part1(&mut self) {
        let mut stack = VecDeque::from([(0, 0, self.floors.clone())]);
        let mut visited = HashSet::new();

        // println!("{:?}", stack);
        while stack.len() > 0 {
            let (pos, steps, floors) = stack.pop_front().unwrap();
            // println!("{pos:?}");
            // if pos == 3 {
            //     println!("steps - {steps:?}, floors - {:?}", floors[3]);
            //     // break;
            // }

            // println!("{pos}");

            if visited.contains(&floors) {
                continue;
            }
            visited.insert(floors.clone());

            let c_floor = floors[pos].clone();

            let chips: Vec<_> = c_floor
                .clone()
                .into_iter()
                .filter(|p| p.is_chip())
                .collect();
            let generators: Vec<_> = c_floor
                .clone()
                .into_iter()
                .filter(|p| !p.is_chip())
                .collect();
            // println!("asdf");

            // if generators.len() > 0 && fried(chips, generators) {
            //     continue;
            // }

            // move either gen | chip randomly using combination 1, 2
            //
            // println!("pos - {pos}");
            let current = floors[pos].clone();
            match pos {
                0 => {
                    for n in 1..3 {
                        for mut comb in current.clone().into_iter().combinations(n) {
                            let mut floors = floors.clone();
                            floors[0] = c_floor
                                .clone()
                                .into_iter()
                                .filter(|p| !comb.contains(&p))
                                .collect();
                            floors[1].append(&mut comb);
                            // println!("{floors:?}");
                            stack.push_back((pos + 1, steps + 1, floors.clone()))
                        }
                    }
                }
                1 | 2 => {
                    for n in 1..3 {
                        for mut comb in current.clone().into_iter().combinations(n) {
                            let mut floors = floors.clone();

                            floors[pos] = c_floor
                                .clone()
                                .into_iter()
                                .filter(|p| !comb.contains(&p))
                                .collect();
                            floors[pos + 1].append(&mut comb);
                            // println!("{floors:?}");

                            stack.push_back((pos + 1, steps + 1, floors.clone()))
                        }
                    }

                    for n in 1..3 {
                        for mut comb in current.clone().into_iter().combinations(n) {
                            let mut floors = floors.clone();
                            floors[pos] = c_floor
                                .clone()
                                .into_iter()
                                .filter(|p| !comb.contains(&p))
                                .collect();
                            floors[pos - 1].append(&mut comb);
                            stack.push_back((pos - 1, steps + 1, floors.clone()))
                        }
                    }
                }
                3 => {
                    if pos == 3 && floors[3].len() == 4 {
                        println!("found!!");
                        println!("{steps} {:?}", floors[2])
                    }
                    for n in 1..3 {
                        for mut comb in current.clone().into_iter().combinations(n) {
                            let mut floors = floors.clone();
                            floors[pos] = c_floor
                                .clone()
                                .into_iter()
                                .filter(|p| !comb.contains(&p))
                                .collect();
                            floors[pos - 1].append(&mut comb);
                            // println!("{floors:?}");
                            stack.push_back((pos - 1, steps + 1, floors.clone()))
                        }
                    }
                    // break;
                }
                _ => panic!("invalid floor"),
            }
        }
        // let visited = HashSet::new();
    }
}

fn fried(chips: Vec<Radiation>, generators: Vec<Radiation>) -> bool {
    let fried_chips: Vec<_> = chips
        .iter()
        .filter(|c| {
            if generators
                .iter()
                .find(|g| c.get_val() == g.get_val())
                .is_none()
            {
                return true;
            }
            false
        })
        .collect();
    // println!("{fried_chips:?}");
    if fried_chips.len() > 0 {
        true
    } else {
        false
    }
}

impl Runner for Q11 {
    fn run(&mut self) -> () {
        self.part1();
    }
}

// let test_floor = self.floors.clone();
// self.floors
//     .get_mut(&3)
//     .unwrap()
//     .push(Radiation::Chip("A".to_string()));
// self.floors.get_mut(&3).unwrap().pop();

// println!("{:?}", test_floor == self.floors);

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn it_should_not_return_fried_chips() {
//         let a = Radiation::Chip("A".to_string());
//         let b = Radiation::Chip("B".to_string());
//         let chips = vec![&a, &b];
//         let c = Radiation::Generator("B".to_string());
//         let d = Radiation::Generator("C".to_string());
//         let gens = vec![&c, &d];

//         assert_eq!(fried(chips, gens), true)
//     }

//     #[test]
//     fn it_should_return_fried_chips() {
//         let a = Radiation::Chip("A".to_string());
//         let b = Radiation::Chip("B".to_string());
//         let chips = vec![&a, &b];
//         let c = Radiation::Generator("A".to_string());
//         let d = Radiation::Generator("B".to_string());
//         let gens = vec![&c, &d];

//         assert_eq!(fried(chips, gens), false)
//     }
// }
