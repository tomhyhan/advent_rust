use std::collections::{HashMap, HashSet};
use itertools::Itertools;
use advent_2017::{Runner, get_file};

pub struct Q21 {

}

struct Art {
    books: HashMap<String, String>,
    start_pattern : String
}

impl Art {
    fn new() -> Self {
        let mut books = HashMap::new();
        let content = get_file("q21.txt").unwrap();
        content.lines().for_each(|line|{
            parse(line, &mut books)
        });
        let start_pattern = ".#...####".to_string();
        Self {
            books,
            start_pattern
        }
    }

    fn iteration(&mut self, mut cnt: i32) {
        let mut pattern = self.start_pattern.clone();
        while cnt > 0 {
            let mut size = 0;
            if pattern.len() % 2 == 0 {
                size = 2;
            } else if pattern.len() % 3 == 0 {
                size = 3;
            }

            
            cnt -= 1;
        }
    }

    fn perform_enhancement(&self, mut grid: Vec<Vec<char>>) {

    }

    fn transpose(&self, grid: &mut Vec<Vec<char>> ) {
        let len = grid[0].len();
        for row in 0..grid.len() {
            for col in row..len {
                let temp = grid[col][row];
                grid[col][row] = grid[row][col];
                grid[row][col] = temp;
            }
        }
    }

    fn reverse(&self, grid: &mut Vec<Vec<char>> ) {
        let len = grid[0].len();
        for row in 0..grid.len() {
            for col in 0..(len / 2) {
                let temp = grid[row][col];
                grid[row][col] = grid[row][len - col - 1];
                grid[row][len - col - 1] = temp;
            }
        }
    }

    fn get_grids(&self, new_points: Vec<(usize,usize)>, size:usize, pattern: &Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
        let mut grids = Vec::new();
        for (x,y) in new_points.into_iter() {
            let mut grid = Vec::new();
            for row in x..x+size {
                grid.push(pattern[row][y..y+size].to_vec().clone());
            }
            grids.push(grid);
        }
        grids
    }

    fn get_points(&self, mut length: usize, divisible: usize) -> Vec<usize> {
        let mut points = vec![];
        while length > 0 {
            length -= divisible;
            points.push(length)
        }
        points
    }
}

fn parse(line: &str, books: &mut HashMap<String,String>) {
    let (left_img, right_img) = line.split_once(" => ").unwrap();
    books.insert(left_img.to_string(), right_img.to_string());
}


#[derive(Debug,PartialEq, Eq, Hash, Clone)]
struct Image {
    pattern : Vec<Vec<char>>,
}

impl Q21 {
    pub fn new() -> Self {
        Q21 {}
    }

    fn part1(&mut self) {
        let grid = Grid::new();
        let s = grid.split();
        println!("{:?}", s);
    }
}

impl Runner for Q21 {
    fn run(&mut self) {
        self.part1()
    }
}


#[derive(Debug, Clone)]
struct Grid {
    size: usize,
    pixels: HashSet<(usize, usize)>,
}


impl Grid {
    fn new() -> Self {
        let starting_pixels = vec![(0, 1), (1, 2), (2, 0), (2, 1), (2, 2)];
        Self {
            size: 3,
            pixels: HashSet::from_iter(starting_pixels),
        }
    }

    // 0011
    // 0011
    // 2233
    // 2233
    // 000111222
    // 000111222
    // 000111222
    // 333444555
    // 333444555
    // 333444555
    // 666777888
    fn split(&self) -> Vec<Self> {
        let size = if self.size % 2 == 0 { 2 } else { 3 };

        let width = self.size / size;
        let mut result = vec![
            Grid {
                size,
                pixels: HashSet::new()
            };
            width * width
        ];
        for row in 0..self.size {
            for col in 0..self.size {
                let which = (row / size) * width + (col / size);
                println!("which - {:?}", which);
                if self.pixels.contains(&(row, col)) {
                    result[which].pixels.insert((row % size, col % size));
                }
            }
        }

        result
    }
}

#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn test_reverse() {
        let mut grid = vec![vec!['1','2'], vec!['3','4']];
        let art = Art::new();
        art.reverse(&mut grid);
        println!("{:?}", grid);
        assert_eq!(vec![vec!['2','1'], vec!['4','3']], grid);
    }

    #[test]
    fn test_transpose() {
        let mut grid = vec![vec!['1','2'], vec!['3','4']];
        let art = Art::new();
        art.transpose(&mut grid);
        println!("{:?}", grid);
        assert_eq!(1,2)
    }
}

