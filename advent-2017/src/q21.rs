use advent_2017::{get_file, Runner};
use itertools::Itertools;
use std::collections::{HashMap, HashSet};

pub struct Q21 {}

struct Art {
    books: HashMap<Vec<Vec<char>>, Vec<Vec<char>>>,
    start_pattern: Vec<Vec<char>>,
}

impl Art {
    fn new() -> Self {
        let mut books = HashMap::new();
        let content = get_file("q21.txt").unwrap();
        content.lines().for_each(|line| parse(line, &mut books));
        let start_pattern = vec![
            vec!['.', '#', '.'],
            vec!['.', '.', '#'],
            vec!['#', '#', '#'],
        ];

        Self {
            books,
            start_pattern,
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

            let mut sub_grids = self.get_sub_grids(&pattern, size);
            for grid in 0..sub_grids.len() {
                let enhancement = self.perform_enhancement(&mut sub_grids[grid]);
                sub_grids[grid] = enhancement;
            }
            let joined_grid = self.join(sub_grids);
            let sum = joined_grid.iter().map(|line|line.iter().filter(|&&p| p == '#').count()).sum::<usize>();
            println!("{:?}", sum);
            pattern = joined_grid;
            cnt -= 1;
        }
    }

    fn join(
        &self,
        sub_grids: Vec<Vec<Vec<char>>>,
    ) -> Vec<Vec<char>> {
        let num_of_rows = (sub_grids.len() as f32).sqrt() * sub_grids[0].len() as f32;
        let mut joined_grid = vec![vec![]; num_of_rows as usize];
        for row in 0..sub_grids.len() {
            for col in 0..sub_grids[0].len() {
                let pos = row / (sub_grids.len() as f32).sqrt() as usize * sub_grids[0].len() + col; 
                joined_grid[pos].extend(sub_grids[row][col].clone());
            }
        }
        joined_grid
    }

    // .#.
    // ..#
    // ###
    fn get_sub_grids(&self, pattern: &Vec<Vec<char>>, size: usize) -> Vec<Vec<Vec<char>>> {
        let len = pattern.len() / size;
        let mut sub_grids = vec![vec![vec!['@'; size]; size]; len * len];

        for row in 0..pattern.len() {
            for col in 0..pattern.len() {
                let pos = row / size * len + col / size;
                sub_grids[pos][row % size][col % size] = pattern[row][col];
            }
        }

        sub_grids
    }

    fn perform_enhancement(&self, grid: &mut Vec<Vec<char>>) -> Vec<Vec<char>> {
        for _ in 0..2 {
            for _ in 0..4 {
                self.transpose(grid);
                self.reverse(grid);
                if self.books.contains_key(grid) {
                    return self.books.get(grid).unwrap().clone();
                }
            }
            self.reverse(grid);
        }
        panic!("match pattern not found")
    }

    fn transpose(&self, grid: &mut Vec<Vec<char>>) {
        let len = grid[0].len();
        for row in 0..grid.len() {
            for col in row..len {
                let temp = grid[col][row];
                grid[col][row] = grid[row][col];
                grid[row][col] = temp;
            }
        }
    }

    fn reverse(&self, grid: &mut Vec<Vec<char>>) {
        let len = grid[0].len();
        for row in 0..grid.len() {
            for col in 0..(len / 2) {
                let temp = grid[row][col];
                grid[row][col] = grid[row][len - col - 1];
                grid[row][len - col - 1] = temp;
            }
        }
    }

    // fn get_grids(
    //     &self,
    //     new_points: Vec<(usize, usize)>,
    //     size: usize,
    //     pattern: &Vec<Vec<char>>,
    // ) -> Vec<Vec<Vec<char>>> {
    //     let mut grids = Vec::new();
    //     for (x, y) in new_points.into_iter() {
    //         let mut grid = Vec::new();
    //         for row in x..x + size {
    //             grid.push(pattern[row][y..y + size].to_vec().clone());
    //         }
    //         grids.push(grid);
    //     }
    //     grids
    // }

    // fn get_points(&self, mut length: usize, divisible: usize) -> Vec<usize> {
    //     let mut points = vec![];
    //     while length > 0 {
    //         length -= divisible;
    //         points.push(length)
    //     }
    //     points
    // }
}

fn parse(line: &str, books: &mut HashMap<Vec<Vec<char>>, Vec<Vec<char>>>) {
    let (left_img, right_img) = line.split_once(" => ").unwrap();
    let left_img = left_img.split('/').map(|p| p.chars().collect()).collect();
    let right_img = right_img.split('/').map(|p| p.chars().collect()).collect();

    books.insert(left_img, right_img);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Image {
    pattern: Vec<Vec<char>>,
}

impl Q21 {
    pub fn new() -> Self {
        Q21 {}
    }

    fn part1(&mut self) {
        let mut art = Art::new();
        art.iteration(18);
    }
}

impl Runner for Q21 {
    fn run(&mut self) {
        self.part1()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_reverse() {
        let mut grid = vec![vec!['1', '2'], vec!['3', '4']];
        let art = Art::new();
        art.reverse(&mut grid);
        println!("{:?}", grid);
        assert_eq!(vec![vec!['2', '1'], vec!['4', '3']], grid);
    }

    // #[test]
    fn test_transpose() {
        let mut grid = vec![vec!['1', '2'], vec!['3', '4']];
        let art = Art::new();
        art.transpose(&mut grid);
        println!("{:?}", grid);
        assert_eq!(1, 2)
    }
}
