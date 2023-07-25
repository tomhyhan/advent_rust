use std::{collections::{HashMap, VecDeque, HashSet}, hash::Hash};

use advent_2019::{Runner, get_file};

#[derive(PartialEq, Eq, Debug, Clone)]
enum Ground {
    Empty,
    Open,
    Wall,
    Transport(String)
}
struct DonutMaze {
    donut: HashMap<(i32,i32), Ground>,
    outer_transport: HashMap<String,(i32,i32)>,   
    inner_transport: HashMap<String,(i32,i32)>,   
}

impl DonutMaze {
    fn new() -> Self {
        let content = get_file("src/input/q20.txt").unwrap();
        let grid: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        let mut donut = HashMap::new();
        let mut outer_sizes = [i32::MAX, i32::MIN, i32::MAX, i32::MIN];
        let mut inner_sizes = [i32::MAX, i32::MIN, i32::MAX, i32::MIN];

        // get grid and outer sizes
        for row in 0..grid.len() {
            for col in 0..grid[3].len() {
                let current = grid[row][col];
                let row = row as i32;
                let col = col as i32;
                if current == '.' || current == '#' {
                    let ground = if current == '.' {Ground::Open} else {Ground::Wall};
                    donut.insert((row as i32, col as i32), ground);
                    outer_sizes[0] = outer_sizes[0].min(row);
                    outer_sizes[1] = outer_sizes[1].max(row);
                    outer_sizes[2] = outer_sizes[2].min(col);
                    outer_sizes[3] = outer_sizes[3].max(col);
                }
                if current == ' ' || (current >= 'A' && current <= 'Z') {
                    donut.insert((row,col), Ground::Empty);
                } 
            }
        }

        // get inner sizes
        for row in outer_sizes[0]..=outer_sizes[1] {
            for col in outer_sizes[2]..=outer_sizes[3] {
                let current = donut.get(&(row,col)).unwrap();
                if *current == Ground::Empty {
                    inner_sizes[0] = inner_sizes[0].min(row);
                    inner_sizes[1] = inner_sizes[1].max(row);
                    inner_sizes[2] = inner_sizes[2].min(col);
                    inner_sizes[3] = inner_sizes[3].max(col);
                }
            }
        }
        
        let outer_transport = get_outer_transport(&outer_sizes, &mut donut, &grid);
        let inner_transport = get_inner_transport(&mut inner_sizes, &mut donut, &grid);

        // println!("{:?}", outer_sizes);
        // println!("{:?}", inner_sizes);
        // println!("{:?}", outer_transport);
        // println!("{:?}", inner_transport);
        // println!("{:?}", donut);

        Self {donut,inner_transport,outer_transport}
    }

    fn traverse_maze(&self) {
        let start = *self.outer_transport.get("AA").unwrap();
        // let end = *self.outer_transport.get("ZZ").unwrap();
        let mut queue = HashSet::from([(start, 0, vec![],0)]);
        let mut visited = HashSet::new();
        'outer: loop {
            let current = queue.iter().min_by(|x,y|(x.3 as i32).abs().cmp(&(y.3 as i32).abs())).unwrap().clone();
            
            if !queue.remove(&current) {
                panic!("it should be removed!")
            }

            let  ((row,col), distance, path, level) = current;
            if !visited.insert((row,col, level)) {
                continue
            }   

            for dir in [(-1,0),(1,0),(0,-1),(0,1)] {
                let (nrow, ncol) = (row + dir.0, col + dir.1);
                let next = self.donut.get(&(nrow,ncol)).unwrap();

                match next {
                    Ground::Open => {queue.insert(((nrow,ncol), distance +1,path.clone(), level));},
                    Ground::Transport(letter) => {
                        let mut new_path = path.clone();
                        new_path.push(letter);
                        if level != 0 && (letter == "AA" || letter == "ZZ") {
                            continue
                        } else if letter == "ZZ" {
                            println!("{distance} {path:?}");
                            break 'outer;
                        } else if letter =="AA" {
                            continue
                        }
                        // println!("{letter} {level}");
                        let outer = self.outer_transport.get(letter).unwrap();
                        let inner = self.inner_transport.get(letter).unwrap();

                        if (nrow,ncol) == *outer {
                            queue.insert((*inner, distance +2, new_path, level - 1));
                        } else {
                            queue.insert((*outer, distance +2,new_path, level + 1));
                        }
                    },
                    _ => continue
                }
            }
        }
        // print!("{:?}", self.outer_transport);
    }
}

fn get_outer_transport(outer_sizes: &[i32;4], donut: &mut HashMap<(i32,i32), Ground>, grid: &Vec<Vec<char>>) -> HashMap<String, (i32, i32)>{
    let mut outer_transport = HashMap::new();
    for row in outer_sizes[0]..=outer_sizes[1] {
        for col in outer_sizes[2]..=outer_sizes[3] {
            let current = donut.get(&(row,col)).unwrap();
            let letter; 
            if row == outer_sizes[0] && *current == Ground::Open {
                letter = format!("{}{}", grid[row as usize - 2][col as usize], grid[row as usize - 1][col as usize]);
            } else if row == outer_sizes[1] && *current == Ground::Open {
                letter = format!("{}{}", grid[row as usize + 1][col as usize], grid[row as usize + 2][col as usize]);
            } else if col == outer_sizes[2] && *current == Ground::Open {
                letter = format!("{}{}", grid[row as usize][col as usize - 2], grid[row as usize][col as usize - 1]);
            } else if col == outer_sizes[3] && *current == Ground::Open {
                letter = format!("{}{}", grid[row as usize][col as usize + 1], grid[row as usize][col as usize + 2]);
            } else {continue}
            outer_transport.insert(letter.clone(), (row,col));
            donut.insert((row,col), Ground::Transport(letter));
        }
    }
    outer_transport
}

fn get_inner_transport(inner_sizes: &mut [i32;4], donut: &mut HashMap<(i32,i32), Ground>, grid: &Vec<Vec<char>>) -> HashMap<String, (i32, i32)>{
    let mut inner_transport = HashMap::new();
    inner_sizes[0] -=1;
    inner_sizes[1] +=1;
    inner_sizes[2] -=1;
    inner_sizes[3] +=1;
    for row in inner_sizes[0]..=inner_sizes[1] {
        for col in inner_sizes[2]..=inner_sizes[3] {
            let current = donut.get(&(row,col)).unwrap();
            let letter; 
                if row == inner_sizes[0] && *current == Ground::Open {
                    letter = format!("{}{}", grid[row as usize + 1][col as usize], grid[row as usize + 2][col as usize]);
                } else if row == inner_sizes[1] && *current == Ground::Open {
                    letter = format!("{}{}", grid[row as usize - 2][col as usize], grid[row as usize - 1][col as usize]);
                } else if col == inner_sizes[2] && *current == Ground::Open {
                    letter = format!("{}{}", grid[row as usize][col as usize + 1], grid[row as usize][col as usize + 2]);
                } else if col == inner_sizes[3] && *current == Ground::Open {
                    letter = format!("{}{}", grid[row as usize][col as usize - 2], grid[row as usize][col as usize - 1]);
                } else {continue}
            inner_transport.insert(letter.clone(), (row,col));
            donut.insert((row,col), Ground::Transport(letter));
        }
    }
    inner_transport
}
pub struct Q20 {

}

impl Q20 {
    pub fn new() -> Self {
        Q20 {}
    }

    fn part1(&mut self) {
        let donut_maze = DonutMaze::new();
        donut_maze.traverse_maze();
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

// fn traverse_maze(&self) {
//     let start = *self.outer_transport.get("AA").unwrap();
//     let end = *self.outer_transport.get("ZZ").unwrap();
//     let mut queue = VecDeque::from([(start, 0, vec![],0)]);
//     let mut visited = HashSet::new();
//     visited.insert(start);
//     'outer: while let Some(((row,col), distance, path, level)) = queue.pop_front() {

//         for dir in [(-1,0),(1,0),(0,-1),(0,1)] {
//             let (nrow, ncol) = (row + dir.0, col + dir.1);
//             let next = self.donut.get(&(nrow,ncol)).unwrap();
//             if !visited.insert((nrow,ncol)) {
//                 continue
//             }   
//             match next {
//                 Ground::Open => {queue.push_back(((nrow,ncol), distance +1,path.clone(), level))},
//                 Ground::Transport(letter) => {
//                     let mut new_path = path.clone();
//                     new_path.push(letter);

//                     if level == 0 && letter == "ZZ" {
//                         println!("distance - {:?}", distance +1);
//                         println!("path - {:?}", new_path);
//                         break 'outer
//                     }

//                     let outer = self.outer_transport.get(letter).unwrap();
//                     let inner = self.inner_transport.get(letter).unwrap();
//                     if (row,col) == *outer {
//                         queue.push_back((*inner, distance +2, new_path))
//                     } else {
//                         queue.push_back((*outer, distance +2,new_path))
//                     }
//                 },
//                 _ => continue
//             }
//         }
//     }

// }