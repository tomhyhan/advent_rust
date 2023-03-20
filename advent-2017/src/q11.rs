use std::{collections::HashMap, vec};
use lazy_static::lazy_static;
use advent_2017::{Runner, get_file};

lazy_static! {
    static ref DIRECTION : HashMap<String, (i32,i32,i32)> = HashMap::from([("n".to_string(), (1,-1,0)),("ne".to_string(), (0,-1,1)),("se".to_string(), (-1,0,1)),("s".to_string(), (-1,1,0)),("sw".to_string(), (0,1,-1)),("nw".to_string(), (1,0,-1))]);
}

pub struct Q11 {
    process: Vec<String>
}

impl Q11 {
    pub fn new() -> Self {
        let content = get_file("q11.txt").unwrap();
        let process = content.split(",").map(|p| p.to_string()).collect();
        Q11 { process }
    }

    // method 1
    fn part1(&mut self) {
        let mut hex_grid = vec![0,0,0];

        let mut max = 0;
        let mut min = 0;
        self.process.iter().for_each(|dir| {
            let direction = DIRECTION.get(dir).unwrap();
            hex_grid[0] += direction.0;
            hex_grid[1] += direction.1;
            hex_grid[2] += direction.2;
            max = max.max(*hex_grid.iter().max().unwrap());
            min = min.min(*hex_grid.iter().min().unwrap());
        });
        println!("{:?}", hex_grid);
        println!("{} {}", max,min);
    }
}

impl Runner for Q11 {
    fn run(&mut self) {
        self.part1();
    }
}