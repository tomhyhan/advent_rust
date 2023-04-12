use std::collections::{HashMap, HashSet};

use advent_2018::{Runner, get_file};

pub struct Q2 {

}

struct Warehouse {
    boxes: Vec<String>,
    counts: Vec<HashMap<char, i32>>
}

impl Warehouse {
    fn new() -> Self {
        let content = get_file("src/input/q2.txt").unwrap();
        let boxes : Vec<_> = content.lines().map(String::from).collect();
        let counts: Vec<_> = boxes.iter().map(|bx| {
            let mut count = HashMap::new();
            bx.chars().for_each(|char| {
                count.entry(char).and_modify(|cnt| *cnt += 1).or_insert(1);
            });
            count
        }).collect();
        Warehouse { boxes, counts }
    }

    fn scan(&self) {
        let mut two = 0;
        let mut three = 0; 
        self.counts.iter().for_each(|count| {
            let mut is_two = false;
            let mut is_three = false;
            count.iter().for_each(|(_,val)|{
                match *val {
                    2 => is_two = true,
                    3 => is_three = true,
                    _ => ()
                }
            });
            if is_two { two += 1};
            if is_three { three += 1};
        });
        println!("{:?}", two * three)
    }

    fn find_common(&self) {
        let mut commons = vec![];
        for i in 0..self.boxes.len() {
            let mut result = ("".to_string(), i32::MAX);
            for j in i+1..self.boxes.len() {
                let current_diff = self.find_diff(i,j);
                if current_diff < result.1 {
                    result = (self.boxes[j].clone(),current_diff) 
                }
            }
            commons.push((self.boxes[i].clone(),result.0,result.1));
        }
        
        let min = commons.iter().min_by(|x,y|x.2.cmp(&y.2)).unwrap();

        let set1: HashSet<_>= min.1.chars().collect();
        
        let mut result = String::new();
        for char in min.0.chars() {
            if set1.contains(&char) {
                result.push_str(&char.to_string());
            }
        }
        println!("{:?}", result);
    }

    fn find_diff(&self, i:usize, j:usize) -> i32 {
        let chars_vec: Vec<char>= self.boxes[j].chars().collect();
        let mut diff = 0;
        self.boxes[i].chars().enumerate().for_each(|(idx, char)| {
            if char != chars_vec[idx] {
                diff +=1;
            }
        });
        diff
    }
}

impl Q2 {
    pub fn new() -> Self {
        Q2 {}
    }

    fn part1(&mut self) {
        let warehouse = Warehouse::new();
        warehouse.scan()
    }

    fn part2(&mut self) {
        let warehouse = Warehouse::new();
        warehouse.find_common();
    }

}

impl Runner for Q2 {
    fn run(&mut self) {
        // self.part1()
        // self.part2()
        let x = "abcd".to_string();
        let y = "abcf".to_string();
        let z = x.chars().zip(y.chars())
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q2() {
        assert_eq!(1, 1);
    }
}