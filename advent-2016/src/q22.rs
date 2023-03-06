use std::{collections::{HashMap, HashSet}, vec};
use lazy_static::lazy_static;
use regex::Regex;
use crate::{Runner, common::my_modules::get_file};
use std::cmp;

//  come back

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Node {
    size: i32,
    used: i32,
    avail: i32,
    usep: i32
}

pub struct Q22 {
    nodes: HashMap<(i32,i32), Node>,
    x_border: i32,
    y_border: i32,
}

impl Q22 {
    pub fn new() ->  Self {
        let mut nodes = HashMap::new();
        
        let content = get_file("q22.txt").unwrap();

        let mut lines = content.lines();
        lines.next();
        lines.next();
        let mut x_border = 0;   
        let mut y_border = 0;

        lines.for_each(|line| parse(line, &mut nodes, &mut x_border, &mut y_border));
        Q22 {nodes, x_border, y_border}
    }
    
    fn find_emtpy(&self) -> (&(i32, i32), &Node){
        self.nodes.iter().find(|p| p.1.used == 0).unwrap().clone()
    }

    fn find_vaiable_pairs(&self) -> HashSet<((i32, i32), Node)>{
        let mut pairs = HashSet::new();
        let empty_spot = self.find_emtpy();

        self.nodes.clone().into_iter().for_each(|node| {
            if self.is_valid_pair(&node.1, empty_spot.1) {
                pairs.insert((node.0,node.1.clone()));
            };
        });
        pairs
    }

    fn is_valid_pair(&self, x_node: &Node, empty_node: &Node) -> bool {
        if x_node.used == 0 {
            return false
        }
        if empty_node.size >= x_node.used {
            return true
        }
        false
    }
    fn part1(&self) {
        let pair = self.find_vaiable_pairs();
    }

    fn part2(&mut self) {
        let mut grid = vec![vec!['.';self.x_border as usize + 1];self.y_border as usize +1];
        let pair = self.find_vaiable_pairs();
        let empty_node = self.find_emtpy();

        self.nodes.clone().into_iter().for_each(|node| {
            if node.1.used > 100 {
                grid[node.0.1 as usize][node.0.0 as usize] = '#';
            }
        });
        grid[0][0] = 'S';
        grid[0][self.x_border as usize] = 'G';
        grid[empty_node.0.1 as usize][empty_node.0.0 as usize] = '_';


        for g in grid {
            println!("{:?}",g.iter().collect::<String>());
        }
    }

}



fn parse(line: &str, nodes:&mut HashMap<(i32,i32), Node>,  x_border: &mut i32, y_border: &mut i32) {
    lazy_static! {
        static ref RE : Regex = Regex::new(r"\d+").unwrap();
    }

    let data : Vec<i32> = RE.find_iter(line).map(|m| m.as_str().parse::<i32>().unwrap()).collect();
    let node = Node {
        size: data[2],
        used: data[3],
        avail: data[4],
        usep: data[5]
    };

    *x_border = cmp::max(*x_border, data[0]);
    *y_border = cmp::max(*y_border, data[1]);
    nodes.insert((data[0], data[1]), node);
}

impl Runner for Q22 {
    fn run(&mut self) -> () {
        self.part2();
    }
}
