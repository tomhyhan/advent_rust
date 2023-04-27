use advent_2018::Runner;

pub struct Q14 {

}

struct Chocolate {
    recipes: Vec<i32>,
    elfs: [usize;2],
    after: usize
}

impl Chocolate {
    fn new(after: usize) -> Self {
        let recipes = vec![3,7];
        let elfs = [0,1];
        Self {after,elfs,recipes}
    }

    fn cook(&mut self) -> String{
        let input = &[5,0,9,6,7,1];
       loop {
            let mut score = self.recipes[self.elfs[0]] + self.recipes[self.elfs[1]];

            if score >= 10 {
                self.recipes.push(score / 10);
                score %= 10
            }
            self.recipes.push(score);
            // break;
            for elf in 0..self.elfs.len() {
                self.elfs[elf] = (self.elfs[elf] + self.recipes[self.elfs[elf]] as usize + 1)  % self.recipes.len();
            }
            if self.recipes.len() > input.len() {
                if &self.recipes[self.recipes.len()-input.len()..self.recipes.len()] == input {
                    println!("r1 - {:?}", self.recipes.len()-input.len());
                    break
                }
                if &self.recipes[self.recipes.len()-1-input.len()..self.recipes.len()-1] == input {
                    println!("r2 - {:?}", self.recipes.len()-1-input.len());
                    break
                }

            }
        }

        println!("{:?}",self.recipes.len());
        println!("{:?}",self.recipes.len()-input.len());
        // println!("{:?}",self.recipes);

        self.recipes[self.after..self.after+5].iter().map(|num|num.to_string()).collect::<Vec<String>>().join("")
    }
}
//  3  7  1  0 [1] 0  1  2 (4) 5  1  5  8  9  1  6  7  7  9  2 
impl Q14 {
    pub fn new() -> Self {
        Q14 {}
    }

    fn part1(&mut self) {
        let mut chocolate = Chocolate::new(18);
        let r= chocolate.cook();
        println!("{:?}", r)
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q14 {
    fn run(&mut self) {
        self.part1()
    }
}


#[cfg(test)]
mod test{
    use super::*;
    #[test]
    fn Q14() {
        let mut cho = Chocolate::new(9);
        let r = cho.cook();
        assert_eq!(r, "5158916779".to_string())
    }
    #[test]
    fn Q141() {
        let mut cho = Chocolate::new(5);
        let r = cho.cook();
        assert_eq!(r, "0124515891".to_string())
    }
    #[test]
    fn Q142() {
        let mut cho = Chocolate::new(18);
        let r = cho.cook();
        assert_eq!(r, "9251071085".to_string())
    }
    #[test]
    fn Q143() {
        let mut cho = Chocolate::new(2018);
        let r = cho.cook();
        assert_eq!(r, "5941429882".to_string())
    }
}