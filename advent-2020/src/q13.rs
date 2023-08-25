use advent_2020::{Runner, get_file};

const MAX: usize= usize::MAX;
struct Shuttle {
    depart_time: usize,
    bus_time: Vec<usize>
}

impl Shuttle {
    fn new() -> Self{
        let content = get_file("src/input/q13.txt").unwrap();
        let info: Vec<&str>= content.lines().collect();
        let depart_time = info[0].parse::<usize>().unwrap();
        let bus_time = info[1].split(',').map(|time| match time.parse() {
            Ok(num) => num,
            Err(_) => MAX
        }).collect();
        Shuttle {bus_time,depart_time}
    }

    fn find_earliest_bus(&self) {
        let mut min_depart = MAX;
        let mut id = 0;
        for time in self.bus_time.iter() {
            if *time == MAX {
                continue
            }
            let previous_bus = self.depart_time / *time;
            let next_bus = (previous_bus + 1) * *time; 
            let arrive = next_bus - self.depart_time;
            if arrive < min_depart {
                min_depart = arrive;
                id = *time
            }
        }
        println!("earliest_bus: {:?}", min_depart * id)
    }

    fn earliest_offset(&self) {
        let mut diff = 1;
        let mut share = 1;
        for (offset, time) in self.bus_time.iter().enumerate() {
            if *time == MAX {
                continue
            }
            let mut diff_tracker = vec![];
            while diff_tracker.len() <= 2 {
                if (share + offset) % *time == 0 {
                    diff_tracker.push(share);
                }
                share += diff
            }
            diff = diff_tracker[1] - diff_tracker[0]; 
            share = diff_tracker[0];
        }
        println!("first bus: {:?}", share)
    }
}

pub struct Q13 {

}

impl Q13 {
    pub fn new() -> Self {
        Q13 {}
    }

    fn part1(&mut self) {
        let shuttle = Shuttle::new();
        shuttle.find_earliest_bus();
    }
    
    fn part2(&mut self) {
        let shuttle = Shuttle::new();
        shuttle.earliest_offset()
    }

}

impl Runner for Q13 {
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
    fn Q13() {
        assert_eq!(1, 1);
    }
}