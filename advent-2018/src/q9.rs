use std::{collections::{HashMap, HashSet, VecDeque}, time::Instant};

use advent_2018::{Runner, get_file};

pub struct Q9 {

}

struct Game {
    scores: HashMap<i64,i64>,
    marbles: VecDeque<i64>,
    current: usize,
    num_of_marbles: usize,
    players: usize
}

#[derive(Clone, Copy)]
struct Marble {
    value: usize,
    prev: usize,
    next:usize
}

impl Game {
    fn new() -> Game{
        let contents = get_file("src/input/q9.txt").unwrap();
        let tokens: Vec<_> = contents.split_whitespace().collect();
        let players = tokens[0].parse::<usize>().unwrap();
        let num_of_marbles = tokens[6].parse::<usize>().unwrap();
        let scores = HashMap::new();
        let marbles = VecDeque::from([0]);
        let current = 0;
        Game {current,num_of_marbles,players,marbles,scores}
    }

    fn play(&mut self) {
        let mut marbles: Vec<Marble> = vec![];
        marbles.push(Marble { value: 0, prev: 0, next: 0 });

        for marble in 1..self.num_of_marbles {
            match marble % 23 ==0 {
                true => {
                    let player = marble % self.players;
                    let mut current = self.current;
                    for _ in 0..7 {
                        current = marbles[current].prev
                    }
                    let mar = marbles[current];
                    marbles[mar.next].prev = mar.prev; 
                    marbles[mar.prev].next = mar.next; 
                    *self.scores.entry(player as i64).or_insert(0) += marble as i64 + mar.value as i64;
                    self.current = mar.next
                },
                false => {
                    let prev = marbles[self.current].next;
                    let next = marbles[prev].next;
                    let mar = marbles.len();
                    marbles.push(Marble { value: marble, prev, next  });
                    marbles[next].prev = mar; 
                    marbles[prev].next = mar; 
                    self.current = mar; 
                }
            }
            
        }
        println!("{:?}", self.scores.iter().max_by_key(|score|score.1).unwrap());
        
    }


    fn play1(&mut self) {

        for marble in 1..self.num_of_marbles {
            match marble % 23 ==0 {
                true => {
                    let player = marble % self.players;
                    for _ in 0..7 {
                        let val = self.marbles.pop_back().unwrap();
                        self.marbles.push_front(val);
                    }
                    *self.scores.entry(player as i64).or_insert(0) += self.marbles.pop_front().unwrap() + marble as i64;
                },  
                false => {
                    for _ in 0..2 {
                        let val = self.marbles.pop_front().unwrap();
                        self.marbles.push_back(val);
                    }
                    self.marbles.push_front(marble as i64);
                }
            }
        }
        println!("{:?}", self.scores.iter().max_by_key(|score|score.1).unwrap());
        
    }
    // fn play(&mut self) {
    //     let mut visited = HashSet::new();
    //     let mut found =  false;
    //     for marble in 1..=self.num_of_marbles {
    //         let current_player = marble % (self.players );
    //         match marble % 23 == 0 {
    //             false => {
    //                 self.current = (self.current + 2) % self.marbles.len() as i64;
    //                 match self.current ==0 {
    //                     true => self.current = self.marbles.len() as i64,
    //                     false => ()
    //                 };
    //                 self.marbles.insert(self.current as usize, marble);
    //             },
    //             true => {
    //                 let mut score = 0;
    //                 score += marble;
 
    //                 self.current = if self.current - 7 < 0 {
    //                     self.marbles.len() as i64 + (self.current - 7)
    //                 } else {
    //                     self.current - 7
    //                 };
    //                 score += self.marbles[self.current as usize];
    //                 self.marbles.remove(self.current as usize);
    //                 if found {
    //                     println!("next score");
    //                     println!("{:?}", score);
    //                     break
    //                 }
    //                 if  !visited.insert(score) {
    //                     found = true;
    //                     println!("{:?}", "found");
    //                     println!("{:?}", self.scores);
    //                     println!("{:?}", marble);
    //                 }
    //                 self.current = self.current % self.marbles.len() as i64;
    //                 *self.scores.entry(current_player).or_insert(0) += score; 
    //             }
    //         }
    //     }
    //     println!("{:?}", self.scores.iter().max_by_key(|score|score.1).unwrap());
    // }
}

impl Q9 {
    pub fn new() -> Self {
        Q9 {}
    }

    fn part1(&mut self) {
        // let mut game = Game::new();
        // let one = Instant::now();
        // game.play();
        // let d = one.elapsed();
        // println!("play - {:?}", d);

        let mut game2 = Game::new();

        let one = Instant::now();
        game2.play1();
        let d = one.elapsed();
        println!("play1 - {:?}", d);
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q9 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q9() {
        assert_eq!(1, 1);
    }
}