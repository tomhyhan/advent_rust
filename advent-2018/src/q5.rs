use advent_2018::{Runner, get_file};

pub struct Q5 {
    polymer: Vec<char>
}

impl Q5 {
    pub fn new() -> Self {
        let content = get_file("src/input/q5.txt").unwrap();
        let polymer: Vec<char> = content.chars().collect();
        Q5 {
            polymer
        }
    }

    fn part1(&mut self) -> &Vec<char> {
        loop {
            let new_polymer = generate_new_polymer(&self.polymer);
            if self.polymer == new_polymer {
                return &self.polymer
            }
            self.polymer = new_polymer
        }    
    }

    fn part2(&mut self) {
        let units = "abcdefghijklmnopqrstuvwxyz".to_string();
        let mut smallest_length = usize::MAX;
        for alpha in units.chars() {
            let polymer = self.polymer.clone();
            let mut distance_polymer = generate_distance_new_polymer(&polymer, (alpha, alpha.to_ascii_uppercase()));
            loop {
                let new_polymer = generate_new_polymer(&distance_polymer);
                if distance_polymer == new_polymer {
                    break
                }
                distance_polymer = new_polymer
            }   
            smallest_length = smallest_length.min(distance_polymer.len());
        }
        println!("{:?}", smallest_length)
    }

}

fn generate_new_polymer(polymer: &Vec<char>) -> Vec<char> {
    let mut new_polymer = vec![];
    for char in polymer.iter() {
        if !new_polymer.is_empty() && (*new_polymer.last().unwrap() as u8 as i32 - *char as u8 as i32).abs() == 32 {
            new_polymer.pop();
        } else {
            new_polymer.push(*char);
        }
    }
    new_polymer
}
//     polymer.chars().filter(|&c| c.to_ascii_lowercase() != unit.to_ascii_lowercase()).collect()
//  dabAcCaCBAcCcaDA
fn generate_distance_new_polymer(polymer: &Vec<char>, (x,y): (char,char)) -> Vec<char> {
    let new_polymer : Vec<char> =  polymer.iter().cloned().filter(|&c| c.to_ascii_lowercase() != x.to_ascii_lowercase()).collect();

    // method 1 => assumes that you only need to delete a/A A/a pair
    // let mut new_polyer = vec![];
    // for char in polymer.iter() {
    //     if !new_polymer.is_empty() && (*char == x || *char == y) {
    //         let mut did_find = false;
    //         for (idx, sub_char) in new_polymer.iter().enumerate().rev() {
    //             match is_found(*char, *sub_char) {
    //                 true => {
    //                     new_polymer.remove(idx);
    //                     did_find = true;
    //                     break;
    //                 },
    //                 false => ()
    //             }
    //         }
    //         if !did_find {
    //             new_polymer.push(*char);
    //         }
    //     } else {
    //         new_polymer.push(*char);
    //     }
    // }
    new_polymer
}

fn is_found(char: char, char2: char) -> bool {
    (char as u8 as i32 - char2 as u8 as i32).abs() == 32
}

impl Runner for Q5 {
    fn run(&mut self) {
        // let result = self.part1();
        // println!("{}", result.len())
        self.part2()
    }
}


#[cfg(test)]
mod test{
    use super::generate_distance_new_polymer;

    #[test]
    fn distance_polymer() {
        let str =  "dabAcCaCBAcCcaDAa".to_string();
        let polymer = str.chars().collect::<Vec<char>>();
        let result = generate_distance_new_polymer(&polymer, ('a', 'A'));
        assert_eq!(result, "dbcCCBcCcD".chars().collect::<Vec<char>>() )
    }
}
