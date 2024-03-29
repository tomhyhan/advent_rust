use std::collections::{HashMap, HashSet, BinaryHeap};

use advent_2018::{Runner, get_file};

pub struct Q22 {

}

#[derive(Debug,Clone, Copy, PartialEq, Eq)]
enum Erosion {
    Rocky,
    Wet,
    Narrow,
    Empty
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Equipment {
    Torch,
    Climb,
    Neither
}

struct Maze {
    map: Vec<Vec<Erosion>>,
    depth: usize,
    target_y: usize,
    target_x: usize,
    extra: usize
}

// 0,0 => g index = 0
// 0,_ => g index = 16807 
// _,0 => g index = 48271
// target => g_index = 0
// _ => g_index = map[y-1][x] + map[y][x-1]

impl Maze {
    fn new() -> Self{
        let content = get_file("src/input/q22.txt").unwrap();
        let info: Vec<_>= content.lines().collect();
        let (_, depth) = info[0].split_once(" ").unwrap(); 
        let (_, targets) = info[1].split_once(" ").unwrap();
        let (target_x, target_y) = targets.split_once(",").unwrap(); 
        let depth = depth.parse::<usize>().unwrap();
        let target_y = target_y.parse::<usize>().unwrap();
        let target_x = target_x.parse::<usize>().unwrap();
        let extra = 1000;
        let map = vec![vec![Erosion::Empty;target_x+1+extra]; target_y+1+extra];
        Maze {map, target_y, target_x, depth, extra}
    }

    fn drawing(&mut self, extra: usize) {
        let mut erosions = vec![vec![0;self.target_x+1+self.extra]; self.target_y+1+self.extra];
        for row in 0..=self.target_y+self.extra {
            for col in 0..=self.target_x+self.extra {
                match (row,col) {
                    (0,0)  => {
                        let g_index = 0;
                        let region_type = self.get_region_type(g_index, &mut erosions, row, col);
                        self.map[row][col] = region_type;                       
                        self.map[self.target_y][self.target_x] = region_type                       
                    }
                    (0,x) => {
                        let g_index = 16807 * x;
                        let region_type = self.get_region_type(g_index, &mut erosions, row, col);
                        self.map[row][col] = region_type                       
                    }
                    (y,0) => {
                        let g_index = 48271 * y;
                        let region_type = self.get_region_type(g_index, &mut erosions, row, col);
                        self.map[row][col] = region_type 
                    }
                    (y,x)=> {
                        if y == self.target_y && x == self.target_x {
                            continue
                        }
                        let g_index = erosions[row-1][col] * erosions[row][col-1];
                        let region_type = self.get_region_type(g_index, &mut erosions, row, col);
                        self.map[row][col] = region_type 
                    }
                }
            }
        }
    }
    fn get_region_type(&self, g_index: usize, erosions: &mut Vec<Vec<usize>>, row: usize, col: usize) -> Erosion {
        let erosion = self.calculate_erosion(g_index);
        erosions[row][col] = erosion;
        let erosion_lvl = erosion % 3;
        match erosion_lvl {
            0 => Erosion::Rocky,
            1 => Erosion::Wet,
            2 => Erosion::Narrow,
            _ => panic!("invalid erosion level!")
        }
    }

    fn calculate_erosion(&self, g_index: usize) -> usize {
        let modulo = 20183;
        (g_index + self.depth) % modulo
    }
}



impl Q22 {
    pub fn new() -> Self {
        Q22 {}
    }

    fn part1(&mut self) {
        let mut maze = Maze::new();
        maze.drawing(0);
        // println!("{:?}", maze.map);
        let mut risk = 0;
        // println!("{:?}", maze.target_y);
        // println!("{:?}", maze.target_x);
        for row in 0..=maze.target_y {
            for col in 0..=maze.target_x {
                match maze.map[row][col] {
                    Erosion::Wet => risk += 1,
                    Erosion::Narrow => risk += 2,
                    _ => {} 
                }
            }
        }
        println!("{:?}", risk)
    }

    fn part2(&mut self) {
        let mut maze = Maze::new();
        maze.drawing(7);
        // println!("{:?}", maze.map);
        dijkstra(maze.map, maze.target_y, maze.target_x);
    }

}

fn get_weight(y:i32,x:i32,target_y:i32,target_x:i32) -> i32 {
    (target_y - y).abs() + (target_x - x).abs() 
}
fn dijkstra(map: Vec<Vec<Erosion>>, target_y:usize, target_x:usize) {
    let mut queue = HashSet::new();
    let mut visited = HashMap::new();
    queue.insert((0,0,0,Equipment::Torch));
    // visited.insert((0,0), 0);
    let mut cnt = 0;
    let equips = [Equipment::Neither, Equipment::Torch, Equipment::Climb];
    let areas = [Erosion::Rocky, Erosion::Wet, Erosion::Narrow];

    loop {
        let current = queue.iter().min_by(|x,y|x.0.cmp(&y.0)).unwrap().clone();
        let (minutes, row, col, equip) = current;

        if !queue.remove(&current) {
            panic!("something wrong!")
        }
        if let Some(old_minutes) = visited.get(&(row, col, equip)){
            if *old_minutes <= minutes {
                continue
            }
        }
        visited.insert((row,col, equip), minutes);

        if row == target_y && col == target_x && equip == Equipment::Torch {
            println!("distance {:?}", minutes);
            println!("equip {:?}", equip);
            break                
        }

        for i in  0..3{
            // areas[i] != map[row][col]
            if equips[i] != equip && areas[i] != map[row][col]{
                queue.insert((minutes +7, row, col, equips[i].clone()));
            }
        }
        
        
        for (dir_y, dir_x) in [(-1,0), (1,0),(0,-1),(0,1)] {
            let n_y = row as i32 + dir_y;
            let n_x = col as i32 + dir_x;
            // if n_y < 0 || n_x < 0  || n_y > target_y as i32+ 90 || n_x > target_x as i32 + 90{
            //     continue
            // } 
            if n_y < 0 {
                continue
            }
            if n_x < 0 {
                continue
            }
            let n_y = n_y as usize;
            let n_x = n_x as usize;
            let neighbor = map[n_y][n_x];

            match neighbor {
                Erosion::Rocky => {if equip == Equipment::Neither {continue}}
                Erosion::Wet => {if equip == Equipment::Torch {continue}}
                Erosion::Narrow => {if equip == Equipment::Climb {continue}},
                Erosion::Empty => {}
            }
            
            queue.insert((minutes + 1, n_y, n_x, equip.clone()));
        }
    }
}

impl Runner for Q22 {
    fn run(&mut self) {
        self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q22() {
        assert_eq!(1, 1);
    }
}


 // match equip {
//     Equipment::Torch => {
//         match neighbor {
//             Erosion::Wet => {
//                 current_distance += 1 + 7;
//                 let current_weight = current_distance + get_weight(n_y as i32, n_x as i32, target_y as i32, target_x as i32);
//                 if old_distance > current_distance {
//                     queue.insert((n_y,n_x,Equipment::Climb,current_distance, current_weight));
//                     queue.insert((n_y,n_x,Equipment::Neither,current_distance, current_weight));
//                     visited.insert((n_y,n_x), current_distance);
//                 }
//             }
//             _ => {
//                 current_distance += 1;
//                 let current_weight = current_distance + get_weight(n_y as i32, n_x as i32, target_y as i32, target_x as i32);
//                 if old_distance > current_distance {
//                     queue.insert((n_y,n_x,equip,current_distance, current_weight));
//                     visited.insert((n_y,n_x), current_distance);
//                 }
//             }
//         }
//     }
//     Equipment::Climb => {
//         match neighbor {
//             Erosion::Narrow => {
//                 current_distance += 1 + 7;
//                 let current_weight = current_distance + get_weight(n_y as i32, n_x as i32, target_y as i32, target_x as i32);
//                 if old_distance > current_distance {
//                     queue.insert((n_y,n_x,Equipment::Torch,current_distance, current_weight));
//                     queue.insert((n_y,n_x,Equipment::Neither,current_distance, current_weight));
//                     visited.insert((n_y,n_x), current_distance);
//                 }
//             }
//             _ => {
//                 current_distance += 1;
//                 let current_weight = current_distance + get_weight(n_y as i32, n_x as i32, target_y as i32, target_x as i32);
//                 if old_distance > current_distance {
//                     queue.insert((n_y,n_x,equip,current_distance, current_weight));
//                     visited.insert((n_y,n_x), current_distance);
//                 }
//             }
//         }
//     }
//     Equipment::Neither => {
//         match neighbor {
//             Erosion::Rocky=> {
//                 current_distance += 1 + 7;
//                 let current_weight = current_distance + get_weight(n_y as i32, n_x as i32, target_y as i32, target_x as i32);
//                 if old_distance > current_distance {
//                     queue.insert((n_y,n_x,Equipment::Torch,current_distance, current_weight));
//                     queue.insert((n_y,n_x,Equipment::Climb,current_distance, current_weight));
//                     visited.insert((n_y,n_x), current_distance);
//                 }
//             } 
//             _ => {
//                 current_distance += 1;
//                 let current_weight = current_distance + get_weight(n_y as i32, n_x as i32, target_y as i32, target_x as i32);
//                 if old_distance > current_distance {
//                     queue.insert((n_y,n_x,equip,current_distance, current_weight));
//                     visited.insert((n_y,n_x), current_distance);
//                 }
//             }
//         }
//     }
// }