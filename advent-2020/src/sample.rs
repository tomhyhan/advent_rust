use advent_2020::Runner;

pub struct Sample {

}

impl Sample {
    pub fn new() -> Self {
        Sample {}
    }

    fn part1(&mut self) {
        println!{"Sample works",};
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Sample {
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
    fn Sample() {
        assert_eq!(1, 1);
    }
}