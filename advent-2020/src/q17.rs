use std::collections::HashSet;

use advent_2020::{Runner, get_file};

struct Cube {
    actives: HashSet<(i32,i32,i32,i32)>,
    range: Range,
}

impl Cube {
    fn new() -> Self {
        let content = get_file("src/input/q17.txt").unwrap();
        let mut actives = HashSet::new();
        let mut range = Range::default();
        content.lines().enumerate().for_each(|(row,line)| {
            line.chars().enumerate().for_each(|(col, char)|{
                if char == '#'{
                    actives.insert((col as i32, row as i32,0,0));
                }
                range.range(col as i32, row as i32);
            });
        });
        Self {actives, range}
    }

    fn change_state(&mut self, cycle: usize) {
        for _ in 0..cycle {
            self.range.increase_range();
            let mut new_cube = self.actives.clone();
            for x in self.range.low_x..=self.range.high_x {
                for y in self.range.low_y..=self.range.high_y {
                    for z in self.range.low_z..=self.range.high_z {
                        for w in self.range.low_w..=self.range.high_w {
                            let actives = self.find_active_neighbors(x,y,z,w);
                            match self.actives.contains(&(x,y,z,w)) {
                                true => {
                                    if actives == 2 || actives == 3 {
                                        continue
                                    }
                                    new_cube.remove(&(x,y,z,w));
    
                                },
                                false => {
                                    if actives == 3 {
                                        new_cube.insert((x,y,z,w));    
                                    }
                                }
                            } 
                        }
                    }
                }
            }
            self.actives =new_cube
        }
        println!("actives - {}", self.actives.len());
    }

    fn find_active_neighbors(&self, x:i32, y:i32, z:i32, w:i32) -> i32 {
        // what are other ways to find neighbors?
        let neighbors = self.get_neighbors(x, y, z, w);
        let mut actives = 0;
        for neighbor in neighbors {
            if self.actives.contains(&neighbor) {
                actives += 1;
            }
        }
        actives
    }   

    fn get_neighbors(&self, x:i32, y:i32, z:i32, w:i32) -> Vec<(i32, i32, i32, i32)> {
        let mut neighbors = vec![];
        for nx in x-1..=x+1 {
            for ny in y-1..=y+1 {
                for nz in z-1..=z+1 {
                    for nw in w-1..=w+1 {
                        if (x,y,z,w) == (nx,ny,nz,nw) {
                            continue
                        }
                        neighbors.push((nx,ny,nz,nw));
    
                    }
                }
            }
        }
        neighbors
    }
}

#[derive(Default, Debug)]
struct Range {
    low_x: i32,
    high_x: i32,
    low_y: i32,
    high_y: i32,
    low_z: i32,
    high_z: i32,
    low_w: i32,
    high_w: i32,
}

impl Range {
    fn range(&mut self,col: i32, row: i32 ) {
        self.low_x = self.low_x.min(col);
        self.high_x = self.high_x.max(col);
        self.low_y = self.low_y.min(row);
        self.high_y = self.high_y.max(row);
    }

    fn increase_range(&mut self) {
        self.low_x -= 1;
        self.high_x += 1;
        self.low_y -= 1;
        self.high_y += 1;
        self.low_z -= 1;
        self.high_z += 1;
        self.low_w -= 1;
        self.high_w += 1;
    }
}

pub struct Q17 {

}

impl Q17 {
    pub fn new() -> Self {
        Q17 {}
    }

    fn part1(&mut self) {
        let mut cube = Cube::new();
        cube.change_state(6);
        // println!("{:?}", cube.actives);
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q17 {
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
    fn Q17() {
        assert_eq!(1, 1);
    }
}

// part 1 3 dimension
// struct Cube {
//     actives: HashSet<(i32,i32,i32)>,
//     range: Range,
// }

// impl Cube {
//     fn new() -> Self {
//         let content = get_file("src/input/q17.txt").unwrap();
//         let mut actives = HashSet::new();
//         let mut range = Range::default();
//         content.lines().enumerate().for_each(|(row,line)| {
//             line.chars().enumerate().for_each(|(col, char)|{
//                 if char == '#'{
//                     actives.insert((col as i32, row as i32,0));
//                 }
//                 range.range(col as i32, row as i32);
//             });
//         });
//         Self {actives, range}
//     }

//     fn change_state(&mut self, cycle: usize) {
//         for _ in 0..cycle {
//             self.range.increase_range();
//             let mut new_cube = self.actives.clone();
//             for x in self.range.low_x..=self.range.high_x {
//                 for y in self.range.low_y..=self.range.high_y {
//                     for z in self.range.low_z..=self.range.high_z {
//                         let actives = self.find_active_neighbors(x,y,z);
//                         match self.actives.contains(&(x,y,z)) {
//                             true => {
//                                 if actives == 2 || actives == 3 {
//                                     continue
//                                 }
//                                 new_cube.remove(&(x,y,z));

//                             },
//                             false => {
//                                 if actives == 3 {
//                                     new_cube.insert((x,y,z));    
//                                 }
//                             }
//                         }
//                     }
//                 }
//             }
//             self.actives =new_cube
//         }
//         println!("actives - {}", self.actives.len());
//     }

//     fn find_active_neighbors(&self, x:i32, y:i32, z:i32) -> i32 {
//         // what are other ways to find neighbors?
//         let neighbors = self.get_neighbors(x, y, z);
//         let mut actives = 0;
//         for neighbor in neighbors {
//             if self.actives.contains(&neighbor) {
//                 actives += 1;
//             }
//         }
//         actives
//     }   

//     fn get_neighbors(&self, x:i32, y:i32, z:i32) -> Vec<(i32, i32, i32)> {
//         let mut neighbors = vec![];
//         for nx in x-1..=x+1 {
//             for ny in y-1..=y+1 {
//                 for nz in z-1..=z+1 {
//                     if (x,y,z) == (nx,ny,nz) {
//                         continue
//                     }
//                     neighbors.push((nx,ny,nz));
//                 }
//             }
//         }
//         neighbors
//     }
// }

// #[derive(Default, Debug)]
// struct Range {
//     low_x: i32,
//     high_x: i32,
//     low_y: i32,
//     high_y: i32,
//     low_z: i32,
//     high_z: i32,
// }

// impl Range {
//     fn range(&mut self,col: i32, row: i32 ) {
//         self.low_x = self.low_x.min(col);
//         self.high_x = self.high_x.max(col);
//         self.low_y = self.low_y.min(row);
//         self.high_y = self.high_y.max(row);
//     }

//     fn increase_range(&mut self) {
//         self.low_x -= 1;
//         self.high_x += 1;
//         self.low_y -= 1;
//         self.high_y += 1;
//         self.low_z -= 1;
//         self.high_z += 1;
//     }
// }