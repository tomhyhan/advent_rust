use std::collections::{VecDeque, HashSet};

use advent_2020::{Runner, get_file};

struct Combat {
    crab: VecDeque<i32>,
    player: VecDeque<i32>
}

impl Combat {
    fn new() -> Self {
        let content = get_file("src/input/q22.txt").unwrap();
        let (player1, player2) = content.split_once("\r\n\r\n").unwrap();
        let crab = player1.lines().skip(1).map(|num| num.parse::<i32>().unwrap()).collect();
        let player = player2.lines().skip(1).map(|num| num.parse::<i32>().unwrap()).collect();
        Self {crab,player}
    }
    // 29813  XX
    fn recursive_play(&mut self, mut player: VecDeque<i32>, mut crab: VecDeque<i32>) -> (VecDeque<i32>, bool){
        let mut player_inf = HashSet::new();
        let mut crab_inf = HashSet::new();

        while !player.is_empty() && !crab.is_empty() {
            if !player_inf.insert(player.clone()) || !crab_inf.insert(crab.clone()) {
                return (crab,false)
            }
            let crab_top = crab.pop_front().unwrap();
            let player_top = player.pop_front().unwrap();
            let current_winner;
            if crab_top <= crab.len() as i32 && player_top <= player.len() as i32 {
                let new_player = player.make_contiguous().iter().take(player_top as usize).cloned().collect();
                let new_crab = crab.make_contiguous().iter().take(crab_top as usize).cloned().collect();
                let (_, winner) = self.recursive_play(new_player,new_crab);
                current_winner = winner
            } else {
                current_winner = if player_top > crab_top {true} else {false}
            }
            if current_winner {
                player.push_back(player_top);
                player.push_back(crab_top);
            } else {
                crab.push_back(crab_top);
                crab.push_back(player_top);
            }
        } 

        if player.is_empty() {
            (crab, false)
        } else {
            (player, true)
        }
    }

    fn play(&mut self) {
        let player = self.player.clone();
        let crab = self.crab.clone();
        let (cards, _) = self.recursive_play(player, crab);
        self.calc_total_score(cards); 
    }

    fn calc_total_score(&self, cards: VecDeque<i32>) {
        let mut factor = cards.len() as i32;
        let mut score = 0;
        for card in cards.iter() {
            score += *card * factor;
            factor -= 1
        }
        println!("score - {:?}", score)
    }
}

pub struct Q22 {

}

impl Q22 {
    pub fn new() -> Self {
        Q22 {}
    }

    fn part1(&mut self) {
        let mut combat = Combat::new();
        combat.play()
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q22 {
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
    fn Q22() {
        assert_eq!(1, 1);
    }
}