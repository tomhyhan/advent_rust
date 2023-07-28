use std::collections::HashMap;

use advent_2020::{Runner, get_file};

struct Regulations {
    rules: HashMap<String, Vec<(String, usize)>>,
    bag_list: Vec<String>
}

impl Regulations {
    fn new() -> Self {
        let content = get_file("src/input/q8.txt").unwrap();
        let mut rules = HashMap::new();
        let mut bag_list = Vec::new();
        content.lines().for_each(|line| {
            let (bag, inside_bags) = line.split_once(" contain ").unwrap();
            let bags;
            if inside_bags.contains("no other bags") {
                bags = vec![];
            } else {
                bags = inside_bags.split(", ").map(|bag_info|{
                    let (num_of_bags, current_bag) = bag_info.split_once(" ").unwrap();
                    (current_bag.trim_end_matches(|c| c== 's'|| c=='.').to_string(), num_of_bags.parse::<usize>().unwrap())
                }).collect();

            }
            rules.insert(bag.trim_end_matches('s').to_string(), bags);
            bag_list.push(bag.trim_end_matches('s').to_string());
        });
        Self { rules, bag_list }
    }
    
    fn find_how_many_bags_contains(&self, target_bag: String) -> usize{
        self.bag_list.iter().filter(|bag| self.contains(bag, &target_bag)).count()
    }

    fn contains(&self, bag: &String, target_bag: &String)-> bool {
        let mut is_contain = false;
        for inner_bag_info in self.rules.get(bag).unwrap() {
            let inner_bag = &inner_bag_info.0;
            if inner_bag == target_bag {
                return true
            };
            if self.contains(inner_bag, target_bag) {
                is_contain = true
            }
        }
        is_contain
    }

    fn cnt_how_many_bags_contains(&self, bag: &String) -> usize{
        // 2 + 4 + 8
        let mut cnt = 0;
        for (inner_bag, bag_cnt) in self.rules.get(bag).unwrap() {
            cnt += bag_cnt + bag_cnt * self.cnt_how_many_bags_contains(inner_bag);
        }
        println!("{:?} {}", cnt, bag);
        cnt  
    }
}

pub struct Q8 {

}

impl Q8 {
    pub fn new() -> Self {
        Q8 {}
    }

    fn part1(&mut self) {
        let regulations = Regulations::new();
        println!("found - {:?}", regulations.find_how_many_bags_contains("shiny gold bag".to_string()));
    }

    fn part2(&mut self) {
        let regulations = Regulations::new();
        let shiny_gold_bag = "shiny gold bag".to_string();
        assert_eq!(regulations.contains(&shiny_gold_bag, &shiny_gold_bag), false);
        println!("total - {:?}", regulations.cnt_how_many_bags_contains(&"shiny gold bag".to_string()))
    }

}

impl Runner for Q8 {
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
    fn Q8() {
        assert_eq!(1, 1);
    }
}