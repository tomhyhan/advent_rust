use advent_2020::{Runner, get_file};

struct Area {
    grid: Vec<Vec<char>>,
}

impl Area {
    fn new() -> Self {
        let content = get_file("src/input/q3.txt").unwrap();
        let grid = content.lines().map(|line| line.chars().collect()).collect();
        Self {grid}
    }

    fn traverse_area(&self, right: usize, down: usize) -> i64 {
        let mut row = 0;
        let mut col = 0;

        let mut trees = 0;
        while row < self.grid.len() {
            let current_tile = self.grid[row][col]; 
            if current_tile == '#' {
                trees += 1
            }
            row += down;
            col = (col + right) % self.grid[0].len()
        }
        trees
    }
}

pub struct Q3 {

}

impl Q3 {
    pub fn new() -> Self {
        Q3 {}
    }

    fn part1(&mut self) {
        let area = Area::new();
        println!("{:?}", area.traverse_area(3,1))
    }

    fn part2(&mut self) {
        let area = Area::new();
        let mut trees = 1;
        for (r,d) in [(1,1),(3,1),(5,1),(7,1),(1,2)] {
            trees *= area.traverse_area(r, d);
        }
        println!("{:?}", trees);
    }

}

impl Runner for Q3 {
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
    fn Q3() {
        assert_eq!(1, 1);
    }
}