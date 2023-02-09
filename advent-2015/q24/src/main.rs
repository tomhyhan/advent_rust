use itertools::Itertools;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Present {
    weights: Vec<i32>,
}

impl FromStr for Present {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let weights: Vec<i32> = s.lines().map(|num| num.parse::<i32>().unwrap()).collect();

        Ok(Present { weights })
    }
}

fn find_possible_balance(present: &Present) {
    // let mut possible = Vec::new();

    let sets: Vec<_> = present.weights.clone().into_iter().powerset().collect();

    let min_quantum: Vec<_> = present.weights.iter().copied().combinations(2).collect();

    // for i in 0..sets.len() {
    //     for j in i + 1..sets.len() {
    //         if sets[i].iter().sum::<i32>() == sets[j].iter().sum::<i32>() {
    //             possible.push((sets[i].clone(), sets[j].clone()))
    //         }
    //     }
    // }

    println!("{min_quantum:?}")
    // for idx in 0..present.weights.len() {
    //     if present.weights[0..idx + 1].iter().sum()
    //         <= present.weights[idx + 1..present.weights.len()].iter().sum()
    //     {}
    // }
}
fn main() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("file cannot be read");

    let presents: Present = content.parse().unwrap();
    println!("{presents:?}");

    find_possible_balance(&presents)
}
