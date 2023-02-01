use std::cmp::{max, min};
use std::fs;
// Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3
// for comb in self.ingredients.keys().combinations_with_replacement(100) {
// for (num, i) in comb.iter().dedup_with_count() {

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

fn parse(line: &str, array: &mut Vec<Ingredient>) {
    let info: Vec<_> = line.split(" ").map(|w| w.trim_matches(',')).collect();
    println!("{:?}", info);
    array.push(Ingredient {
        capacity: info[2].parse::<i64>().unwrap(),
        durability: info[4].parse::<i64>().unwrap(),
        flavor: info[6].parse::<i64>().unwrap(),
        texture: info[8].parse::<i64>().unwrap(),
        calories: info[10].parse::<i64>().unwrap(),
    })
}

fn get_scores(Ingredients: &Vec<Ingredient>) -> i64 {
    let mut max_score = i64::MIN;

    for c in 1..=100 {
        let d = 100 - c;
        let capacity = max(Ingredients[0].capacity * c + Ingredients[1].capacity * d, 0);
        let dura = max(
            Ingredients[0].durability * c + Ingredients[1].durability * d,
            0,
        );
        let flavor = max(Ingredients[0].flavor * c + Ingredients[1].flavor * d, 0);
        let texture = max(Ingredients[0].texture * c + Ingredients[1].texture * d, 0);
        // for f in 1..(100 - c - d) {
        //     for c in 1..(100 - c - d - f) {
        //         let mut score = 1;

        //         for ingredient in Ingredients {
        //           ingredient
        //         }
        //     }
        // }
        // println!("{}", capacity);
        // println!("{}", dura);
        // println!("{}", flavor);
        // println!("{}", texture);
        // println!("{}", capacity * dura * flavor * texture);
        max_score = max(max_score, capacity * dura * flavor * texture);
        // break;
    }
    max_score
}

pub fn run_code() {
    let path = "q15.txt";
    let context = fs::read_to_string(path).expect("fail to read a file");

    let mut Ingredients = Vec::new();
    context
        .lines()
        .for_each(|line| parse(line, &mut Ingredients));

    println!("{Ingredients:?}");

    let result = get_scores(&Ingredients);

    println!("{result:?}")
}
