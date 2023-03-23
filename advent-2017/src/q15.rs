use advent_2017::Runner;

// 4000000
pub struct Q15 {

}

struct Generator {
    value: i64,
    factor: i64,
    division : i64,
    multiple :i64
}

impl Generator {
    fn new(value:i64, factor: i64, multiple: i64) -> Self {
        let division = 2147483647;
        Generator { value, factor, division, multiple }
    }

    fn get_next_value(&mut self) {
        let mut value = self.value * self.factor % self.division;
        while value % self.multiple != 0 {
            value = value * self.factor % self.division;
        } 
        self.value = value;
    }
}

impl Q15 {
    pub fn new() -> Self {
        Q15 {}
    }

    fn part1(&mut self){
        let mut a = Generator::new(634,16807, 4);
        let mut b = Generator::new(301, 48271, 8);

        
        let cnt = (0..5_000_000).map(|_| {
            a.get_next_value();
            b.get_next_value();
            (a.value, b.value)
        }).filter(|(a,b)| a & 0xffff == b & 0xffff).count();
        println!("{:?}", cnt)
    }
}

impl Runner for Q15 {
    fn run(&mut self) {
        self.part1();
    }
}