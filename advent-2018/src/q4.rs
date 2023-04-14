use std::{cmp::Ordering, collections::{HashMap, HashSet}};

use advent_2018::{Runner, get_file};

pub struct Q4 {
    records: Vec<Record>
}

#[derive(Debug, Eq)]
struct Record {
    year: i32,
    month: i32,
    day: i32,
    hour: i32,
    min: i32,
    detail: String
}

impl Ord for Record {
    fn cmp(&self, other: &Self) -> Ordering {
        self.year.cmp(&other.year)
        .then(self.month.cmp(&other.month))
        .then(self.day.cmp(&other.day))
        .then(self.hour.cmp(&other.hour))
        .then(self.min.cmp(&other.min))
        .then(self.detail.cmp(&other.detail))
    }
}

impl PartialOrd for Record {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Record {
    fn eq(&self, other: &Self) -> bool {
        self.year == other.year &&
        self.month == other.month &&
        self.day == other.day &&
        self.hour == other.hour &&
        self.min == other.min &&
        self.detail == other.detail
    }
}

impl Q4 {
    pub fn new() -> Self {
        let contents = get_file("src/input/q4.txt").unwrap();
        let records: Vec<_>= contents.lines().map(|line| {
            let (left, right) = line.split_once(']').unwrap();
            let time: Vec<i32>= left.trim_matches('[').split(|p| p =='-' || p ==':' || p ==' ').map(|token| {
                token.parse::<i32>().unwrap()
            }).collect();
            Record {year: time[0], month: time[1], day: time[2], hour: time[3], min: time[4], detail: right.to_string()}
        }).collect();

        Q4 {records}
    }

    fn part1(&mut self) {
        let mut times: HashMap<i32,i32> = HashMap::new();
        let mut guards: HashMap<i32, HashMap<i32,HashSet<i32>>>= HashMap::new();
        self.records.sort();

        let mut id = None;
        let mut fall = None;
        for record in self.records.iter() {
            if record.detail.contains("begins") {
                id = Some(record.detail.split_whitespace().nth(1).unwrap().trim_matches('#').parse::<i32>().unwrap());
            } else if record.detail.contains("falls asleep") {
                fall = Some(record.min)
            } else if record.detail.contains("wakes up"){
                let total_time = record.min - fall.unwrap();
                let g_id = id.unwrap();
                *times.entry(g_id).or_insert(0) += total_time;
                let inner = guards.entry(g_id).or_insert(HashMap::new());
                for t in fall.unwrap()..record.min {
                    inner.entry(record.day).or_insert(HashSet::new()).insert(t);
                }
            }
        }
        
        // part 1
        // let max_time = times.iter().min_by(|x,y|x.1.cmp(&y.1)).unwrap();
        // println!("{:?}", max_time);
        // let guard = guards.get(max_time.0).unwrap();
        

        // part 2
        for (id, guard) in guards.iter() {
            let mut count = HashMap::new();
            for (_,g) in guard.iter() {
                for num in g.iter().cloned() {
                    count.entry(num).and_modify(|cnt| *cnt += 1).or_insert(1);
                }
            }
            let min = count.iter().max_by_key(|x| x.1).unwrap();
            println!("{id} {:?} {:?}", min, min.0 * id);
        }

    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q4 {
    fn run(&mut self) {
        self.part1();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q4() {
        assert_eq!(1, 1);
    }
}