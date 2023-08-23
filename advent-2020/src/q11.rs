use std::collections::HashMap;

use advent_2020::{Runner, get_file};

struct Seating {
    positions: HashMap<(i32,i32), char>,
    grid_size: [usize;4]
}

impl Seating {
    fn new() -> Self {
        let content = get_file("src/input/q11.txt").unwrap();
        let mut positions = HashMap::new();
        let mut grid_size = [usize::MAX,usize::MIN,usize::MAX,usize::MIN];
        content.lines().enumerate().for_each(|(row, line)|{
            line.chars().enumerate().for_each(|(col, c)|{
                grid_size[0] = grid_size[0].min(row);
                grid_size[1] = grid_size[1].max(row); 
                grid_size[2] = grid_size[2].min(col); 
                grid_size[3] = grid_size[3].max(col); 
                positions.insert((row as i32,col as i32), c);
            })
        });
        Self {positions, grid_size}
    }
    
    fn stabilize_seats(&mut self) {
        loop {
            let mut new_positions = HashMap::new();
            let mut change = false;
            for row in self.grid_size[0]..=self.grid_size[1] {
                for col in self.grid_size[2]..=self.grid_size[3] {
                    let row = row as i32;
                    let col = col as i32;
                    let current = self.positions.get(&(row,col)).unwrap();
                    match current {
                        '.' => {
                            new_positions.insert((row,col), *current);
                        },
                        'L' => {
                            let num_of_seats = self.get_neighbors(row, col);
                            if num_of_seats == 0 {
                                new_positions.insert((row,col), '#');
                                change = true;
                            } else {
                                new_positions.insert((row,col), *current);
                            }
                        }
                        '#' => {
                            let num_of_seats = self.get_neighbors(row, col);
                            if num_of_seats >= 5 {
                                new_positions.insert((row,col), 'L');
                                change = true;
                            } else {
                                new_positions.insert((row,col), *current);
                            }
                        }
                        _ => panic!("invalid seat status")
                    }
                }
            }
            if !change {
                println!("{:?}", self.positions.iter().filter(|(_,&v)| v == '#').count());
                break
            }
            self.positions = new_positions;
        }
    }

    // part1
    // fn get_neighbors(&self, row: i32, col: i32) -> i32 {
    //     let mut num_of_seats = 0;
    //     for nr in row-1..=row+1 {
    //         for nc in col-1..=col+1 {
    //             if (nr, nc) == (row,col) {
    //                 continue
    //             }
    //             if let Some(c) = self.positions.get(&(nr,nc)) {
    //                 if *c == '#' {
    //                     num_of_seats += 1;
    //                 }
    //             }
    //         }
    //     }
    //     num_of_seats
    // }
    
    fn get_neighbors(&self, row: i32, col: i32) -> i32 {
        let mut num_of_seats = 0;
        for nr in -1..=1 {
            for nc in -1..=1 {
                if (nr, nc) == (0,0) {
                    continue
                }
                if self.is_seat_in_line(row, col, nr, nc){
                    num_of_seats += 1
                }
            }
        }
        num_of_seats
    }

    fn is_seat_in_line(&self, row: i32, col: i32, row_step: i32, col_step: i32) -> bool{
        let (mut cr, mut cc) = (row + row_step, col + col_step);
        while self.is_in_border(cr, cc) {
            let current = self.positions.get(&(cr, cc)).unwrap(); 
            match current {
                '.' => {}
                '#' => {return true}
                'L' => {return false}
                _ => panic!("invalid seat status")
            }
            cr += row_step;
            cc += col_step;
        }
        false
    }

    fn is_in_border(&self, row:i32, col:i32) -> bool {
        if row >= self.grid_size[0] as i32 && row <= self.grid_size[1] as i32 && col >= self.grid_size[2] as i32 && col <= self.grid_size[3] as i32  {
            return true
        }
        false
    }
}

pub struct Q11 {

}

impl Q11 {
    pub fn new() -> Self {
        Q11 {}
    }

    fn part1(&mut self) {
        let mut seatings = Seating::new();
        seatings.stabilize_seats();
    }

    fn part2(&mut self) {
    }

}

impl Runner for Q11 {
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
    fn Q11() {
        assert_eq!(1, 1);
    }
}