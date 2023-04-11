use advent_2018::Runner;

pub struct Sample {

}

impl Sample {
    pub fn new() -> Self {
        Sample {}
    }

    fn part1(&mut self) {

    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Sample {
    fn run(&mut self) {
        println!("Sample");
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Sample() {
        assert_eq!(1, 1);
    }
}