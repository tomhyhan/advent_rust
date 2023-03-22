//  do this
use std::collections::HashMap;

use advent_2017::{Runner, get_file};

pub struct Q13 {

}

#[derive(Debug, PartialEq, Eq, Clone)]
struct FireWall {
    depth: i32,
    pos: i32,
    direction: bool,
    layer: i32
}

impl FireWall {
    fn move_firewall(&mut self) {
        if self.direction {
            self.pos += 1;
            if self.pos == self.depth - 1 {
                self.direction = false;
            }
        } else {
            self.pos -= 1;
            if self.pos == 0 {
                self.direction = true;
            }
        }
    }
}

#[derive(Clone)]
struct Scanners {
    layers: HashMap<i32, Option<FireWall>>,
    layer_length: i32,
}

impl Scanners {
    fn new() -> Self {
        let content = get_file("q13.txt").unwrap();
        let mut layers = HashMap::new();
        let mut layer_length = 0;

        content.lines().for_each(|line| {
            let (layer, depth) = line.split_once(": ").unwrap();
            let layer = layer.parse::<i32>().unwrap();
            let depth = depth.parse::<i32>().unwrap();
            layers.insert(layer, Some(FireWall {depth, direction:true, pos:0, layer}));
            layer_length = layer_length.max(layer);
        });

        Scanners { layers, layer_length }
    }

    fn move_scanner(&mut self) {
        for idx in 0..=self.layer_length {
            let firewall = self.layers.entry(idx).or_insert(None);
            if firewall.is_none() {
                continue
            }
            firewall.as_mut().unwrap().move_firewall();
        }
    }
}


impl Q13 {
    pub fn new() -> Self {
        Q13 {}
    }

    fn part1(&mut self) {
        let mut scanners = Scanners::new();

        let mut severity = 0 ; 
        for you in 0..=scanners.layer_length {
            let current_firewall = scanners.layers.entry(you).or_insert(None);
            if current_firewall.is_none() {
                scanners.move_scanner();
                continue
            }
            let pos = current_firewall.as_ref().unwrap().pos;
            let depth = current_firewall.as_ref().unwrap().depth;
            if pos == 0 {
                println!("{:?}", you);
                severity += you * depth;
            }
            scanners.move_scanner();
            println!("{:?}",scanners.layers.entry(you).or_insert(None));
        }
        println!("{:?}", severity);   
    }

    // method 3
    // find when camera position is 0 and nothing else ;(


    // method 2
    // 0 1 2 3 4 5 6   depth: 4 do: 4 + 2
    //         ->  >= depth then, 6 - 2 
    // 0 1 2 3 2 1 0 
    // 0 1 2 1 0 
    fn part2(&mut self) {
        let contents = get_file("q13.txt").unwrap();
        let firewalls: Vec<FireWall> = contents.lines().map(|line|{
            let (layer, depth) = line.split_once(": ").unwrap();
            let layer = layer.parse::<i32>().unwrap();
            let depth = depth.parse::<i32>().unwrap();
            FireWall {depth, direction:true, pos:0, layer}
        }).collect();
        let mut iterations = HashMap::new();

        let mut step = 0;
        for n in 2..=20 {
            iterations.insert(n, (n+step, step));
            step += 1;
        }

        let mut time = 0;
        loop {
            let mut severity = false;
            let mut temp = 0;
            for firewall in firewalls.iter() {
                // let depth = firewall.depth; 
                let iteration = iterations.get(&firewall.depth).unwrap();
                let pos = (firewall.layer + time) % iteration.0;
                let current_camera = if pos >= iteration.0 - iteration.1 { iteration.0 - (pos)} else {pos};
                if current_camera == 0{
                    severity = true;
                    temp += firewall.depth * firewall.layer;
                }
            }    
            if !severity {
                println!("{time}");
                break;
            }
            println!("{temp}");

            time += 1;
            break
        }
    }

    // method 1
    // fn part2(&mut self) {
    //     let mut delay = 0;

    //     let start = Scanners::new();
    //     let mut previous = start.layers;
    //     let mut delays = HashMap::new();
    //     loop {
    //         let mut scanners = Scanners::new();

    //         if delay == 0 {
    //             delays.insert(0, scanners.layers.clone());
    //         } else {
    //             scanners.layers = delays.get(&(delay-1)).unwrap().clone();
    //             scanners.move_scanner();
    //             delays.insert(delay, scanners.layers.clone());
    //         }


    //         let mut severity = false ; 
    //         'inner: for you in 0..=scanners.layer_length {
    //             let current_firewall = scanners.layers.entry(you).or_insert(None);
    //             if current_firewall.is_none() {
    //                 scanners.move_scanner();
    //                 continue
    //             }
    //             let pos = current_firewall.as_ref().unwrap().pos;

    //             if pos == 0 {
    //                 severity = true;
    //                 break 'inner;
    //             }
    //             scanners.move_scanner();
    //         }
    //         // println!("{delay}");
    //         if !severity {
    //             println!("{:?}", delay);
    //             break;
    //         }
    //         delay += 1;
    //     }        
          
    // }
}

impl Runner for Q13 {
    fn run(&mut self) {
        // self.part1();
        self.part2()
    }
}


#[cfg(test)]
mod test{
    use super::*;
    
    #[test]
    fn firewall_moves_back() {
        let mut firewall = FireWall {depth: 3, direction: true, pos: 0, layer: 5};
        firewall.move_firewall();
        assert_eq!(firewall.pos, 1);
        firewall.move_firewall();
        assert_eq!(firewall.pos, 2);
        firewall.move_firewall();
        assert_eq!(firewall.pos, 1);
        firewall.move_firewall();
        assert_eq!(firewall.pos, 0);
        firewall.move_firewall();
        assert_eq!(firewall.pos, 1);
        firewall.move_firewall();
        assert_eq!(firewall.pos, 2);
    }
}


// 0: 3
// 1: 2
// 4: 4
// 6: 4
