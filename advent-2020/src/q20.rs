use std::collections::{HashMap, HashSet};
use regex::Regex;
use lazy_static::lazy_static;
use advent_2020::{Runner, get_file};

struct CameraArray {
    images: HashMap<i64, Vec<Vec<char>>>
}

impl CameraArray {
    fn new() -> Self{
        let content = get_file("src/input/q20.txt").unwrap();
        let tiles: Vec<_> = content.split("\r\n\r\n").collect();
        let mut images = HashMap::new(); 
        tiles.iter().for_each(|tile| {
            lazy_static! {
                static ref RE: Regex = Regex::new(r"\d+").unwrap();
            }
            let mut tile_lines = tile.lines();
            let first_line = tile_lines.next().unwrap();
            let num = RE.find(first_line).unwrap().as_str();
            let id = num.parse::<i64>().unwrap();
            let image: Vec<Vec<char>>= tile_lines.map(| line| line.chars().collect()).collect();
            images.insert(id, image);
        });
        Self { images }
    }

    fn rotate(&self, image: &mut Vec<Vec<char>>) {
        image.reverse()
    }

    fn flip(&self, image: &mut Vec<Vec<char>>) {
        for tile in image.iter_mut() {
            tile.reverse();
        }
    }

    fn match_y(&self, side: &Vec<char>, id: &i64) -> bool {
        for (cid, image) in self.images.iter() {
            if id == cid {
                continue
            }
            let rotations = self.generate_4sides(image.clone());
            for rotation in rotations {
                if side == &rotation[9] {
                    return true
                }
            }
        }            
        false
    }

    fn match_left(&self, side: &Vec<char>, seen: &mut HashSet<i64>) -> bool {
        for (cid, image) in self.images.iter() {
            if seen.contains(cid) {
                continue
            }
            let rotations = self.generate_4sides(image.clone());
            for rotation in rotations {
                let left: Vec<_> = rotation.iter().map(|line|line[0]).collect();
                if side == &left {
                    return true
                }
            }
        }            
        false
    }

    fn generate_4sides(&self, mut image: Vec<Vec<char>>) -> Vec<Vec<Vec<char>>> {
        let mut rotations = vec![];    
        for i in 0..4 {
            if i % 2 == 0 {
                self.flip(&mut image);      
            } else {
                self.rotate(&mut image)      
            }
            rotations.push(image.clone());
        }
        rotations
    }

    // fn top_left(&self) {
    //     for (id, image) in self.images.iter() {
    //         let rotations = self.generate_4sides(image.clone());
    //         for rotation in rotations {
    //             let top = &rotation[0];
    //             let left: Vec<_> = rotation.iter().map(|line|line[0]).collect();
    //             if self.match_y(top,id) {
    //                 continue
    //             }
    //             if self.match_x(&left,id) {
    //                 continue
    //             }
    //             println!("id - {:?}", id);
    //             return
    //         }
    //     }
    // }

    fn create_image(&self, mut row: usize, mut col: usize, board: &mut Vec<Vec<Vec<Vec<char>>>>, seen: &mut HashSet<i64>) {
        if seen.len() == 9 {
            println!("{:?}", board);
            return
        }
        if row == 2 && col == 3 {
            println!("{:?}", board)
        }
        if col > 3 {
            row += 1;
            col = 0
        }

        for (id, image) in self.images.iter() {
            if seen.contains(id) {
                continue
            }
            let rotations = self.generate_4sides(image.clone());
            for rotation in rotations {
                match (row,col) {
                    (0,0) => {
                        board[row][col] = rotation.clone();
                        seen.insert(*id);
                        self.create_image(row, col+1, board, seen);
                        seen.remove(id);
                        board[row][col].clear()
                    }
                    (0,_) => {
                        let prev = &board[row][col-1];
                        
                    }
                    (_,0) => {

                    }
                    (_,_) => {
                        
                    }
                }                 
            }
        }
    }
}

pub struct Q20 {

}

impl Q20 {
    pub fn new() -> Self {
        Q20 {}
    }

    fn part1(&mut self) {
        let camera = CameraArray::new();
        // println!("{:?}", camera.images.len())
        // camera.top_left()
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q20 {
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
    fn Q20() {
        assert_eq!(1, 1);
    }
}