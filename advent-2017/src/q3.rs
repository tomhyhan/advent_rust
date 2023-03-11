use std::collections::HashMap;
use std::cmp;
use advent_2017::Runner;

pub struct Q3 {

}

impl Q3 {
    pub fn new() -> Self {
        Q3 {}
    }

    // fn part1(&mut self, max_step: i32) -> HashMap<i32, (i32, i32)>{
    //     let mut square_locations = HashMap::new();
        
    //     let mut step = 0;
    //     let mut start_row = 0;
    //     let mut end_row = 0;
    //     let mut start_col = 0;
    //     let mut end_col = 0;
        
    //     square_locations.insert(0, (0,0));
        
    //     while step <= max_step {

    //         for row in (start_row..end_row).rev() {
    //             step+=1;
    //             square_locations.insert(step, (row,end_col));
    //         }
    //         for col in (start_col..end_col).rev() {
    //             step+=1;
    //             square_locations.insert(step, (start_row,col));
    //         }
    //         for row in start_row+1..end_row {
    //             step+=1;
    //             square_locations.insert(step, (row, start_col));
    //         }
    //         for col in start_row..=end_row {
    //             step+=1;
    //             square_locations.insert(step, (end_row, col));
    //         }

    //         start_row -= 1;
    //         end_row += 1;
    //         start_col -= 1;
    //         end_col += 1;
    //     }

    //     square_locations
    // }

    fn part1(&mut self, steps: i32) -> i32 {
        let go_left = HashMap::from([((0,1),(-1,0)),((-1,0),(0,-1)),((0,-1),(1,0)),((1,0),(0,1))]);
        
        let mut direction = (0,1);
        let mut locations = HashMap::new();

        let mut step = 1;
        let mut x = 0i32;
        let mut y = 0i32;
        locations.insert((0,0), step);
        while step < steps {
            step += 1;
            x += direction.1;
            y += direction.0;
            locations.insert((y,x), step);
            let new_dir = *go_left.get(&direction).unwrap();
            if locations.contains_key(&(y + new_dir.0, x + new_dir.1,)) {
                continue;
            }
            direction = new_dir;
        }
        x.abs() + y.abs()
    }


    fn part2(&mut self, max_val: i32) -> i32{
        let mut square_locations = HashMap::new();
        
        let mut start_row = -1;
        let mut end_row = 1;
        let mut start_col = -1;
        let mut end_col = 1;
        let mut val = 0;


        square_locations.insert((0,0), 1);
        
        loop {

            for row in (start_row..end_row).rev() {
                let sum = get_neightbor_sum((row, end_col), &square_locations);
                square_locations.insert((row, end_col), sum);
                val = cmp::max(val, sum);
                if sum > max_val {
                    return sum
                }
            }
            for col in (start_col..end_col).rev() {
                let sum = get_neightbor_sum((start_row, col), &square_locations);
                square_locations.insert((start_row, col), sum);
                val = cmp::max(val, sum);
                if sum > max_val {
                    return sum
                }
            }
            for row in start_row+1..end_row {
                let sum = get_neightbor_sum((row, start_col), &square_locations);
                square_locations.insert((row, start_col), sum);
                val = cmp::max(val, sum);
                if sum > max_val {
                    return sum
                }
            }
            for col in start_row..=end_row {
                let sum = get_neightbor_sum((end_row, col), &square_locations);
                square_locations.insert((end_row, col), sum);
                val = cmp::max(val, sum);
                if sum > max_val {
                    return sum
                }
            }

            start_row -= 1;
            end_row += 1;
            start_col -= 1;
            end_col += 1;
            
        }

    }

}

impl Runner for Q3 {
    fn run(&mut self) {
        let locations = self.part1(265149);
        println!("{locations}");
        println!("{:?}", self.part2(265149));
    }
}


fn get_neightbor_sum(cor: (i32,i32), square_locations: &HashMap<(i32,i32), i32>) -> i32{
    let mut sum = 0;
    for (row, col) in [(1,-1),(1,0),(1,1),(0,-1),(0,1),(-1,-1),(-1,0),(-1,1)] {
        let new_row = cor.0 + row;
        let new_col = cor.1 + col;
        match square_locations.get(&(new_row, new_col)) {
            Some(val) => {
                sum += val
            }
            None => continue
        };
    }
    sum
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn square_is_1()  {
        let mut q3 = Q3::new();
        let steps = q3.part1(0);
        assert_eq!(steps, 0);
    }    

    #[test]
    fn square_is_12()  {
        let mut q3 = Q3::new();
        let steps = q3.part1(12);
        assert_eq!(steps, 3);
    }    


    #[test]
    fn square_is_1024()  {
        let mut q3 = Q3::new();
        let steps = q3.part1(1024);
        assert_eq!(steps, 31);
    }   

}