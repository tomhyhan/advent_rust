use std::collections::HashSet;

use advent_2018::{Runner, get_file};

// 0,0,0,0
// 3,0,0,0
// 0,3,0,0
// 0,0,3,0
// 0,0,0,3
// 0,0,0,6
// 9,0,0,0
// 12,0,0,0

#[derive(Debug)]
struct Dimensions {
    coords: Vec<Coord>
}

impl Dimensions {
    fn new () -> Self{
        let contents = get_file("src/input/q25.txt").unwrap();
        let coords = contents.lines().map(|line| {
            Coord::new(line.split(",").map(|s|s.parse().unwrap()).collect())
        }).collect();
        Dimensions {coords}
    }

    fn grp_constellation(&self) {
        let mut seen = HashSet::new();
        let mut constellations = vec![];
        for coord in self.coords.iter() {
            if seen.contains(coord) {
                continue
            }
            let constellation = self.find_constellation(&mut seen, coord);
            constellations.push(constellation)
        }
        println!("{:?}", constellations.iter().count());
    }

    fn find_constellation(&self, seen: &mut HashSet<Coord>, coord: &Coord) -> Vec<Coord> {
        let mut constellation = vec![];
        let mut stack = vec![coord.clone()];
        while let Some(current_coord) = stack.pop() {
            if !seen.insert(current_coord) {
                continue
            }
            constellation.push(coord.clone());
            for other_coord in self.coords.iter() {
                if seen.contains(other_coord) {
                    continue
                }
                if current_coord.is_distance_with_in(other_coord) {
                    stack.push(other_coord.clone())
                }
            }
        }

        constellation
    }
}

#[derive(Debug, Hash, Clone, Copy, PartialEq, Eq)]
struct Coord{
    x: i32,
    y: i32,
    z: i32,
    w: i32
}

impl Coord {
    fn new(line:Vec<i32>) -> Self {
        Coord{ x:line[0],y:line[1],z:line[2],w:line[3]}
    }

    fn is_distance_with_in(&self, other:&Coord) -> bool {
        let distance = (self.x - other.x).abs() + (self.y - other.y).abs() + (self.z - other.z).abs() + (self.w - other.w).abs(); 
        if distance <= 3 {true} else {false}
    }
}

pub struct Q25 {

}

impl Q25 {
    pub fn new() -> Self {
        Q25 {}
    }

    fn part1(&mut self) {
        let dimension = Dimensions::new();
        // println!("{:?}", dimension.coords);
        dimension.grp_constellation();
        
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q25 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q25() {
        assert_eq!(1, 1);
    }
}

// fn grp_constellation(&self) {
//     let mut seen = HashSet::new();
//     let mut constellations = vec![];
//     for coord in self.coords.iter() {
//         if seen.contains(coord) {
//             continue
//         }
//         let constellation = self.find_constellation(&mut seen, coord);
//         constellations.push(constellation)
//     }
//     println!("{:?}", constellations.iter().count());
// }

// fn find_constellation(&self, seen: &mut HashSet<Coord>, coord: &Coord) -> Vec<Coord> {
//     let mut constellation = vec![];
//     let mut stack = vec![coord.clone()];
//     while let Some(current_coord) = stack.pop() {
//         if !seen.insert(current_coord) {
//             continue
//         }
//         constellation.push(coord.clone());
//         for other_coord in self.coords.iter() {
//             if seen.contains(other_coord) {
//                 continue
//             }
//             if current_coord.is_distance_with_in(other_coord) {
//                 stack.push(other_coord.clone())
//             }
//         }
//     }

//     constellation
// }