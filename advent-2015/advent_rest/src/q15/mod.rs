use std::{collections::HashMap, fs};

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

#[derive(Debug)]
struct FoodInfo {
    ingredients: HashMap<String, Ingredient>,
}

impl From<String> for FoodInfo {
    fn from(context: String) -> FoodInfo {
        let mut food_info = FoodInfo {
            ingredients: HashMap::new(),
        };

        println!("{context:?}");
        context.lines().for_each(|line| {
            let info: Vec<_> = line
                .split(" ")
                .map(|w| w.trim_matches(|c| c == ',' || c == ':'))
                .collect();
            println!("{:?}", info);
            food_info.ingredients.insert(
                info[0].to_string(),
                Ingredient {
                    capacity: info[2].parse::<i64>().unwrap(),
                    durability: info[4].parse::<i64>().unwrap(),
                    flavor: info[6].parse::<i64>().unwrap(),
                    texture: info[8].parse::<i64>().unwrap(),
                    calories: info[10].parse::<i64>().unwrap(),
                },
            );
        });

        food_info
    }
}

pub fn run_code() {
    let path = "q15.txt";
    let context = fs::read_to_string(path).expect("fail to read a file");

    let food_info: FoodInfo = context.into();

    println!("{:?}", food_info);
    // let mut Ingredients = Vec::new();
    // context
    //     .lines()
    //     .for_each(|line| parse(line, &mut Ingredients));

    // println!("{Ingredients:?}");

    // let result = get_scores(&Ingredients);

    // println!("{result:?}")
}
