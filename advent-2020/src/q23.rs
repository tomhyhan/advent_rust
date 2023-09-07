use std::ops::Index;

use advent_2020::Runner;

struct CrabCup {
    cups: Vec<i32>
}

impl CrabCup {
    fn new(cups: Vec<i32>) -> Self {
        Self {cups}
    }

    fn play(&mut self, moves: usize) {
        let lowest = *self.cups.iter().min().unwrap();
        let highest = *self.cups.iter().max().unwrap();
        let cup_length = self.cups.len();
        
        let mut current = 0;
        for _ in 0..moves {
            let mut picks = vec![];
            let mut next = (current + 1) % cup_length; 

            let mut des = self.cups[current] - 1;
            let current_val = self.cups[current];
            for _ in 1..4 {
                picks.push(self.cups.remove(next));
                next = next % self.cups.len();
            }
            while des < lowest || picks.contains(&des) {
                des -= 1;
                if des < lowest {
                    des = highest
                }
            }
            let insert_idx = self.cups.iter().position(|x| *x == des).unwrap();
            for _ in 1..4 {
                self.cups.insert(insert_idx+1, picks.pop().unwrap());
            }
            current = (self.cups.iter().position(|x| *x == current_val).unwrap() + 1) % cup_length;
        } 
        println!("{:?}", self.cups);

    }
}

pub struct Q23 {

}

impl Q23 {
    pub fn new() -> Self {
        Q23 {}
    }

    fn part1(&mut self) {
        let mut crabcup = CrabCup::new("784235916".chars().map(|c| c.to_digit(10).unwrap() as i32).collect());
        crabcup.play(100);
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q23 {
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
    fn Q23() {
        assert_eq!(1, 1);
    }
}