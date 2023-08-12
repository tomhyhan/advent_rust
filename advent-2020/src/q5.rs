use std::collections::HashSet;

use advent_2020::{Runner, get_file};

struct Boarding {
    seat_codes: Vec<String>,
}

impl Boarding {
    fn new() -> Self {
        let content = get_file("src/input/q5.txt").unwrap();
        let seat_codes = content.lines().map(|line|{
            let mut seat_codes = line.to_string();
            seat_codes = seat_codes.replace('F', "0");
            seat_codes = seat_codes.replace('B', "1");
            seat_codes = seat_codes.replace('L', "0");
            seat_codes = seat_codes.replace('R', "1");
            seat_codes
        }).collect();
        Self { seat_codes }
    }

    fn generate_highest_id(&self) -> i32 {
        let mut id = 0;
        for seat in self.seat_codes.iter() {
            let current_id = i32::from_str_radix(seat, 2).unwrap();
            id = id.max(current_id)
        }
        id
    }

    fn find_my_id(&self) {
        let mut ids = HashSet::new();
        for seat in self.seat_codes.iter() {
            let current_id = i32::from_str_radix(seat, 2).unwrap();
            ids.insert(current_id);
        }
        for id in 0..128*8 {
            if !ids.contains(&id) && ids.contains(&(id-1)) && ids.contains(&(id+1)) {
                println!("id - {id}")
            }             
        }
    }
}


pub struct Q5 {

}

impl Q5 {
    pub fn new() -> Self {
        Q5 {}
    }

    fn part1(&mut self) {
        let boarding = Boarding::new();
        println!("{:?}", boarding.generate_highest_id())
    }
    

    fn part2(&mut self) {
        let boarding = Boarding::new();
        boarding.find_my_id();
    }

}

impl Runner for Q5 {
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
    fn Q5() {
        assert_eq!(1, 1);
    }
}

// solution 1
// struct Boarding {
//     seat_codes: Vec<(String,String)>,
// }

// impl Boarding {
//     fn new() -> Self {
//         let content = get_file("src/input/q5.txt").unwrap();
//         let seat_codes = content.lines().map(|line|{
//             let row_info = line[0..7].to_string();
//             let col_info = line[7..10].to_string();
//             (row_info, col_info)
//         }).collect();
//         Self { seat_codes }
//     }

//     fn generate_highest_id(&self) -> usize {
//         let mut id = usize::MIN;
//         for (row_info, col_info) in self.seat_codes.iter() {
//             let row = self.get_seat_num(row_info, 0, 127);
//             let col = self.get_seat_num(col_info, 0,7);
//             id = id.max(self.generate_id(row, col));
//         }
//         id
//     }

//     fn find_my_seat_id(&self) -> usize{
//         let seat_map = self.create_seat_map();
//         for row in 1..127 {
//             for col in 1..7 {
//                 let is_not_empty = seat_map[row][col];
//                 if is_not_empty {
//                     continue
//                 }
//                 if seat_map[row][col-1] && seat_map[row][col+1] {
//                     return self.generate_id(row, col);
//                 } 
//             }
//         }
//         panic!("could not find your seat id")
//     }

//     fn create_seat_map(&self) -> [[bool;8];128] {
//         let mut seat_map = [[false;8];128];
//         for (row_info, col_info) in self.seat_codes.iter() {
//             let row = self.get_seat_num(row_info, 0, 127);
//             let col = self.get_seat_num(col_info, 0,7);
//             seat_map[row][col] = true
//         }
//         seat_map
//     }

//     fn generate_id(&self, row: usize, col: usize) -> usize {
//         row * 8 + col
//     }

//     fn get_seat_num(&self, info: &str, mut left: usize, mut right: usize)  -> usize {
//         let mut seats = info.chars();
//         let mut seat = 0;
//         while let Some(next) = seats.next() {
//             let mid = (left + right) / 2 ;
//             if next == 'B' || next == 'R'{
//                 left = mid +1;
//                 seat = left;
//             } else {
//                 right = mid;
//                 seat = right;
//             }
//         }
//         seat
//     }
// }