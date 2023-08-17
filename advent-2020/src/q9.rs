use std::collections::{HashSet, HashMap};

use advent_2020::{Runner, get_file};


struct Xmax {
    payload: Vec<i64>
}

impl Xmax {
    fn new() -> Self {
        let content = get_file("src/input/q9.txt").unwrap();
        let payload: Vec<i64> = content.lines().map(|num|{
            num.parse::<i64>().unwrap()
        }).collect();

        Self {payload}
    }

    fn find_first_not_sum(&self, preamble_size: usize) -> i64 {
        for i in preamble_size..self.payload.len() {
            let target = self.payload[i];
            let preambles = &self.payload[i-preamble_size..i];
            if !self.can_find_sum(target, preambles) {
                return target
            }
        }
        panic!("all numbers have sum")
    }

    fn can_find_sum(&self, target: i64, preambles: &[i64]) -> bool {
        let mut matches = HashSet::new();
        for preamble in preambles.iter() {
            let diff = target - preamble;
            if matches.contains(preamble) {
                return true
            } else {
                matches.insert(diff);
            }
        }
        false
    }

    // right sum
    fn find_invalid_sum(&self, target: i64) -> Vec<i64> {
        let mut right_sum = 0;
        let mut right_sum_tracker = HashMap::new();
        for i in 0..self.payload.len() {
            right_sum_tracker.insert(right_sum, i);
            right_sum += self.payload[i]; 
            if right_sum_tracker.contains_key(&(right_sum - target)) {
                let low =  *right_sum_tracker.get(&(right_sum - target)).unwrap();
                let high = i;
                return self.payload[low..high+1].to_vec()
            } 
        }
        panic!("sum not found")
    }

    // brute force 
    // fn find_invalid_sum(&self, target: i64) -> Vec<i64> {
    //     for i in 0..self.payload.len() {
    //         let mut nums = vec![self.payload[i]];
    //         for j in i+1..self.payload.len() {
    //             nums.push(self.payload[j]);
    //             let sum: i64 = nums.iter().sum();
    //             if sum == target {
    //                 return nums
    //             } else if sum > target {
    //                 break
    //             }
    //         }
    //     }
    //     panic!("sum not found")
    // }
}

pub struct Q9 {

}

impl Q9 {
    pub fn new() -> Self {
        Q9 {}
    }

    fn part1(&mut self) {
        // let xmax = Xmax::new();
        // println!("{:?}", xmax.find_first_not_sum(25));
    }

    fn part2(&mut self) {
        let xmax = Xmax::new();
        let invalid = xmax.find_first_not_sum(25);
        let mut sum_vec = xmax.find_invalid_sum(invalid);
        sum_vec.sort();
        println!("{:?}", sum_vec[0] + sum_vec[sum_vec.len()-1]);
    }

}

impl Runner for Q9 {
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
    fn Q9() {
        assert_eq!(1, 1);
    }
}