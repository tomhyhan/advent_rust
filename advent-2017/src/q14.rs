//  do again -> using hashset for part2
use std::hash::Hash;

use advent_2017::{Runner, get_file};

pub struct Q14 {
}

struct Knot {
    list: Vec<usize>,
    length: usize,
    skip: usize,
    current_pos: usize
}

impl Knot {
    fn new(elements:usize) -> Self {
        let list: Vec<usize> = (0..elements).into_iter().collect();
        let length = list.len();
        let skip = 0;
        let current_pos = 0;
        Knot {list,length,skip,current_pos}
    }
    
    fn hash(&mut self, des: usize) {
        //  0 1 2  
        for d  in 0..(des/2) {
            let left = (d + self.current_pos) % self.length;
            let right = (des + self.current_pos - d - 1) % self.length;
            self.list.swap(left, right);
        }

        self.current_pos = (self.current_pos + des + self.skip) % self.length;
        self.skip += 1;
    }
}

struct HashString {
    input: Vec<usize>
}

impl HashString {
    fn new(hash_inputs:String) -> Self {  
        let mut input: Vec<usize> = hash_inputs.chars().map(|c|( c as u8 ) as usize).collect();
        let mut x: Vec<usize> = vec![17,31,73,47,23];
        input.append(&mut x);
        HashString {input}
    }
}

fn knot_hash(input:Vec<usize>) -> String {
    let mut knot = Knot::new(256);
    for _ in 0..64 {
        for des in input.iter() {
            knot.hash(*des);
        }
    }

    let mut answer = "".to_string();
    for c in knot.list.chunks(16) {
        answer.push_str(&format!("{:08b}", c.iter().fold(0, |x,y|x ^ *y)));
    }
    answer
}

impl Q14 {
    pub fn new() -> Self{
        Q14{}
    }

    fn part1(&mut self) {
        let mut ones = 0;
        for n in 0..128 {
            let hash_string = HashString::new(format!("ljoxqyyw-{n}"));
            let bits = knot_hash(hash_string.input);
            ones += bits.chars().filter(|p| *p == '1').count();
        }
        println!("{ones}");
    }

    fn part2(&mut self) {
        let mut grid: Vec<Vec<char>> = vec![];
        let mut visited = vec![vec![false;128];128];
        for n in 0..128 {
            let hash_string = HashString::new(format!("ljoxqyyw-{n}"));
            let bits = knot_hash(hash_string.input);
            grid.push(bits.chars().collect());
        }

        let mut cnt = 0;
        for row in 0..128 {
            for col in 0..128 {
                if grid[row][col] == '0' {
                    continue
                }
                if visited[row][col] {
                    continue
                }
                cnt += find_area(row,col, &grid, &mut visited)
            }
        }
        println!("{cnt}");
    }
}

fn find_area(row:usize, col:usize, grid: &Vec<Vec<char>>, visited: &mut Vec<Vec<bool>>) -> i32 {
    let mut stack = vec![(row as i32,col as i32)];

    while let Some((row,col)) = stack.pop() {

        if visited[row as usize][col as usize] {
            continue
        };
        visited[row as usize][col as usize] = true;
        
        for dir in [(0,1),(0,-1),(1,0),(-1,0)] {
            let new_row = row + dir.0;
            let new_col = col + dir.1;
        
            if new_row >= 0 && new_row < 128 && new_col >= 0 && new_col < 128 {
                if grid[new_row as usize][new_col as usize] == '1' {
                    stack.push((new_row,new_col));
                }
            }
        }   
    }
    1
}

impl Runner for Q14 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}