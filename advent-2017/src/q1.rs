use advent_2017::{Runner, get_file};

pub struct Q1 {
    captcha: Vec<char>
}

impl Q1 {
    pub fn new() -> Self {
        let content = get_file("q1.txt").unwrap();
        let captcha = content.chars().collect();
        Q1 {captcha}
    }

    fn part1(&mut self) -> i32{
        let sum : i32 = self.captcha.iter().cloned().enumerate().filter(|(idx, c)| {
            let next = (idx + 1) % self.captcha.len();
            *c == self.captcha[next]
        }).map(|c| c.1.to_string().parse::<i32>().unwrap()).sum();    
        println!("{sum}");    
        return sum
    }

    fn part2(&mut self) -> i32{
        let step = self.captcha.len() / 2;
        let sum : i32 = self.captcha.iter().cloned().enumerate().filter(|(idx, c)| {
            let next = (idx + step) % self.captcha.len();
            *c == self.captcha[next]
        }).map(|c| c.1.to_string().parse::<i32>().unwrap()).sum();    
        println!("{sum}");    
        return sum
    }
}

impl Runner for Q1 {
    fn run(&mut self) {
        self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{

    use super::*;

    
    // #[test]
    // fn test_part1(){
    //     let mut q1 = Q1::new();
    //     assert_eq!(q1.part1(), 4)
    // }

    #[test]
    fn test_part2(){
        let mut q1 = Q1::new();
        assert_eq!(q1.part2(), 12)
    }}