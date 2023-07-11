use std::{collections::{HashMap, HashSet}, f64::consts::PI};

use advent_2019::{Runner, get_file};

pub struct Q9 {

}
#[derive(Debug)]
struct Stations {
    map: HashMap<(i32,i32), char>,
    max_row: i32,
    min_row: i32,
    max_col: i32,
    min_col: i32,
}

impl Stations {
    fn new() -> Self {
        let content = get_file("src/input/q9.txt").unwrap();
        let mut max_row = i32::MIN;
        let mut min_row = i32::MAX;
        let mut max_col = i32::MIN;
        let mut min_col = i32::MAX;
        let mut map = HashMap::new();
        content.lines().enumerate().for_each(|(row, line)| {
            line.chars().enumerate().for_each(|(col, char)| {
                max_row = max_row.max(row as i32); 
                min_row = min_row.min(row as i32); 
                max_col = max_col.max(col as i32); 
                min_col = min_col.min(col as i32); 
                if char == '#' {
                    map.insert((row as i32, col as i32), char);
                }
            })
        });
        Self {map, max_row, min_row, max_col, min_col}
    }

    fn find_200th_position(&self, seen: HashSet<(i32,i32)>) -> (i32,i32) {

        let mut sort = vec![];
        for (row,col) in seen.iter() {
            let angle = f64::atan2(*row as f64, *col as f64);
            let mut angle_degrees = f64::to_degrees(angle);
            if angle_degrees > 180.0/2.0 {
                angle_degrees -= 2.0*180.0
            }
            sort.push((angle_degrees, (row,col)))
        }
        
        sort.sort_by(|x,y|y.0.partial_cmp(&x.0).unwrap()); 
        let y = *sort[199].1.0;
        let x = *sort[199].1.1;
        (y,x)

    }

    fn find_best_station(&self) -> (i32, i32, i32, HashSet<(i32, i32)>) {
        let mut max_visible = (0,0,0,HashSet::new());
        for (row, col) in self.map.keys() {
            let (num, seen) = self.count_visible_stations(*row, *col);
            if max_visible.0 < num {
                max_visible = (num, *row, *col, seen)
            }
        }
        max_visible
    }

    fn gcd(&self, mut x:i32, mut y:i32) -> i32{
        if y > x {
            let tmp = x;
            x = y;
            y = tmp;
        }
        while y != 0 {
            let tmp = y;
            y = x % y;
            x= tmp
        }
        x.abs()
    }

    fn count_visible_stations(&self, row:i32 ,col:i32) -> (i32,HashSet<(i32,i32)>) {
        let mut unique_station: HashSet<(i32,i32)> = HashSet::new();

        for (neighbor_row, neighbor_col) in self.map.keys() {
            if (row, col) == (*neighbor_row , *neighbor_col) {continue;}
            let diff1 = neighbor_row - row;
            let diff2 = neighbor_col - col;
            let gcd = self.gcd(diff1, diff2);
            let commons_distance = (-diff1 /gcd, diff2 / gcd);
            unique_station.insert(commons_distance);
        }
        (unique_station.iter().count() as i32, unique_station)
    }
}

impl Q9 {
    pub fn new() -> Self {
        Q9 {}
    }

    fn part1(&mut self) {
        let stations = Stations::new();
        stations.find_best_station();
    }

    fn part2(&mut self) {
        let stations = Stations::new();
        let result = stations.find_best_station();
        let (row,col) = stations.find_200th_position(result.3);
        println!("{:?}",stations.map.get(&(result.1 - row, result.2 + col)));
        // 4 2 
    }

}

impl Runner for Q9 {
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
    fn Q9() {
        assert_eq!(1, 1);
    }
}

// ;p
// #[derive(Debug)]
// struct Stations {
//     map: HashMap<(i32,i32), char>,
//     max_row: i32,
//     min_row: i32,
//     max_col: i32,
//     min_col: i32,
// }

// impl Stations {
//     fn new() -> Self {
//         let content = get_file("src/input/q9.txt").unwrap();
//         let mut max_row = i32::MIN;
//         let mut min_row = i32::MAX;
//         let mut max_col = i32::MIN;
//         let mut min_col = i32::MAX;
//         let mut map = HashMap::new();
//         content.lines().enumerate().for_each(|(row, line)| {
//             line.chars().enumerate().for_each(|(col, char)| {
//                 max_row = max_row.max(row as i32); 
//                 min_row = min_row.min(row as i32); 
//                 max_col = max_col.max(col as i32); 
//                 min_col = min_col.min(col as i32); 
//                 if char == '#' {
//                     map.insert((row as i32, col as i32), char);
//                 }
//             })
//         });
//         Self {map, max_row, min_row, max_col, min_col}
//     }

//     fn find_best_station(&self) -> i32{
//         let mut max_found = 0;

//         for ((row,col), location) in self.map.iter() {
//             let found = self.sight_from_current_station(*row, *col);
//             println!("{:?} {:?}", (row,col), found);
//             max_found = max_found.max(found);
//         }
//         max_found
//     }

//     fn sight_from_current_station(&self, row: i32, col: i32) -> i32{
//         let mut min_row = row - 1;
//         let mut max_row = row + 1;
//         let mut min_col = col - 1;
//         let mut max_col = col + 1;
//         let mut visited = HashSet::new();
//         let mut found = 0;
//         while min_row >= self.min_row || max_row <= self.max_row || min_col >= self.min_col || max_col <= self.max_col {
//             for new_col in min_col..=max_col {
//                 // println!("{:?}",new_col);
//                 if self.map.contains_key(&(min_row, new_col)) {
//                     if visited.contains(&(min_row, new_col)) {
//                         continue
//                     }
//                     visited.insert((min_row, new_col));
//                     found += 1;
//                     self.exclude_stations_in_path(min_row, new_col, min_row-row, new_col-col, &mut visited)
//                 }
//             }

//             for new_row in min_row+1..=max_row {
//                 // println!("{:?}",new_row);
//                 if self.map.contains_key(&(new_row, max_col)) {
//                     if visited.contains(&(new_row, max_col)) {
//                         continue
//                     }
//                     visited.insert((new_row, max_col));
//                     found += 1;
//                     self.exclude_stations_in_path(new_row,max_col, new_row-row, max_col-col, &mut visited)
//                 }
//             }

//             for new_col in (min_col..=max_col-1).rev() {
//                 // println!("{:?}",new_col);
//                 if self.map.contains_key(&(max_row, new_col)) {
//                     if visited.contains(&(max_row, new_col)) {
//                         continue
//                     }
//                     visited.insert((max_row, new_col));
//                     found += 1;
//                     self.exclude_stations_in_path(max_row, new_col, max_row-row, new_col-col, &mut visited)
//                 }
//             }

//             for new_row in (min_row+1..=max_row-1).rev() {
//                 // println!("{:?}",new_row);

//                 if self.map.contains_key(&(new_row, min_col)) {
//                     if visited.contains(&(new_row, min_col)) {
//                     continue
//                     }
//                     visited.insert((new_row, min_col));
//                     found += 1;
//                     self.exclude_stations_in_path(new_row, min_col, new_row-row, min_col-col, &mut visited)
//                 }
//             }
//             min_row -= 1;
//             max_row += 1;
//             min_col -= 1;
//             max_col += 1;
//             // println!("{:?}", min_col)
//             // break
//         } 
//         // println!("v f {:?} {:?}", visited, found);
//         found
//     }

//     fn cover_everything_in_path(&self, mut row:i32 ,mut col:i32,row_slope: i32, col_slope:i32, visited: &mut HashSet<(i32,i32)>) {
//         loop  {
//             row += row_slope;
//             col += col_slope;
//             if row < -100 || row > 100 || col < -100 || col > 100 {
//                 break
//             }
//             visited.insert((row,col));
//         }
//     }

//     // row >= self.min_row && row <= self.max_row && col >= self.min_col && col <= self.max_col
//     fn exclude_stations_in_path(&self,crow:i32, ccol:i32, row: i32, col:i32, visited: &mut HashSet<(i32,i32)>) {
//         // 3 4 , 2 4 => 1, 0
//         // -1 -1 
//         //    0 0 
//         //        1 1
//         match row.abs() == col.abs(){
//             true => {
//                 let row_slope = if row > 0 {1} else {-1};
//                 let col_slope = if col > 0 {1} else {-1};
//                 self.cover_everything_in_path(crow, ccol, row_slope, col_slope, visited)
//             }
//             false => {
//                 match (row,col) {
//                     (0,_) => {
//                         let row_slope = 0;
//                         let col_slope = if col > 0 {1} else {-1};
//                         self.cover_everything_in_path(crow, ccol, row_slope, col_slope, visited)
//                     },
//                     (_,0) => {
//                         let row_slope = if row > 0 {1} else {-1};
//                         let col_slope = 0;
//                         self.cover_everything_in_path(crow, ccol, row_slope, col_slope, visited)
//                     },
//                     (r,c) => {
//                         self.cover_everything_in_path(crow, ccol, r, c, visited)
//                     },
//                 }
//             }
//         }
//     }
    
// }
