use std::collections::{HashMap, HashSet};

use advent_2017::{Runner, get_file};

pub struct Q19 {

}

struct Network {
    map: HashMap<(i32,i32),char>,
    visited: HashSet<(i32,i32)>
}

impl Network {
    fn new() -> Self {
        let content = get_file("q19.txt").unwrap();
        let mut map = HashMap::new();
        let list: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();
        for row in 0..list.len() {
            for col in 0..list[0].len() {
                map.insert((row as i32,col as i32), list[row][col]);
            }
        };
        let visited = HashSet::new();
        Network { map, visited }
    }

    fn start_pos(&self) -> (&(i32, i32), &char){
        self.map.iter().find(|(k,&v)| k.0 == 0 && v == '|').unwrap()
    }

    fn send_packet(&mut self, mut packet: Packet) {
        'outer: loop {
            let (row, col) = packet.position;
            self.visited.insert((row,col));

            match packet.direction {
                Direction::Up => {
                    packet.position.0 -= 1;
                },
                Direction::Down => {
                    packet.position.0 += 1;
                },
                Direction::Left =>{
                    packet.position.1 -= 1;
                },
                Direction::Right => {
                    packet.position.1 += 1;
                }
            }

            packet.steps += 1;
            let current_path = *self.map.get(&packet.position).unwrap();
            if current_path == '+'{
                for dir in [(1,0,Direction::Down),(-1,0,Direction::Up),(0,1,Direction::Right),(0,-1,Direction::Left)] {
                    let next_row = packet.position.0 + dir.0; 
                    let next_col = packet.position.1 + dir.1;
                    if self.visited.contains(&(next_row,next_col)) {
                        continue
                    }
                    if *self.map.get(&(next_row,next_col)).unwrap() != ' ' {
                        packet.direction = dir.2;
                        continue 'outer; 
                    }
                }
                break
            } else if current_path.is_alphabetic() {
                packet.letters.push(current_path);
            } else if current_path == ' ' {
                break
            }
        }
        //  17424 - x
        println!("{:?}", packet.letters.iter().collect::<String>());
        println!("{:?}", packet.steps)
    }
}
struct Packet {
    position: (i32,i32),
    letters: Vec<char>,
    direction: Direction,
    steps: i32,
}

enum Direction {
    Up,
    Down,
    Left,
    Right
}

impl Packet {
    fn new(position: (i32,i32)) -> Self {
        let letters = Vec::new();
        let direction = Direction::Down;
        let steps = 0;
        Packet { position, letters, direction, steps }
    }
}

impl Q19 {
    pub fn new() -> Self {
        Q19 {}
    }

    fn part1(&mut self) {
        let mut network = Network::new();
        let mut packet = Packet::new(network.start_pos().0.clone());
        network.send_packet(packet)
    }
}

impl Runner for Q19 {
    fn run(&mut self) {
        self.part1();
    }
}