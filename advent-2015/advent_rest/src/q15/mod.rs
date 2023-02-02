use itertools::Itertools;
use std::cmp;
use std::{collections::HashMap, fs};

#[derive(Debug)]
struct Ingredient {
    capacity: i64,
    durability: i64,
    flavor: i64,
    texture: i64,
    calories: i64,
}

impl Ingredient {
    fn new() -> Ingredient {
        Ingredient {
            capacity: 0,
            durability: 0,
            flavor: 0,
            texture: 0,
            calories: 0,
        }
    }
    fn add(&mut self, num: i64, ingredient: &Ingredient) {
        self.capacity += ingredient.capacity * num;
        self.durability += ingredient.durability * num;
        self.flavor += ingredient.flavor * num;
        self.texture += ingredient.texture * num;
        self.calories += ingredient.calories * num;
    }

    fn total(self) -> i64 {
        cmp::max(self.capacity, 0)
            * cmp::max(self.durability, 0)
            * cmp::max(self.flavor, 0)
            * cmp::max(self.texture, 0)
    }
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

impl FoodInfo {
    fn get_scores(self) {
        let mut max_score = i64::MIN;

        for comp in self.ingredients.keys().combinations_with_replacement(100) {
            let mut current = Ingredient::new();
            for (num, name) in comp.iter().dedup_with_count() {
                let x = self.ingredients.get(&name.to_string()).unwrap();
                current.add(num as i64, &x)
            }
            if current.calories == 500 {
                max_score = cmp::max(max_score, current.total());
            }
        }
        println!("{max_score}")
    }
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

    // println!("{:?}", food_info);
    food_info.get_scores()
    // let mut Ingredients = Vec::new();
    // context
    //     .lines()
    //     .for_each(|line| parse(line, &mut Ingredients));

    // println!("{Ingredients:?}");

    // let result = get_scores(&Ingredients);

    // println!("{result:?}")
}
