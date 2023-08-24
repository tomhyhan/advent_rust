use advent_2020::{Runner, get_file};

struct Navigation {
    instructions: Vec<(char, i32)>,
    head: i32,
    current: (i32,i32),
    way_pt: (i32,i32)
}

impl Navigation {
    fn new() -> Self {
        let content = get_file("src/input/q12.txt").unwrap();
        let mut instructions = vec![];
        content.lines().for_each(|line| {
            let info: Vec<char>= line.chars().collect(); 
            let ins = info[0];
            let units = *&info[1..].iter().collect::<String>().parse::<i32>().unwrap();
            instructions.push((ins, units));
        });
        let head = 0;
        let current = (0,0);
        let way_pt = (-1,10);
        Self {head,instructions, current, way_pt}
    }

    fn start(&mut self) {
        // let directions = [(0,1),(1,0),(0,-1),(-1,0)];
        // E S W N
        
        for (ins, unit) in self.instructions.iter() {
            match ins {
                'N' => {
                    self.way_pt.0 -= unit
                }
                'S' => {
                    self.way_pt.0 += unit
                }
                'W' => {
                    self.way_pt.1 -= unit
                }
                'E' => {
                    self.way_pt.1 += unit
                }
                // (north east)
                'L' => {
                    // let rotate = self.turn_head(*unit);
                    // self.head = if self.head - rotate >= 0 {self.head - rotate} else {4 + self.head - rotate}
                    for _ in 0..unit/90 {
                        self.way_pt = (-self.way_pt.1, self.way_pt.0);
                    }
                }
                'R' => {
                    // let rotate = self.turn_head(*unit);
                    // self.head = (self.head + rotate) % 4;
                    for _ in 0..unit/90 {
                        self.way_pt = (self.way_pt.1, -self.way_pt.0);
                    }
                }
                'F' => {
                    self.current.0 += self.way_pt.0 * unit;
                    self.current.1 += self.way_pt.1 * unit;
                }
                _ => panic!("unknown instruction!")
            }
            println!("pos - {:?}", self.current);
        }
        println!("head - {:?}", self.head);
        println!("pos - {:?}", self.current);
        println!("Manhattan distance - {:?}", self.current.0.abs() + self.current.1.abs());
    }


    fn turn_head(&self, turn: i32) -> i32{
        match turn {
            90 => 1,
            180 => 2,
            270 => 3,
            _ => panic!("not known turn!")
        }
    }
}

pub struct Q12 {

}

impl Q12 {
    pub fn new() -> Self {
        Q12 {}
    }

    fn part1(&mut self) {
        let mut nav = Navigation::new();
        println!("{:?}", nav.instructions);
        nav.start()
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q12 {
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
    fn Q12() {
        assert_eq!(1, 1);
    }
}