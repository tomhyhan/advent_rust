use std::collections::{HashSet, HashMap};

use advent_2020::{Runner, get_file};

struct Food {
    ingredients: Vec<HashSet<String>>,
    allergens: Vec<HashSet<String>>
}

impl Food {
    fn new() -> Self {
        let content = get_file("src/input/q21.txt").unwrap();
        let mut ingredients = vec![];
        let mut allergens= vec![];
        content.lines().for_each(|line|{
            let (left, right) = line.split_once(" (contains ").unwrap();
            ingredients.push(left.split_whitespace().map(|ing|ing.to_string()).collect::<HashSet<String>>());
            allergens.push(right.split(", ").map(|allergen|allergen.trim_end_matches(')').to_string()).collect::<HashSet<String>>());
        });
        Self { ingredients, allergens }
    }

    fn find_matches(&self) {
        let mut af = HashMap::new();
        // println!("{:?} {:?}", self.ingredients, self.allergens);
        for i in 0..self.ingredients.len() {
            let ingredients = &self.ingredients[i]; 
            let allergens = &self.allergens[i];
            for aller in allergens.iter() {
                let ings = af.entry(aller).or_insert(ingredients.clone());
                let intersection: HashSet<_>= ings.intersection(ingredients).map(String::from).collect();
                af.insert(aller, intersection);
            }
        }

        let food_with_allergions= af.values().cloned().reduce(|acc,e|{
            let union: HashSet<String> = acc.union(&e).map(String::from).collect();
            union
        }).unwrap();

        let mut no_allergions = 0;
        for ingredients in self.ingredients.iter() {
            for ings in ingredients.iter() {
                if food_with_allergions.contains(ings) {
                    continue;
                }
                no_allergions += 1;
            }
        }
        println!("food - {:?}", no_allergions);

        let mut used = HashSet::new();
        let mut food_aller_matches = vec![];
        while used.len() < food_with_allergions.len() {
            println!("{:?}", food_aller_matches);
            for (aller, foods) in af.iter() {
                let mut foods: Vec<_> = foods.iter().filter(|food|!used.contains(*food)).collect();
                if foods.len() == 1 {
                    let food = foods.pop().unwrap();
                    used.insert(food);
                    food_aller_matches.push((aller, food))
                }
            } 
        }
        food_aller_matches.sort();
        println!("{:?}", food_aller_matches.iter().map(|(_,v)| v.to_string()).collect::<Vec<_>>().join(","));
    }
}

pub struct Q21 {

}

impl Q21 {
    pub fn new() -> Self {
        Q21 {}
    }

    fn part1(&mut self) {
        let food = Food::new();
        food.find_matches();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q21 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q21() {
        assert_eq!(1, 1);
    }
}