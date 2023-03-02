use crate::{Runner, common::my_modules::get_file};
use std::cmp;
pub struct Q20 {
    block_list : Vec<Vec<i64>>
}

impl Q20 {
    pub fn new() -> Self {
        let content = get_file("q20.txt").unwrap();
    
        let mut block_list : Vec<Vec<i64>> = content.lines().map(|line| parse(line)).collect();
        block_list.sort_by_key(|ips| ips[0]);
        Q20 {block_list}
    }

    fn part1(&mut self) {
        let mut valid_ips = 0;
        let mut smallest = 0;
        let mut largest = 0;
        let mut range = vec![];
        let mut lists = self.block_list.iter_mut();
        let mut current = lists.next().unwrap() ;

        while let Some(list) = lists.next() {
            let (c_l_range, c_h_range ) = (current[0], current[1]);
            let (l_range, h_range) = (list[0],list[1]);

            if c_h_range + 1 >= l_range {
                current[1] = cmp::max(c_h_range, h_range);
            } else {
                range.push(current.clone());
                current = list;
            }
        } 
        range.push(current.to_vec());
        println!("{range:?}");
        println!("{:?}", 4294967295 -  range.iter().map(|ips| ips[1] - ips[0] + 1).sum::<i64>() + 1)


    }
}




fn parse(line : &str) -> Vec<i64> {
    let ips : Vec<i64> = line.split("-").map(|ip|ip.parse::<i64>().unwrap()).collect();
    ips
}

impl Runner for Q20 {
    fn run(&mut self) -> () {
        self.part1()
    }
}