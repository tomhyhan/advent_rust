// again
use std::time::Instant;

use advent_2018::{Runner, get_file};

pub struct Q8 {

}

#[derive(Debug)]
struct File {
    data: Vec<i32>
}

impl File {
    fn new() -> Self{
        let contents = get_file("src/input/q8.txt").unwrap();
        let data = contents.split_whitespace().map(|num| num.parse::<i32>().unwrap()).collect();
        Self {data}
    }
}

impl Q8 {
    pub fn new() -> Self {
        Q8 {}
    }

    fn part1(&mut self) { 
        let mut file = File::new(); 
        let mut metadata = 0; 

        while !file.data.is_empty() {
            let zero_idx= find_zero_idx(&mut file.data); 
            metadata += get_sum(zero_idx, &mut file.data);
        }
        println!("{:?}", metadata);
    }
    
    fn part2(&mut self) {
        let mut file = File::new();
        let r = find_refence_sum(&mut file.data, 0);
        println!("{:?}", r);
    }

    // method2 of part2
    fn part3(&mut self) {
        let mut file = File::new();

        let (total ,value, data) = memory_maneuver(&file.data);
        println!("{total} {value}");
    }
}

// 2 3 0 3 10 11 12 1 1 0 1 99 2 1 1 2
fn memory_maneuver(data: &[i32]) -> (i32, i32 ,&[i32]) {
    let children = data[0];
    let metadata = data[1];
    let mut rest = &data[2..]; 
    let mut total = 0;
    let mut references = vec![];

    // println!("{:?}", children);
    // println!("{:?}", metadata);
    // println!("{:?}", rest);
    // println!("",);
    for _ in 0..children {
        let (sum, val, data) = memory_maneuver(rest);
        total += sum;
        rest = data;
        references.push(val)
    };

    total += &rest[..metadata as usize].iter().sum();

    if children ==0 {
        (total, *&rest[..metadata as usize].iter().sum::<i32>(), &rest[metadata as usize..])
    } else {
        let values: _ = &rest[..metadata as usize].iter().map(|&meta| {
            if meta != 0 && meta <= references.len() as i32{
                return references[meta as usize - 1]
            };
            0
        }).sum::<i32>();
        (total, *values, &rest[metadata as usize..])
    }

}

fn get_sum(zero_idx: usize, data: &mut Vec<i32>) -> i32 {
    let mut metadata = 0;
    let start = zero_idx;
    let num_of_metadata = data[start+1];
    let end = start + num_of_metadata as usize + 1;

    if start > 0 {
        data[start-2] -= 1;
    }

    metadata += &data[start+2..=end].iter().sum();
    data.drain(start..=end);
    metadata
}

fn find_refence_sum(data: &mut Vec<i32>, starting_idx:usize) -> i32 {
    let num_of_children = data[starting_idx];
    let num_of_metadata = data[starting_idx+1];
    let mut metadata = 0;

    let mut children: Vec<i32>= vec![];
    for _ in 0..num_of_children {
        let zero_idx = find_zero_idx(data);
        if zero_idx - 2 == starting_idx {
            let current_sum = get_sum(zero_idx, data);
            children.push(current_sum);
        } else {
            let current_sum = find_refence_sum(data, starting_idx+ 2);
            children.push(current_sum);
        }
    }

    for i in starting_idx+ 2.. starting_idx + 2 + num_of_metadata as usize {
        let mut current_reference = data[i];
        if current_reference == 0{
            continue
        }
        current_reference -= 1;
        match children.get(current_reference as usize) {
            Some(val) => metadata += val,
            None => continue
        }
    };

    data.drain(starting_idx..=starting_idx+num_of_metadata as usize +1);

    metadata

}

fn find_zero_idx(data: &mut Vec<i32>) -> usize {
    data.iter().position(|num| *num == 0).unwrap()
}



impl Runner for Q8 {
    fn run(&mut self) {
        let one = Instant::now();
        self.part1();
        let d = one.elapsed();
        println!("part1 - {:?}", d);

        let start = Instant::now();
        self.part2();
        let duration = start.elapsed();
        println!("part2 - {:?}", duration);

        let start = Instant::now();
        self.part3();
        let duration = start.elapsed();
        println!("part3 - {:?}", duration);
        
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q8() {
        assert_eq!(1, 1);
    }
}
