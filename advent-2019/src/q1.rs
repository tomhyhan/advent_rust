use advent_2019::{Runner, get_file};

pub struct Q1 {

}

struct SpaceCraft {
    masses: Vec<i32>
}

impl SpaceCraft {
    fn new() -> Self{
        let content = get_file("src/input/q1.txt").unwrap();
        let masses = content.lines().map(|line| line.parse::<i32>().unwrap()).collect();
        SpaceCraft {masses}
    }

    fn get_feul(&self) -> i32 {
        self.masses.iter().map(|mass| mass / 3 - 2).sum()
    }

    fn added_feul(&self) -> i32 {
        self.masses.iter().map(|mass| self.add_up_masses(*mass,0)).sum()
    }

    fn add_up_masses(&self, mass: i32, mut total: i32) -> i32{
        let new_mass = mass / 3 - 2;
        if new_mass <= 0 {
            return total
        }
        total += new_mass;
        self.add_up_masses(new_mass, total)
    }
}

impl Q1 {
    pub fn new() -> Self {
        Q1 {}
    }

    fn part1(&mut self) {
        let spacecraft = SpaceCraft::new();
        println!("feul - {:?}", spacecraft.get_feul())
    }

    fn part2(&mut self) {
        let spacecraft = SpaceCraft::new();
        println!("feul - {:?}", spacecraft.added_feul())
    }

}

impl Runner for Q1 {
    fn part1(&mut self) {
        self.part1();
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    use crate::q1::SpaceCraft;

    #[test]
    fn Q1() {
        let s = SpaceCraft::new();
        assert_eq!(966, s.add_up_masses(1969, 0));
    }
}