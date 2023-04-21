use std::collections::{HashMap, HashSet};

use advent_2018::{Runner, get_file};


#[derive(Debug)]
struct Sky {
    lights: Vec<Light>
}

impl Sky {
    fn new() -> Self {
        let mut lights = Vec::new();
        let contents = get_file("src/input/q10.txt").unwrap();
        contents.lines().for_each(|line| parse(line,&mut lights));
        Sky {lights}
    }

    fn display(&mut self){
        let mut min_row = i32::MIN;
        let mut max_row = i32::MAX;
        let mut min_col = i32::MIN;
        let mut max_col = i32::MAX;
        let mut seen = HashSet::new();
        let mut row_distance = i32::MAX;
        let mut col_distance = i32::MAX;
        let mut cnt = 0;
        
        loop {
            let mut c_min_row = i32::MAX;
            let mut c_max_row = i32::MIN;
            let mut c_min_col = i32::MAX;
            let mut c_max_col = i32::MIN;
            let mut c_seen = HashSet::new();

            for light in self.lights.iter_mut() {
                light.row += light.row_vec;
                light.col += light.col_vec;
                c_min_row = c_min_row.min(light.row);
                c_max_row = c_max_row.max(light.row);
                c_min_col = c_min_col.min(light.col);
                c_max_col = c_max_col.max(light.col);
                c_seen.insert((light.row, light.col));
            }
            let c_row_distance = (c_max_row - c_min_row).abs();
            let c_col_distance = (c_max_col - c_min_col).abs();

            if c_row_distance > row_distance || c_col_distance > col_distance {
                break
            }

            row_distance = row_distance.min(c_row_distance);
            col_distance = col_distance.min(c_col_distance);

            seen = c_seen;
            min_row = min_row.max(c_min_row);
            max_row = max_row.min(c_max_row);
            min_col = min_col.max(c_min_col);
            max_col = max_col.min(c_max_col);
            cnt += 1
        }

        let col_length = (max_col-min_col+1)  as usize;
        let row_length = (max_row-min_row+1) as usize;

        let mut grid = vec![vec!['.'; 250]; 250];

        seen.iter().for_each(|l| {
            grid[(l.0) as usize][(l.1) as usize] = '#';
        });

        grid.iter().for_each(|line| {
            println!("{:?}", line.iter().collect::<String>());
        });
        println!("{}", cnt)
    }
}

fn parse(line:&str, lights: &mut Vec<Light>) {
    let iter: Vec<_> = line.split(|c| c == '<' || c == '>' || c == ',' ).map(|token| token.trim()).collect();
    let col = iter[1].parse::<i32>().unwrap();
    let row = iter[2].parse::<i32>().unwrap();
    let col_vec = iter[4].parse::<i32>().unwrap();
    let row_vec = iter[5].parse::<i32>().unwrap();
    lights.push(Light{row, col, row_vec, col_vec});
}

#[derive(Clone, Copy, Debug, Eq, Hash, PartialEq)]
struct Light {
    row: i32,
    col: i32,
    row_vec: i32,
    col_vec: i32
}

pub struct Q10 {

}

impl Q10 {
    pub fn new() -> Self {
        Q10 {}
    }

    fn part1(&mut self) {
        let mut sky = Sky::new();
        sky.display();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q10 {
    fn run(&mut self) {
        self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q10() {
        assert_eq!(1, 1);
    }
}