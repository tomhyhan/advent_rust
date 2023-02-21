use itertools::Itertools;
use std::{
    collections::{HashMap, HashSet},
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
    fn get_rad(&self) -> bool {
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
    floors: HashMap<usize, Vec<Radiation>>,
}

impl Q11 {
    pub fn new() -> Self {
        let mut floors = HashMap::new();

        floors.insert(
            0,
            vec![
                Radiation::Chip("H".to_string()),
                Radiation::Chip("L".to_string()),
            ],
        );
        floors.insert(1, vec![Radiation::Generator("H".to_string())]);
        floors.insert(2, vec![Radiation::Generator("L".to_string())]);
        floors.insert(3, vec![]);

        Q11 { floors }
    }

    fn part1(&mut self) {
        let mut stack = Vec::from([(0, 0, self.floors.clone())]);
        let mut visited = HashSet::new();

        // println!("{:?}", stack);
        while stack.len() > 0 {
            let (current_pos, cnt, current_floors) = stack.pop().unwrap();

            let check_floor: Vec<_> = current_floors.clone().into_iter().map(|(k, v)| v).collect();
            if visited.contains(&check_floor) {
                continue;
            }
            visited.insert(check_floor.clone());

            let c_floor = current_floors.get(&current_pos).unwrap();

            let chips: Vec<_> = c_floor.into_iter().filter(|p| p.get_rad()).collect();
            let generators: Vec<_> = c_floor.into_iter().filter(|p| !p.get_rad()).collect();

            if fried(chips, generators) {
                continue;
            }

            if current_pos == 4 && current_floors.get(&4).unwrap().len() == 4 {
                println!("{cnt}");
                break;
            }

            // move either gen | chip randomly using combination 1, 2
            //
            match current_pos {
                1 => {
                    for n in 1..3 {
                        current_floors
                    }
                }
                2 | 3 => (),
                4 => (),
                _ => panic!("invalid floor"),
            }
        }
        // let visited = HashSet::new();
    }
}

fn fried(chips: Vec<&Radiation>, generators: Vec<&Radiation>) -> bool {
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
    println!("{fried_chips:?}");
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_should_not_return_fried_chips() {
        let a = Radiation::Chip("A".to_string());
        let b = Radiation::Chip("B".to_string());
        let chips = vec![&a, &b];
        let c = Radiation::Generator("B".to_string());
        let d = Radiation::Generator("C".to_string());
        let gens = vec![&c, &d];

        assert_eq!(fried(chips, gens), true)
    }

    #[test]
    fn it_should_return_fried_chips() {
        let a = Radiation::Chip("A".to_string());
        let b = Radiation::Chip("B".to_string());
        let chips = vec![&a, &b];
        let c = Radiation::Generator("A".to_string());
        let d = Radiation::Generator("B".to_string());
        let gens = vec![&c, &d];

        assert_eq!(fried(chips, gens), false)
    }
}
