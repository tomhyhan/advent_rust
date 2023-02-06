use crate::common::read_file;
use std::collections::{HashMap, HashSet, VecDeque};
use std::str::FromStr;
use std::thread;

#[derive(Debug, PartialEq, Eq)]
struct Rudolph {
    replacements: HashMap<String, Vec<String>>,
    molecules: String,
}

#[derive(Debug, PartialEq, Eq)]
struct ParsePointError;

fn find_matches(
    replacements: &HashMap<String, Vec<String>>,
    molecules: &String,
) -> HashSet<String> {
    let mut distict = HashSet::new();
    replacements.iter().for_each(|(key, values)| {
        let patterns: Vec<_> = molecules.match_indices(key).collect();
        // println!("{patterns:?}");
        patterns.iter().for_each(|pattern| {
            values.iter().for_each(|val| {
                let mut new_molecules = molecules.clone();
                // println!("{}", pattern.0);j
                // println!("{}", pattern.1);
                new_molecules.replace_range(pattern.0..(pattern.0 + pattern.1.len()), val);
                // println!("{new_molecules:?}");
                distict.insert(new_molecules);
            })
        })
    });
    distict
}

impl FromStr for Rudolph {
    type Err = ParsePointError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let split_content: Vec<_> = s.split("\n\n").collect();
        let molecules = split_content[1].to_string();

        let mut replacements = HashMap::new();

        split_content[0].lines().for_each(|line| {
            let replacement: Vec<_> = line.split("=>").map(|s| s.trim()).collect();
            let from = replacement[0].to_string();
            let to = replacement[1].to_string();
            replacements.entry(from).or_insert(Vec::new()).push(to);
        });

        Ok(Rudolph {
            replacements,
            molecules,
        })
    }
}

fn find_molecules(start: &str, rudolph: &Rudolph) {
    // enum or struct
    // let mut queue = VecDeque::from([start, 0]);
}

pub fn run() {
    // let content = read_file("q19.txt").unwrap();
    // let rudolph: Rudolph = content.parse().unwrap();

    // println!("{:?}", rudolph);

    // let d = find_matches(&rudolph.replacements, &rudolph.molecules);
    // println!("{:?}", d);
    // println!("{:?}", d.len());

    // find_molecules("e", &rudolph)
    let list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list);

    thread::spawn(|| println!("From thread: {:?}", list))
        .join()
        .unwrap();
}

// fn find_matches(str: &str, match_key: &str) -> Vec<(usize, &str)> {
//     let matches: Vec<(usize, String)> = str.match_indices(match_key).collect();
//     matches
// }

// println!("{:?}", "HOH".match_indices('H').collect::<Vec<_>>());
// println!("{:?}", "HOH".matches('H').collect::<Vec<_>>());

// let cols = "HOH".match_indices('H').collect::<Vec<_>>();

// let mut s = String::from("α is alpha, β is beta");
// let beta_offset = s.find('β').unwrap_or(s.len());
// let mut x = "HOH".to_string();
// x.replace_range(..1, "asdf");
// println!("{:?}", x);
