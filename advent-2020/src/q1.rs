use advent_2020::Runner;

pub struct Q1 {

}

impl Q1 {
    pub fn new() -> Self {
        Q1 {}
    }

    fn part1(&mut self) {
        println!{"Q1 works",};
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q1 {
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
    fn Q1() {
        assert_eq!(1, 1);
    }
}