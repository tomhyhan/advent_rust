use std::collections::{HashSet, HashMap};

use advent_2020::{Runner, get_file};

struct Power {
    jolts: Vec<i64>
}

impl Power {
    fn new() -> Self {
        let content = get_file("src/input/q10.txt").unwrap();
        let mut jolts: Vec<i64> = content.lines().map(|line|line.parse::<i64>().unwrap()).collect();
        // jolts.push(0); part 1
        Self { jolts }
    }
    
    fn find_differences(&mut self) {
        self.jolts.sort();
        let mut diffs = [0,0,0];
        self.jolts.windows(2).for_each(|grp|{
            let (low, high) = (grp[0],grp[1]);
            let diff = high - low;
            println!("{:?}", grp);
            match diff {
                1 => diffs[0] += 1,
                2 => diffs[1] += 1,
                3 => diffs[2] += 1,
                _ => panic!("invalid jolt")
            }
        });
        diffs[2] += 1;
        println!("{:?}", diffs[0] * diffs[2]);
    }

    // solution 1 vec dp
    // fn ways_arrange_adapters(&mut self) {
    //     let max = self.jolts.iter().max().unwrap();
    //     let size = *max as usize;
    //     let mut memo: Vec<i64> = vec![0;size+1];
    //     memo[0] = 1;
    //     for i in 1..memo.len() {
    //         if !self.jolts.contains(&(i as i64)) {
    //             continue
    //         }

    //         let mut cnt = 1; 
    //         while cnt < 4 && i as i64 - cnt as i64 >= 0 {
    //             let idx = i - cnt;
    //             memo[i] += memo[idx];
    //             cnt += 1;
    //         }
    //         println!("{:?}", memo)
    //     }
    // }

    // solution 2 recursion dp
    fn ways_arrange_adapters(&mut self) -> i64 {
        let max = *self.jolts.iter().max().unwrap();
        self.jolts.sort();
        let mut memo = HashMap::new();
        let ways = self.count_ways( 0, 0, max+3, &mut memo);
        ways
    }

    fn count_ways(&self, start: i64, adapter_idx: usize, target: i64, memo: &mut HashMap::<(i64, usize), i64>) -> i64  {
        let key = (start, adapter_idx);
        if memo.contains_key(&key) {
            return *memo.get(&key).unwrap();
        }
        let mut ways = 0;
        if target - start <= 3 {
            return ways + 1
        }
        if adapter_idx >= self.jolts.len() {
            return ways
        }
        if self.jolts[adapter_idx] - start  <= 3 {
            ways += self.count_ways(self.jolts[adapter_idx], adapter_idx+1, target, memo)
        }
        ways += self.count_ways(start, adapter_idx+1, target, memo);
        memo.insert(key, ways);
        ways
    } 
    
}

pub struct Q10 {

}

impl Q10 {
    pub fn new() -> Self {
        Q10 {}
    }

    fn part1(&mut self) {
        // let mut power = Power::new();
    }
    
    fn part2(&mut self) {
        let mut power = Power::new();
        println!("{:?}",  power.ways_arrange_adapters());
    }

}

impl Runner for Q10 {
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
    fn Q10() {
        assert_eq!(1, 1);
    }
}