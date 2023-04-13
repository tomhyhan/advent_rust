use std::collections::HashMap;

use advent_2018::{Runner, get_file};

pub struct Q3 {

}

struct Fabric {
    prototype: Vec<Vec<usize>>,
    claims: HashMap<usize,Claim>
}


impl Fabric {
    fn new() -> Self {
        let mut claims = HashMap::new();
        let content = get_file("src/input/q3.txt").unwrap();
        let prototype = vec![vec![0; 1000]; 1000];
        content.lines().for_each(|line| parse(line, &mut claims));
        Fabric { prototype, claims }
    }

    fn count_overlapping(&mut self) -> i32 {

        let mut count = 0;
        for (_, claim) in self.claims.iter() {
            for row in claim.top..claim.top+claim.height {
                for col in claim.left..claim.left+claim.width {
                    self.prototype[row][col] += 1;
                    if self.prototype[row][col] == 2 {
                        count += 1
                    }
                }
            }
        }

        count
    }

    fn non_overlapping(&mut self) -> usize{
        for (_, claim) in self.claims.iter() {
            for row in claim.top..claim.top+claim.height {
                for col in claim.left..claim.left+claim.width {
                    self.prototype[row][col] += 1;
                }
            }
        };

        for (id, claim) in self.claims.iter() {
            let mut found = true;
            'outer: for row in claim.top..claim.top+claim.height {
                for col in claim.left..claim.left+claim.width {
                    if self.prototype[row][col] != 1 {
                        found = false;
                        break 'outer;
                    } 
                }
            }
            if found {
                println!("{}", id);
                break
            }
        };
        0
    }
}

fn parse(line:&str, claims: &mut HashMap<usize, Claim>) {
    let tokens: Vec<_>= line.split_whitespace().collect();
    let id = tokens[0].trim_matches(|p| p == '#').parse::<usize>().unwrap();
    let (left, top) = tokens[2].split_once(',').unwrap();
    let (width, height) =  tokens[3].split_once('x').unwrap();
    let top = top.trim_matches(':');
    let claim = Claim {
        left: left.parse::<usize>().unwrap(), 
        top: top.parse::<usize>().unwrap(), 
        width: width.parse::<usize>().unwrap(), 
        height: height.parse::<usize>().unwrap()
    };
    claims.insert(id, claim);

}

#[derive(Debug)]
struct Claim {
    left : usize,
    top: usize,
    width: usize,
    height: usize
}


impl Q3 {
    pub fn new() -> Self {
        Q3 {}
    }

    fn part1(&mut self) {
        let mut fabric = Fabric::new();
        let count = fabric.count_overlapping();
        println!("{:?}", count);
    }

    fn part2(&mut self) {
        let mut fabric = Fabric::new();
        let count = fabric.non_overlapping();
    }

}

impl Runner for Q3 {
    fn run(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q3() {
        assert_eq!(1, 1);
    }
}