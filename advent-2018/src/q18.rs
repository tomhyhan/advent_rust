use std::collections::HashMap;

use advent_2018::{Runner, get_file};

pub struct Q18 {

}

struct Construction {
    acres: HashMap<(i32,i32), Acre>,
    max_row: usize,
    max_col: usize
}

impl Construction {
    fn new() -> Self {
        let content = get_file("src/input/q18.txt").unwrap();
        let mut max_row = 0;
        let mut max_col = 0;
        let mut acres = HashMap::new();
        for (row, line) in content.lines().enumerate() {
            max_row = max_row.max(row);
            for (col, c) in line.chars().enumerate() {
                max_col = max_col.max(col);
                
                match c {
                    '.' => acres.insert((row as i32, col as i32), Acre::Ground),
                    '#' => acres.insert((row as i32, col as i32), Acre::Lumberyard),
                    '|' => acres.insert((row as i32, col as i32), Acre::Tree),
                    _ => panic!("unknown acre!")
                };
            }
        }

        Self {acres, max_col, max_row}
    }

    fn changes (&mut self) {
        let mut minute = 0;
        let mut visited = vec![];
        while minute < 1000 {
            let mut new_acres = self.acres.clone();
            
            for row in 0..=self.max_row {
                for col in 0..=self.max_col {
                    // search and count neighboring acres
                    let neighbors = self.search_and_count(row, col);

                    // check if current acre satisfy the condition
                    //  . => '|' if 3 or more '|', Otherwise nothing
                    //  | => # if 3 or more #, Otherwise nothing
                    //  # => # if 1 or more # and 1 or more |, Otherwise becomes .

                    let current_acre = self.acres.get(&(row as i32, col as i32)).unwrap();
                    match current_acre {
                        Acre::Ground => {
                            if *neighbors.get(&Acre::Tree).unwrap() >= 3 {
                                new_acres.insert((row as i32, col as i32), Acre::Tree);
                            }
                        },
                        Acre::Tree => {
                            if *neighbors.get(&Acre::Lumberyard).unwrap() >= 3 {
                                new_acres.insert((row as i32, col as i32), Acre::Lumberyard);
                            }
                        },
                        Acre::Lumberyard => {
                            if *neighbors.get(&Acre::Tree).unwrap() >= 1 && *neighbors.get(&Acre::Lumberyard).unwrap() >= 1 {
                                new_acres.insert((row as i32, col as i32), Acre::Lumberyard);
                            } else {
                                new_acres.insert((row as i32, col as i32), Acre::Ground);

                            }
                        },
                    }
                }
            }
            self.acres = new_acres;
            minute += 1;
            let lumberyards = self.acres.iter().filter(|(_, &v)| v == Acre::Lumberyard).count();
            let woods = self.acres.iter().filter(|(_, &v)| v == Acre::Tree).count();
            // part 2
            if !visited.contains(&(self.acres)) {
                visited.push(self.acres.clone());
            } else {
                println!("{:?}", lumberyards * woods);
                println!("{:?}", visited.iter().position(|ac| ac == &self.acres).unwrap());
                break;
            }
        }
        // 45627
        let repeat = &visited[496..];
        println!("len {:?}", repeat.len());
        println!("{:?}", (1000000000 - 496 - 1 ) % 28);
        let lumberyards = &visited[496..][27].iter().filter(|(_, &v)| v == Acre::Lumberyard).count();
        let woods = &visited[496..][27].iter().filter(|(_, &v)| v == Acre::Tree).count();
        println!("{:?}", lumberyards * woods)
    }

    fn search_and_count(&self, row: usize, col: usize) -> HashMap<Acre, i32>{
        let mut neighbors = HashMap::from([(Acre::Ground, 0), (Acre::Lumberyard, 0),(Acre::Tree, 0)]);
        for (n_row, n_col) in [(-1,-1),(-1,0),(-1,1),(0,-1),(0,1),(1,-1),(1,0),(1,1)] {
            if let Some(acre) = self.acres.get(&(row as i32 + n_row, col as i32 + n_col)) {
                match acre {
                    Acre::Ground => *neighbors.get_mut(&Acre::Ground).unwrap() += 1,
                    Acre::Lumberyard => *neighbors.get_mut(&Acre::Lumberyard).unwrap() += 1,
                    Acre::Tree => *neighbors.get_mut(&Acre::Tree).unwrap() += 1
                }
            }
        }
        neighbors
    }
}
#[derive(Debug, Hash, PartialEq, Eq, Clone, Copy)]
enum Acre {
    Ground,
    Tree,
    Lumberyard
}
impl Q18 {
    pub fn new() -> Self {
        Q18 {}
    }

    fn part1(&mut self) {
        let mut construction = Construction::new();
        construction.changes();
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q18 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q18() {
        assert_eq!(1, 1);
    }
}