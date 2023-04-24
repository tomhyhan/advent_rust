use std::{collections::HashMap, vec, time};

use advent_2018::{Runner, get_file};
use itertools::iproduct;

pub struct Q11 {

}

struct Charge {
    grid: Vec<Vec<i32>>,
    sums: HashMap<(usize,usize,usize), i32>,
    serial_number: i32
}

impl Charge {
    fn new() -> Self {
        let content = get_file("src/input/q11.txt").unwrap();
        let serial_number = content.parse().unwrap();
        let grid = vec![vec![0;301];301];
        let sums = HashMap::new();
        Charge { grid,sums,serial_number}
    }

    fn find_fuels(&mut self) {
        for row in 1..self.grid.len() {
            for col in 1..self.grid[0].len() {
                let ract_id = (col + 10) as i32;
                let mut power_level = ract_id * row as i32;
                power_level += self.serial_number;
                power_level *= ract_id;
                power_level = (power_level / 100) % 10;
                power_level -= 5;
                self.grid[row][col] = power_level;
            }
        }
    }

    fn generate_areas(&mut self) {
        for row in 1..self.grid.len() {
            for col in 1..self.grid[0].len() {
                    self.grid[row][col] += self.grid[row][col-1] + self.grid[row-1][col] - self.grid[row-1][col-1]
            }
        }
    }

    fn find_max(&mut self) -> i32 {
        let mut max = 0;
        for area in 1..300 {
            for row in area..self.grid.len() {
                for col in area..self.grid[0].len() {
                    let (x,y)= (col - area+ 1, row - area + 1);
                    let size = self.find_size(row,col,area);
                    self.sums.insert((x, y, area), size);
                    max = max.max(size)
                }
            }
        }
        max
    }

    //  1,1 1,2 1,3
    //  2,1 2,2 2,3
    fn find_size(&self, row: usize, col: usize, area: usize) -> i32 {
        let mut size = 0;
        size += self.grid[row][col] - (self.grid[row-area][col] + self.grid[row][col-area] - self.grid[row-area][col-area]);
        size
    }
}

impl Q11 {
    pub fn new() -> Self {
        Q11 {}
    }

    fn part1(&mut self) {
        let start = time::Instant::now();
        let mut charge = Charge::new();
        charge.find_fuels();
        charge.generate_areas();
        charge.find_max();
        println!("{:?}", charge.sums.iter().max_by(|x,y|x.1.cmp(&y.1)).unwrap());
        println!("{:?}", start.elapsed())
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q11 {
    fn run(&mut self) {
        self.part1()
    }
}


#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn Q11() {
        let mut charge = Charge::new();
        charge.find_fuels();
        // 33,45
        let r = charge.grid[45][33];
        assert_eq!(r, 3);
    }

    // #[test]
    // fn find_max_of_18() {
    //     let mut charge = Charge::new();
    //     charge.find_fuels();
    //     // 33,45
    //     charge.find_max();
    //     let r = charge.sums.get(&(33,45)).unwrap();
    //     assert_eq!(*r, 29);
    // }
}
