use std::collections::{HashMap, HashSet};

use advent_2020::{Runner, get_file};

struct Lobby {
    directions: Vec<Vec<String>>
}

impl Lobby {
    fn new() -> Self {
        let content = get_file("src/input/q24.txt").unwrap();

        let directions = content.lines().map(|line| split_line(line)).collect();
        Self {directions}
    }
    
    fn find_black_tiles(&self) {
        let dirs = HashMap::from([("nw", (-1,-1)),("ne", (-1,1)),("e", (0,2)),("se", (1,1)),("sw", (1,-1)),("w", (0,-2))]);
        let mut tiles = HashMap::new();

        for token in self.directions.iter() {
            let mut tile = (0,0);
            for dir in token.iter() {
                let (row,col) = dirs.get(dir.as_str()).unwrap();
                tile.0 += *row;
                tile.1 += *col;
            }
            if tiles.contains_key(&tile) {
                let status: bool = *tiles.get(&tile).unwrap();
                *tiles.get_mut(&tile).unwrap() = !status;
            } else {
                tiles.insert(tile, false);
            }
        }
        
        
        // println!("{:?}", tiles.values().filter(|&&b| b==false).count());
        let blacks = tiles.into_iter().filter(|(k,v)| *v == false).map(|(k,_)| k).collect();
        self.black_after_days(100, blacks)
    }

    fn black_after_days(&self, days: usize, mut blacks: HashSet<(i32,i32)>){
        for _ in 0..days {
            let mut max_row = i32::MIN;
            let mut min_row = i32::MAX;
            let mut max_col = i32::MIN;
            let mut min_col = i32::MAX;

            for (row, col) in blacks.iter() {
                max_row = max_row.max(*row); 
                min_row = min_row.min(*row); 
                max_col = max_col.max(*col); 
                min_col = min_col.min(*col); 
            }

            // println!("{max_row} {min_row} {max_col} {min_col}");
            let mut new_blacks = blacks.clone();
            for row in min_row-1..=max_row+1 {
                for col in min_col-1..=max_col+1 {
                    let number_of_blacks = self.get_blacks(row,col,&blacks);
                    match blacks.contains(&(row,col)) {
                        true => {
                            if number_of_blacks == 0 || number_of_blacks > 2 {
                                new_blacks.remove(&(row,col));
                            }
                        }
                        false => {
                            if number_of_blacks == 2{
                                new_blacks.insert((row,col));
                            }
                        }
                    }
                }
            }
            blacks = new_blacks
        }
        println!("{:?}", blacks.len())
    }

    fn get_blacks(&self, row: i32, col:i32, blacks: &HashSet<(i32,i32)>) -> i32{
        let mut num_of_blacks = 0;
        for (nrow, ncol) in [(-1,-1),(-1,1),(0,2),(1,1),(1,-1),(0,-2)] {
            let r = row + nrow;
            let c = col + ncol;
            if blacks.contains(&(r,c)) {
                num_of_blacks += 1;
            }
        }
        num_of_blacks
    }
}

fn split_line(line: &str) -> Vec<String> {
    // e, se, sw, w, nw, and ne
    let chars: Vec<_>= line.chars().collect();
    let mut i = 0;
    let mut tokens = vec![];
    while i < chars.len() {
        let c = chars[i];

        match c {
            'e' | 'w' => {
                tokens.push(c.to_string());
                i += 1
            }
            's' | 'n'=> {
                let next_c = chars[i+1];
                tokens.push(format!("{c}{next_c}"));
                i += 2
            }
            _ => unreachable!()
        }
    }
    tokens
}


pub struct Q24 {

}

impl Q24 {
    pub fn new() -> Self {
        Q24 {}
    }

    fn part1(&mut self) {
        let lobby = Lobby::new();
        lobby.find_black_tiles();
        // println!("{:?}", split_line("esenee"))
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