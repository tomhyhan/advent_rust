use lazy_static::lazy_static;
use regex::Regex;
use std::cmp::max;
use std::collections::{HashMap, HashSet, VecDeque};
use std::fs;

#[derive(Debug)]
struct Destination {
    place: String,
    distance: usize,
}

#[derive(Debug)]
struct LocationInfo {
    place: String,
    distance: usize,
    visited: HashSet<String>,
}

impl LocationInfo {
    fn add(&mut self, location: String) {
        self.visited.insert(location);
    }
}

fn parse(line: &str, maps: &mut HashMap<String, Vec<Destination>>) {
    lazy_static! {
        static ref SPLIT_MATCHES: Regex = Regex::new(r"to|=").unwrap();
    };
    let cities: Vec<_> = SPLIT_MATCHES
        .split(line)
        .map(|w| w.trim().to_string())
        .collect();

    let cityone = cities[0].clone();
    let citytwo = cities[1].clone();
    let distance: usize = cities[2].parse().unwrap();

    maps.entry(cityone).or_insert(Vec::new()).push(Destination {
        place: citytwo,
        distance,
    });
    let cityone = cities[0].clone();
    let citytwo = cities[1].clone();

    maps.entry(citytwo).or_insert(Vec::new()).push(Destination {
        place: cityone,
        distance,
    });
    println!("{:?}", cities);
}
fn main() {
    // traverse through all the nodes
    // if we cannot travel to all the nodes, then return
    // if we did travel through all the nodes, save the shortest distance
    // if current travel distance is more than shortest distance, then return

    let mut maps: HashMap<String, Vec<Destination>> = HashMap::new();

    // maps.entry("afaf").or_insert(Vec::new()).push(2);
    // maps.entry("afaf").or_insert(Vec::new()).push(3);
    // maps.get("afaf").clone().unwrap();
    // println!("{:?}", maps.get_mut("afaf").unwrap().push(5));
    // println!("{:?}", maps);
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("read failed");
    context.lines().for_each(|line| parse(line, &mut maps));

    println!("{maps:?}");

    // maps.into_keys()
    //     .for_each(|location| travel(location.to_string(), &maps));
    for location in maps.keys() {
        travel(location.to_string(), &maps)
    }
}

fn travel(location: String, maps: &HashMap<String, Vec<Destination>>) {
    // let start = maps.get(&location).unwrap();

    let mut stack: VecDeque<_> = [LocationInfo {
        place: location,
        distance: 0,
        visited: HashSet::new(),
    }]
    .into();

    let mut shortest = usize::MIN;
    let number_of_cities = maps.keys().count();

    while stack.len() > 0 {
        let mut current_location = match stack.pop_front() {
            Some(v) => v,
            None => break,
        };

        // println!("{current_location:?}");

        if current_location.visited.contains(&current_location.place) {
            continue;
        }
        current_location.add(current_location.place.to_string());

        if current_location.visited.len() == number_of_cities {
            shortest = max(current_location.distance, shortest);
            continue;
        }

        for city in maps.get(&current_location.place).unwrap() {
            // println!("{:?}", city);
            if current_location.visited.contains(&city.place.to_string()) {
                continue;
            }
            // if city.distance + current_location.distance < shortest {
            //     continue;
            // }
            stack.push_front(LocationInfo {
                place: city.place.to_string(),
                distance: city.distance + current_location.distance,
                visited: current_location.visited.clone(),
            })
        }
    }
    println!("{shortest}")
}
