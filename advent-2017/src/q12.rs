use std::collections::{HashSet, HashMap};

use advent_2017::{Runner, get_file};

pub struct Q12 {
    // method 1
    // pipes: HashMap<i32, Pipe>,
    // method 2
    pipes: HashMap<i32, Vec<i32>>
}

#[derive(Debug)]
struct Pipe {
    village: i32,
    connect: Vec<i32>
}


impl Q12 {
    pub fn new() -> Self {
        let content = get_file("q12.txt").unwrap();
        // method 1
        // let mut pipes = HashMap::new();
        // content.lines().for_each(|line| parse(line, &mut pipes));
        // method 2
        let mut pipes = HashMap::new();
        content.lines().for_each(|line| parse(line, &mut pipes));

        Q12 {
            pipes
        }
    }

    // method 2 using iteration
    fn part1(&mut self) {
        let mut stack = vec![0];
        let mut visited = HashSet::new();

        while let Some(val) = stack.pop() {
            if visited.insert(val) {
                let connect = self.pipes.get(&val).unwrap();
                stack.extend(connect.iter().filter(|node|!visited.contains(node)))
            }
        }
        println!("{:?}", visited.len()); 
    }


    fn part2(&mut self) {
        let mut unvisited = HashSet::new();
        for key in self.pipes.keys() {
            unvisited.insert(*key);
        }
        
        let mut groups = 0;
        while !unvisited.is_empty() {
            groups += 1;
            let start = *unvisited.iter().next().unwrap();
            unvisited.remove(&start);

            let mut stack = vec![start];
            let mut visited = HashSet::new();
    
            while let Some(val) = stack.pop() {
                unvisited.remove(&val);
                if visited.insert(val) {
                    let connect = self.pipes.get(&val).unwrap();
                    stack.extend(connect.iter().filter(|node|!visited.contains(node)))
                }
            }
        }
        println!("{:?}", groups);
    }

    // method 1 using recursion
    // fn part1(&mut self){
    //     // println!("{:?}", self.pipes);

    //     let mut groups: HashSet<Vec<i32>> = HashSet::new();
    //     'outer: for vil in self.pipes.keys() {
    //         let mut visited = HashSet::new();
    //         for g in groups.iter() {
    //             if g.contains(vil) {
    //                 continue 'outer;
    //             }
    //         }
    //         visited.insert(*vil);
    //         for pipe in self.pipes.values() {
    //             let mut cycle = HashSet::new();
    //             let did_find = find_pipe(&pipe, &mut visited, &self.pipes, &mut cycle, *vil);
    //             if did_find {
    //                 visited.insert(pipe.village);
    //             }
    //         };
    //         let mut s_visited = visited.into_iter().collect::<Vec<i32>>();
    //         s_visited.sort();
    //         groups.insert(s_visited);
    //         // println!("{:?}", groups);
    //     }
    //     // println!("{:?}", visited.len());
    //     // println!("{:?}", visited);
    //     println!("{:?}", groups);
    //     println!("{:?}", groups.len());
    // }
}

fn find_pipe(pipe: &Pipe, visited: &mut HashSet<i32>, pipes: &HashMap<i32, Pipe>, cycle: &mut HashSet<i32>, find: i32) -> bool {
    cycle.insert(pipe.village);
    if visited.contains(&pipe.village) {
        return true
    }
    for con in pipe.connect.iter() {
        if cycle.contains(con) {
            continue;
        }
        if *con == find {
            return true
        }
        let did_find = find_pipe(pipes.get(con).unwrap(), visited, pipes, cycle, find);
        if did_find {
            return true
        }
    }
    return false
}


fn parse(line: &str, pipes: &mut HashMap<i32, Vec<i32>>) {
    let (village, rest) = line.split_once(" <-> ").unwrap();
    let connect= rest.split(", ").map(|s| s.parse::<i32>().unwrap()).collect();
    let village = village.parse::<i32>().unwrap();

    // let pipe = Pipe {
    //     connect, village
    // };

    // pipes.insert(village, pipe);
    
    pipes.insert(village, connect);

}

impl Runner for Q12 {
    fn run(&mut self) {
        self.part1();
        self.part2();
    }
}
