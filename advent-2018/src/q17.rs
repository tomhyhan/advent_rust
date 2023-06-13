use std::{collections::HashMap, hash::Hash};
use regex::Regex;
use advent_2018::{Runner, get_file};

pub struct Q17 {

}
 
struct Reservoir {
    clay_or_water: HashMap<(i32,i32), char>,
    max_y: i32,
}


impl Reservoir {
    fn new() -> Self {
        let contents = get_file("src/input/q17.txt").unwrap();
        let mut clay_or_water = HashMap::new();
        let mut max_y = 0;
        contents.lines().for_each(|line| {
            let re = Regex::new(r"\d+").unwrap();
            let nums: Vec<i32>= re.find_iter(line).map(|n| n.as_str().parse::<i32>().unwrap()).collect();
            if line.starts_with("x") {
                let x = nums[0];
                let y_start = nums[1];
                let y_end = nums[2];
                for y in y_start..=y_end {
                    clay_or_water.insert((y,x), '#');
                    max_y = max_y.max(y);
                }
            } else {
                let y = nums[0];
                max_y = max_y.max(y);
                let x_start = nums[1];
                let x_end = nums[2];
                for x in x_start..=x_end {
                    clay_or_water.insert((y,x), '#');
                }
            }
        });
        clay_or_water.insert((0, 500), '|');
        Self {clay_or_water, max_y} 
    }

    fn flow(&mut self) {
        let mut cnt = 0;
        loop {
            let current_water = self.clay_or_water.clone();
            for ((y,x), value) in current_water.iter() {
                if *value == '|' {
                    let bottom;
                    if self.clay_or_water.contains_key(&(y+1, *x)) {
                        bottom = self.clay_or_water.get(&(y+1, *x)).unwrap();
                    } else {
                        bottom = &'.';
                    }
    
                    if *bottom == '#' || *bottom == '~' {
                        let x_bot_left = self.check_left(y+1, *x, &current_water);
                        let x_bot_right = self.check_right(y+1, *x, &current_water);
                        let mut left = None;
                        let mut right = None;
                        for left_x in (x_bot_left..*x).rev() {
                            if let Some(current) = self.clay_or_water.get(&(*y, left_x)) {
                                if *current == '#' {
                                    left = Some(left_x);
                                    break
                                }
                            } 
                        }
                        for right_x in (*x+1..=x_bot_right) {
                            if let Some(current) = self.clay_or_water.get(&(*y, right_x)) {
                                if *current == '#' {
                                    right = Some(right_x);
                                    break
                                }
                            } 
                        }
                        if left.is_some() && right.is_some() {
                            for river_x in left.unwrap() + 1..right.unwrap() {
                                self.clay_or_water.insert((*y, river_x), '~');
                            }
                        } else if left.is_some() && right.is_none() {
                            for river_x in left.unwrap()+1..=x_bot_right {
                                self.clay_or_water.insert((*y, river_x), '|');
                            }
                        } else if left.is_none() && right.is_some() {
                            for river_x in x_bot_left..=right.unwrap() {
                                self.clay_or_water.insert((*y, river_x), '|');
                            }
                        } else if left.is_none() && right.is_none(){
                            for river_x in x_bot_left..=x_bot_right {
                                self.clay_or_water.insert((*y, river_x), '|');
                            }
                        }
                    } else {
                        self.clay_or_water.insert((*y+1, *x), '|');
                    }
                }
            }
            if cnt == 10000 {
                break
            }
            cnt += 1;
        }

        for row in 0..self.max_y{
            let mut s = "".to_string();
            for col in 450..550 {
                if self.clay_or_water.contains_key(&(row,col)) {
                    s.push(*self.clay_or_water.get(&(row,col)).unwrap());
                } else {
                    s.push('.');
                }
            }
            println!("{:?}", s);
        }
        println!("{:?}", self.clay_or_water.iter().filter(|(_,&v)| v== '~').count());
        println!("{:?}", self.clay_or_water.iter().filter(|((y,_),&v)| v== '|' && *y <= self.max_y).count() - 1);
    }

    fn check_left(&self, y:i32, x:i32, current_water: &HashMap<(i32,i32), char>) -> i32 {
        let mut left_x = x;
        while current_water.contains_key(&(y,left_x)) {
            let left = current_water.get(&(y,left_x)).unwrap();
            if left == &'|' {
                break
            }
            left_x -= 1;
        }
        left_x
    }
    fn check_right(&self,  y:i32, x:i32, current_water: &HashMap<(i32,i32), char>) -> i32 {
        let mut right_x = x;
        while current_water.contains_key(&(y,right_x)) {
            let right = current_water.get(&(y,right_x)).unwrap();
            if right == &'|' {
                break
            }
            right_x += 1;
        }
        right_x
    }
}


impl Q17 {
    pub fn new() -> Self {
        Q17 {}
    }

    fn part1(&mut self) {
        let mut reservoir = Reservoir::new();
        reservoir.flow();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q17 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q17() {
        assert_eq!(1, 1);
    }
}