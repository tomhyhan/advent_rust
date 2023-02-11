use itertools::Itertools;
use std::{fs, str::FromStr};

#[derive(Debug)]
struct Present {
    weights: Vec<i64>,
}

impl FromStr for Present {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let weights: Vec<i64> = s.lines().map(|num| num.parse::<i64>().unwrap()).collect();

        Ok(Present { weights })
    }
}

fn find_possible_balance(present: &Present) {
    let needed_weight: i64 = present.weights.iter().copied().sum::<i64>() / 4;

    for n in 1..7 {
        let sets = present
            .weights
            .clone()
            .into_iter()
            .combinations(n)
            .filter(|p| p.iter().sum::<i64>() == needed_weight)
            .map(|s| s.iter().copied().reduce(|acc, e| acc * e).unwrap())
            .min()
            .unwrap_or_else(|| 0);
        println!("{sets:?}")
    }
}
fn main() {
    let path = "input.txt";
    let content = fs::read_to_string(path).expect("file cannot be read");

    let presents: Present = content.parse().unwrap();
    println!("{presents:?}");

    find_possible_balance(&presents)
}
