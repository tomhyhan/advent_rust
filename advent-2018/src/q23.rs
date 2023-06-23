//  x.x heap doesn't work x.x
//  again
use std::{collections::{HashMap, BinaryHeap, HashSet}, cmp::Ordering};
use regex::Regex;
use lazy_static::lazy_static; 
use itertools::Itertools;

use advent_2018::{Runner, get_file};

pub struct Q23 {

}

struct Teleportation {
    nanobots: HashMap<(i32,i32,i32), i32>,
    max_nanobot: (i32,i32,i32),
    max_signal: i32
}

impl Teleportation {
    fn new() -> Self {
        let content = get_file("src/input/q23.txt").unwrap();
        let mut nanobots = HashMap::new();
        let mut max_signal = i32::MIN;
        let mut max_nanobot = (0,0,0);
        content.lines().for_each(|line| {
            lazy_static!(
                static ref RE: Regex = Regex::new(r"[-]?\d+").unwrap();
            );
            let nums: Vec<_>= RE.find_iter(line).collect();
            let (x,y,z,r) = (nums[0].as_str().parse::<i32>().unwrap(),nums[1].as_str().parse::<i32>().unwrap(),nums[2].as_str().parse::<i32>().unwrap(),nums[3].as_str().parse::<i32>().unwrap());
            nanobots.insert((x,y,z), r);
            if r > max_signal {
                max_signal = max_signal.max(r);
                max_nanobot = (x,y,z)
            } 
        });
        Self {nanobots, max_nanobot, max_signal}
    }

    // pos=<10,12,12>, r=2  => cover from 8,12,12 | 10,10,12 | 10,12,10 to 
    // pos=<12,14,12>, r=2  
    // pos=<16,12,12>, r=4
    // pos=<14,14,14>, r=6
    // pos=<50,50,50>, r=200
    // pos=<10,10,10>, r=5

    fn calculate_distance(&self, bot1: [i32;3], bot2: [i32;3]) -> i32 {
        (bot1[0] - bot2[0]).abs() + (bot1[1] - bot2[1]).abs() + (bot1[2] - bot2[2]).abs()
    }

    fn find_nanobots_within(&self) {
        println!("{:?}", self.max_nanobot);
        println!("{:?}", self.max_signal);
        let nanobots_within = self.nanobots.iter().filter(|((x,y,z), _)|{
            let (max_x, max_y, max_z) = self.max_nanobot;
            let r = (max_x - x).abs() + (max_y - y).abs() + (max_z - z).abs(); 
            r <= self.max_signal
        }).count();
        println!("{:?}", nanobots_within);

        println!("done");
    }

    fn count_num_of_bots(&self, new_boxes: [[i32;3];2]) -> usize {
        self.nanobots.iter().filter(|((x,y,z),&r)| {
            let mut d = 0;
            for i in 0..3 {
                let bot_coord;
                match i {
                    0 => bot_coord = x,
                    1 => bot_coord = y,
                    2 => bot_coord = z,
                    _ => panic!("not in range")
                };
                let (lower_bound, high_bound) = (new_boxes[0][i], new_boxes[1][i]-1);
                d += (bot_coord - lower_bound).abs() + (bot_coord - high_bound).abs();
                d -= high_bound - lower_bound; 
            };
            // println!("d - {d}");
            // println!("bot- {:?}",(x,y,z,r));
            // println!("box - {new_boxes:?}");
            d /= 2;
            d <= r
        }).count()
    }
}

#[derive(Debug, Eq, PartialEq)]
struct CustomTuple(usize, i32, i32, [[i32; 3]; 2]);


impl Ord for CustomTuple {
    fn cmp(&self, other: &Self) -> Ordering {
        // self.0.cmp(&other.0).then(self.1.cmp(&other.1)).then({
        //     if self.2 < other.2 {
        //         return Ordering::Greater
        //     }
        //     Ordering::Less
        // })
        if self.0 > other.0 {
            Ordering::Greater
        } else if self.1 > other.1 {
            Ordering::Greater
        } else if self.2 < other.2 {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    }
}

impl PartialOrd for CustomTuple {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Q23 {
    pub fn new() -> Self {
        Q23 {}
    }

    fn part1(&mut self) {
        let teleportation = Teleportation::new();
        teleportation.find_nanobots_within();
    }

    fn part2(&mut self) {
        let teleportation = Teleportation::new();
        let mut boxsize = 1;
        let max_size = teleportation.nanobots.iter().map(|((x,y,z),r)| {
            (x.abs() + r).max(y.abs() + r).max(z.abs() + r)
        }).max().unwrap();
        println!("{:?}", max_size);
        while boxsize <= max_size {
            boxsize *= 2;
        }
        let init_box = [[-boxsize,-boxsize,-boxsize],[boxsize,boxsize,boxsize]];

        let mut queue = HashSet::from([(teleportation.nanobots.len(), boxsize*2, i32::MAX, init_box)]);
        let elements = vec![0, 1];
        let combination_length = 3;
    
        let combinations: Vec<Vec<_>> = (0..combination_length)
            .map(|_| elements.iter().cloned())
            .multi_cartesian_product()
            .collect();
        
        
        println!("{:?}", combinations);        
        loop  {
            let current = queue.iter().max_by(|x,y| {
                x.0.cmp(&y.0).then(x.1.cmp(&y.1).then((-x.2).cmp(&(-y.2))))
            }).unwrap().clone();
            if !queue.remove(&current) {
                panic!("suppose to exist!")
            }
            // println!("{:?}", current);
            let (num_of_bots, mut to_reach, d_to_goal, boxes) = (current.0,current.1,current.2,current.3);
            if to_reach == 1 {
                println!("{boxes:?} {num_of_bots} {d_to_goal}");
                break
            }
            to_reach /= 2;
            for neighbor in combinations.iter() {
                let box0: [i32; 3] = (0..3).into_iter().map(|i| {
                    boxes[0][i] + to_reach * neighbor[i]
                }).collect::<Vec<_>>().try_into().unwrap();
                let box1: [i32; 3] = (0..3).into_iter().map(|i| {
                    box0[i] + to_reach
                }).collect::<Vec<_>>().try_into().unwrap();
                let new_boxes = [box0, box1];
                let new_num_of_bots= teleportation.count_num_of_bots(new_boxes);
                queue.insert((new_num_of_bots, to_reach, teleportation.calculate_distance(box0, [0,0,0]), new_boxes));
                // println!("info - {:?} {:?} {:?} {:?}", new_num_of_bots, to_reach, teleportation.calculate_distance(box0, [0,0,0]), new_boxes)
            }
        }
       
        }


}




impl Runner for Q23 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q23() {
        assert_eq!(1, 1);
    }
}

// let mut heap = BinaryHeap::from([CustomTuple(teleportation.nanobots.len(), boxsize*2, i32::MAX, init_box)]);
// let elements = vec![0, 1];
// let combination_length = 3;

// let combinations: Vec<Vec<_>> = (0..combination_length)
//     .map(|_| elements.iter().cloned())
//     .multi_cartesian_product()
//     .collect();


// println!("{:?}", combinations);        
// while let Some(current) = heap.pop() {
//     println!("{:?}", current);
//     let (num_of_bots, mut to_reach, d_to_goal, boxes) = (current.0,current.1,current.2,current.3);
//     if to_reach == 1 {
//         println!("{boxes:?} {num_of_bots}");
//         break
//     }
//     to_reach /= 2;
//     for neighbor in combinations.iter() {
//         let box0: [i32; 3] = (0..3).into_iter().map(|i| {
//             boxes[0][i] + to_reach * neighbor[i]
//         }).collect::<Vec<_>>().try_into().unwrap();
//         let box1: [i32; 3] = (0..3).into_iter().map(|i| {
//             box0[i] + to_reach
//         }).collect::<Vec<_>>().try_into().unwrap();
//         let new_boxes = [box0, box1];
//         let new_num_of_bots= teleportation.count_num_of_bots(new_boxes);
//         heap.push(CustomTuple(new_num_of_bots, to_reach, teleportation.calculate_distance(box0, [0,0,0]), new_boxes));
//         // println!("info - {:?} {:?} {:?} {:?}", new_num_of_bots, to_reach, teleportation.calculate_distance(box0, [0,0,0]), new_boxes)
//     }
// }