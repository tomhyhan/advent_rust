use std::collections::{HashMap, HashSet};

use advent_2019::{Runner, get_file};


struct Eris {
    bugs: HashSet<(usize,usize,i32)>,
    row: usize,
    col: usize,
}

// ###
// ###
// ###
impl Eris {
    fn new() -> Self {
        let content = get_file("src/input/q24.txt").unwrap();
        let mut bugs = HashSet::new();
        content.lines().enumerate().for_each(|(row,line)| line.chars().enumerate().for_each(|(col, tile)|{
            if tile == '#' {
                bugs.insert((row,col,0));
            }
        }));
        let row = 5;
        let col = 5;
        Self {bugs, row, col}
    }
    
    fn find_next_bugs(&mut self) {
        let min_depth = self.bugs.iter().min_by(|x,y|x.2.cmp(&y.2)).unwrap().2;
        let max_depth = self.bugs.iter().max_by(|x,y|x.2.cmp(&y.2)).unwrap().2;

        let mut new_bugs = HashSet::new();
        for depth in min_depth-1..=max_depth+1 {
            for row in 0..self.row {
                for col in 0..self.col {
                    if row == self.row / 2 && col == self.col / 2 {
                        continue
                    }
                    let bugs_cnt = self.find_neighbor_bugs(row, col, depth);
                    if self.bugs.contains(&(row,col,depth)) {
                        // bug
                        if bugs_cnt == 1 {
                            new_bugs.insert((row,col,depth));
                        }
                    } else {
                        // empty
                        if bugs_cnt == 1 || bugs_cnt == 2 {
                            new_bugs.insert((row,col,depth));
                        }
                    }
                }
            }
        }
        self.bugs = new_bugs
    }

    fn find_neighbor_bugs(&self, row: usize, col: usize, depth:i32 ) -> i32 {
        let mut neighbors = HashSet::new();
        let row_dir = row as i32;
        let col_dir = col as i32;
        for (nrow,ncol) in [(row_dir-1, col_dir),(row_dir+1, col_dir),(row_dir, col_dir-1),(row_dir, col_dir+1)] {
            if ncol < 0 {
                neighbors.insert((2, 1, depth-1));
            } else if nrow < 0 {
                neighbors.insert((1, 2, depth-1));
            } else if ncol >= self.col as i32 {
                neighbors.insert((2, 3, depth-1));
            } else if nrow >= self.row as i32{
                neighbors.insert((3, 2, depth-1));
            } else if nrow == 2 && ncol == 2{
                match (row,col) {
                    (2,1) => {
                        for r in 0..5 {
                            neighbors.insert((r, 0, depth+1));
                        }
                    }
                    (1,2) => {
                        for c in 0..5 {
                            neighbors.insert((0, c, depth+1));
                        }
                    }
                    (2,3) => {
                        for r in 0..5 {
                            neighbors.insert((r, 4, depth+1));
                        }
                    }
                    (3,2) => {
                        for c in 0..5 {
                            neighbors.insert((4, c, depth+1));
                        }
                    }
                    _ => panic!("Error")
                }
            } else {
                neighbors.insert((nrow as usize, ncol as usize, depth));
            }
        }
        let mut bugs = 0;
        for neighbor in neighbors.iter() {
            if self.bugs.contains(neighbor) {
                bugs += 1
            }
        }
        bugs
    }

}

impl Iterator for Eris {
    type Item = HashSet<(usize,usize,i32)>;
    fn next(&mut self) -> Option<Self::Item> {
        self.find_next_bugs();
        Some(self.bugs.clone())
    }
}

pub struct Q24 {

}

impl Q24 {
    pub fn new() -> Self {
        Q24 {}
    }

    fn part1(&mut self) {
        let eris = Eris::new();
        let mut bugs = eris.into_iter();
        for _ in 0..200 {
            bugs.next().unwrap();
        }
        println!("{:?}", bugs.bugs.len())
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q24 {
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
    fn Q24() {
        assert_eq!(1, 1);
    }
}