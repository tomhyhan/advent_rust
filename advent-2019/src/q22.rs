use std::collections::{VecDeque, HashSet};

use advent_2019::{Runner, get_file};

// To deal into new stack
// cut N cards
// deal with increment 3
struct Cards {
    stack: Vec<i64>,
}
//  what if we think of, not the whole card stack, single position
//  of the card we are interested in?
// [0,1,2,3,4,5,6,7]
// [7,6,5,4,3,2,1,0]
// 2 -> 5 
// 8 - 1 - 2 = 5
// 8 - 1 - 5 = 2

// [0,1,2,3,4,5,6,7]
// cut 3
// [,3,4,5,6,7,0,1,2]
// 2 - 3 = -1 % 7 
//  cut -3
// [5,6,7,0,1,2,3,4]

// [0,1,2,3,4,5,6,7,8,9]
// increment 3
// [0,7,4,1,8,5,2,9,6,3]
// 2 * 3 from 0
impl Cards {
    fn new(num_of_cards:i64) -> Self {
        let mut stack = Vec::new();
        for i in 0..num_of_cards {
            stack.push(i)
        }
        Self {stack}
    }

    fn cut(&mut self, mut by: i64) {
        if by < 0 {
            by = self.stack.len() as i64 + by;       
        }
        let (one,two) = self.stack.split_at(by as usize);
        let new_stack = [two,one].concat();
        self.stack = new_stack
    }
    
    fn deal_into_new_stack(&mut self) {
        let mut left = 0;
        let mut right = self.stack.len() -1;
        while left < right {
            let left_val = self.stack[left];
            self.stack[left] = self.stack[right];
            self.stack[right] = left_val;
            left += 1;
            right -=1;
        }
    }

    fn deal_with_increment(&mut self, by: i64) {
        let mut new_stack = vec![-1; self.stack.len()];
        let mut cnt = 0;
        let stack_length = self.stack.len();
        while cnt < stack_length {
            let idx = (cnt * by as usize) % stack_length ;
            if new_stack[idx] != -1 {
                panic!("this spit is supposed to be empty!")
            }
            new_stack[idx] = self.stack[cnt];
            cnt += 1;
        }
        self.stack = new_stack;
    }


}

struct CardsOne {
    stack_length: i64,
    card_pos: i64
}

impl CardsOne {
    fn new(num_of_cards:i64, card_pos:i64) -> Self {
        let stack_length = num_of_cards;
        Self {stack_length, card_pos}
    }

    fn cut_one(&mut self, by: i64) {
        if by < 0 {
            self.card_pos = (self.card_pos + by.abs()) % self.stack_length; 
        } else {
            self.card_pos = if self.card_pos - by >=0 {
                    (self.card_pos - by) % self.stack_length
                } else {
                    self.stack_length + ((self.card_pos - by) % self.stack_length)
                } ;
        }
    }
    
    fn deal_into_new_stack_one(&mut self) {
        self.card_pos = self.stack_length - 1 - self.card_pos;
    }
    
    fn deal_with_increment_one(&mut self, by: i64) {
        self.card_pos = self.card_pos * by % self.stack_length
    }
}

struct Cardslocation {
    stack_length: i64,
    card_at: i64,
    card_pos: i64,
}

impl Cardslocation {
    fn new(num_of_cards:i64, card_at:i64) -> Self {
        let stack_length = num_of_cards;
        Self {stack_length, card_at, card_pos: card_at}
    }

    fn deal_into_new_stack_location(&mut self) {
        self.card_at = self.stack_length - 1 - self.card_at;
    }

    fn cut_location(&mut self, by: i64) {
        let mut new_num = (self.card_at + by) % self.stack_length;
        if new_num < 0 {
            new_num = self.stack_length + new_num;
        }
        self.card_at = new_num
    }
    
    fn deal_with_increment_location(&mut self, by:i64) {
        // 
        let lcd = (self.card_pos + self.stack_length) * by / gcd(self.card_pos + self.stack_length, by);
        let pos_to_find = lcd / by;
        let offset = pos_to_find - self.card_pos;
        let mut new_num = self.card_pos + offset;
        if new_num < 0 {
            new_num = self.stack_length - new_num;
        }
        self.card_at = new_num

    }
}
pub struct Q22 {

}

impl Q22 {
    pub fn new() -> Self {
        Q22 {}
    }

    fn part1(&mut self) {
        let content = get_file("src/input/q22.txt").unwrap();
        let mut cards = Cards::new(10007);
        content.lines().for_each(|line|{
        if line.starts_with("cut") {
            let by = line.split_whitespace().nth(1).unwrap().parse::<i64>().unwrap();
            cards.cut(by)
        } else if line.starts_with("deal with increment") {
            let by = line.split_whitespace().nth(3).unwrap().parse::<i64>().unwrap();
            cards.deal_with_increment(by)
        } else if line.starts_with("deal into new stack") {
            cards.deal_into_new_stack();
        } else {
            panic!("unknown instruction!")
        }
        });
        println!("{:?}", cards.stack.iter().position(|c| *c == 2019).unwrap());
    }

    fn part2(&mut self) {
        // let content = get_file("src/input/q22.txt").unwrap();
        // let mut seen = HashSet::new();
        // for i in 0..1000 {
        //     let new_count = content.repeat(i);
        //     let mut cards = CardsOne::new(119315717514047, 2019);
        //     new_count.lines().for_each(|line|{
        //         if line.starts_with("cut") {
        //             let by = line.split_whitespace().nth(1).unwrap().parse::<i64>().unwrap();
        //             cards.cut_one(by)
        //         } else if line.starts_with("deal with increment") {
        //             let by = line.split_whitespace().nth(3).unwrap().parse::<i64>().unwrap();
        //             cards.deal_with_increment_one(by)
        //         } else if line.starts_with("deal into new stack") {
        //             cards.deal_into_new_stack_one();
        //         } else {
        //             panic!("unknown instruction!")
        //         }
        //     });
        //     // println!("{:?}", cards.card_pos);
        //     if !seen.insert(cards.card_pos) {
        //         println!("{:?}", cards.card_pos);
        //         break
        //     }
        // }
        // cards.cut_one(by)

        // cards.deal_with_increment_one(by)

        // cards.deal_into_new_stack_one();
            // 0 1 2 3 4 5 6 7 8 9: num (pos + 10k / by)
        // 0 1 2 3 4 5 6 7 8 9
        // 0 7 4 1 8 5 2 9 6 3 
        // pos 2 + 2 = 4
        // 4 
        let mut cards =Cards::new(7);
        cards.deal_with_increment(3);
        println!("{:?}", cards.stack);
        cards.deal_with_increment(3);
        println!("{:?}", cards.stack);
        // let mut cards = CardsOne::new(10,4);
        // cards.deal_with_increment_one(3);
        // println!("{:?}", cards.card_pos);
        // let mut cards = CardsOne::new(10,4);
        // cards.deal_with_increment_one(3);
        // println!("{:?}", cards.card_pos);
    }

}
// 10 4 | 
fn gcd(x:i64,y:i64) -> i64 {
    if x == 0  {y} else {gcd(y%x, x)} 
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
    use super::*;

    #[test]
    fn reverse_stack() {
        let mut cards = Cards::new(10);
        cards.deal_into_new_stack();
        assert_eq!(cards.stack, vec![9,8,7,6,5,4,3,2,1,0]);
    }
    
    #[test]
    fn cut_3() {
        let mut cards = Cards::new(10);
        cards.cut(3);
        assert_eq!(cards.stack, vec![3,4,5,6,7,8,9,0,1,2]);
    }
    #[test]
    fn cut_negative_4() {
        let mut cards = Cards::new(10);
        cards.cut(-4);
        assert_eq!(cards.stack, vec![6,7,8,9,0,1,2,3,4,5]);
    }
    #[test]
    fn deal_with_increment_3() {
        let mut cards = Cards::new(10);
        cards.deal_with_increment(3);
        assert_eq!(cards.stack, vec![0,7,4,1,8,5,2,9,6,3]);
    }
    #[test]
    fn gcd_test() {
        assert_eq!(gcd(10,4), 2);
        assert_eq!(gcd(4,10), 2);
    }
    // #[test]
    // fn cut_location_2() {
    //     let mut cards_loc = Cardslocation::new(10,2);
    //     cards_loc.cut_location(2);
    //     assert_eq!(cards_loc.card_at, 4);
    // }
    // #[test]
    // fn cut_location_5() {
    //     let mut cards_loc = Cardslocation::new(10,2);
    //     cards_loc.cut_location(5);
    //     assert_eq!(cards_loc.card_at, 7);
    // }
    // #[test]
    // fn cut_location_neg5() {
    //     let mut cards_loc = Cardslocation::new(10,2);
    //     cards_loc.cut_location(-5);
    //     assert_eq!(cards_loc.card_at, 7);
    // }
    // #[test]
    // fn reverse_pos() {
    //     let mut cards_loc = Cardslocation::new(10,2);
    //     cards_loc.deal_into_new_stack_location();
    //     assert_eq!(cards_loc.card_at, 7);
    // }
    // #[test]
    // fn reverse_pos1() {
    //     let mut cards_loc = Cardslocation::new(10,2);
    //     cards_loc.deal_into_new_stack_location();
    //     cards_loc.deal_into_new_stack_location();
    //     assert_eq!(cards_loc.card_at, 2);
    // }

    // #[test]
    // fn increment_by_3() {
    //     let mut cards_loc = Cardslocation::new(10,2);
    //     cards_loc.deal_with_increment_location(3);
    //     assert_eq!(cards_loc.card_at, 4);
    //     cards_loc.deal_with_increment_location(3);
    //     println!("{:?}", cards_loc.card_at);
    //     assert_eq!(cards_loc.card_at, 1);
    // }
}
