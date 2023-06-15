use std::{collections::{HashMap, VecDeque}, hash::Hash};
use regex::Regex;
use advent_2018::{Runner, get_file};

pub struct Q17 {

}
 
struct Reservoir {
    grid: Vec<Vec<char>>,
    max_y: i32,
    max_x: i32,
    min_y: i32,
    min_x: i32
}


impl Reservoir {
    fn new() -> Self  {
        let contents = get_file("src/input/q17.txt").unwrap();
        let mut clays: Vec<(i32, i32)> = vec![];
        contents.lines().for_each(|line| {
            let re = Regex::new(r"\d+").unwrap();
            let nums: Vec<i32>= re.find_iter(line).map(|n| n.as_str().parse::<i32>().unwrap()).collect();
            if line.starts_with("x") {
                let x = nums[0];
                let y_start = nums[1];
                let y_end = nums[2];
                for y in y_start..=y_end {
                    clays.push((y,x));
                }
                
            } else {
                let y = nums[0];
                let x_start = nums[1];
                let x_end = nums[2];
                for x in x_start..=x_end {
                    clays.push((y,x));
                }
            }
        });
        let mut max_y = i32::MIN;
        let mut max_x = i32::MIN;
        let mut min_y = i32::MAX;
        let mut min_x = i32::MAX;
        for (y,x) in clays.iter() {
            max_y = max_y.max(*y);
            max_x = max_x.max(*x);
            min_y = min_y.min(*y);
            min_x = min_x.min(*x);
        }
        min_x -= 1;
        max_x += 1;

        let mut grid: Vec<Vec<char>> = vec![];
        for i in 0..=max_y {
            let mut tmp = vec![];
            for j in 0..(max_x-min_x+1) {
                tmp.push('.')
            }
            grid.push(tmp);
        }
        grid[0][(500-min_x) as usize] = '+';
        for (row,col) in clays.iter() {
            grid[*row as usize][(col - min_x) as usize] = '#';
        }

        // for item in grid.iter() {
        //     println!("{:?}", item.iter().collect::<String>());
        // }
        // println!("{:?}", vec!['~','#'].contains(&'.'));
        Self {grid,max_x,max_y,min_x,min_y}
    }

    fn flow(&mut self) {
        let mut to_visit = VecDeque::from(vec![(1,500)]);
        
        while let Some((y,x)) = to_visit.pop_front() {
            if self.grid[y][(x-self.min_x) as usize] == '.'{
                self.grid[y][(x-self.min_x) as usize] = '|';
            } 
            if y == self.max_y as usize {
                continue
            }
            if self.grid[y+1][(x-self.min_x) as usize] == '.' {
                to_visit.push_back((y+1,x));
                continue
            } else if vec!['~', '#'].contains(&self.grid[y+1][(x-self.min_x) as usize]){
                if self.grid[y][(x-self.min_x+1) as usize] == '.'{
                    to_visit.push_back((y,x+1))
                }
                if self.grid[y][(x-self.min_x-1) as usize] == '.' {
                    to_visit.push_back((y,x-1))
                }
                if vec!['|', '#'].contains(&self.grid[y][(x-self.min_x+1) as usize]) && vec!['|', '#'].contains(&self.grid[y][(x-self.min_x-1) as usize]) {
                    let mut right = x;
                    while vec!['|', '~'].contains(&self.grid[y][(right-self.min_x+1) as usize]) {
                        right += 1;
                    }
                    if self.grid[y][(right-self.min_x+1) as usize] !='#' {
                        continue
                    }
                    let mut left = x;
                    while vec!['|', '~'].contains(&self.grid[y][(left-self.min_x-1) as usize]) {
                        left -= 1;
                    }
                    if self.grid[y][(left-self.min_x-1) as usize] !='#' {
                        continue
                    }
                    let mut tmp = x;
                    self.grid[y][(tmp-self.min_x) as usize] = '~';
                    if self.grid[y-1][(tmp-self.min_x) as usize] == '|'{
                        to_visit.push_back((y-1,tmp)) 
                    }
                    while vec!['|', '~'].contains(&self.grid[y][(tmp-self.min_x+1) as usize]) {
                        self.grid[y][(tmp-self.min_x+1) as usize] = '~';
                        tmp += 1;
                        // if self.grid[y-1][(tmp-self.min_x) as usize] == '|' {
                        //     to_visit.push_back((y-1,tmp))
                        // }
                    }
                    while vec!['|', '~'].contains(&self.grid[y][(tmp-self.min_x-1) as usize]) {
                        self.grid[y][(tmp-self.min_x-1) as usize] = '~';
                        tmp -= 1;
                        // if self.grid[y-1][(tmp-self.min_x) as usize] == '|' {
                        //     to_visit.push_back((y-1,tmp))
                        // }
                    }
                }
            } 
        }
        let mut cnt = 0;
        for item in self.grid.iter() {
            println!("{:?}", item.iter().collect::<String>());
            if cnt == 500 {
                break
            }
            cnt += 1
        }
        // // 27213
        let mut r = 0;
        let mut f = 0;
        println!("{} {}", self.min_y, self.max_y);
        self.grid[self.min_y as usize..=self.max_y as usize].iter().for_each(|line | line.iter().for_each(|c| {
            if *c == '~' {
                r += 1;
            }
            if *c == '|' {
                f += 1;
            }
        }));
        println!("r - {:?}", r);
        println!("f - {:?}", f);
        println!("t - {:?}", f + r);
        println!("{:?}", self.grid.len());
        println!("{:?}", self.grid[0].len());
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