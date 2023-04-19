use std::{collections::{HashSet, HashMap, VecDeque}, any};

use advent_2018::{Runner, get_file};

#[derive(Debug)]
struct Sleigh {
    parts: HashSet<char>,
    prereqs: HashMap<char, Vec<char>>,
    depen_numbers: HashMap<char, i32>,
    workers: Vec<Worker>
}

#[derive(Debug, Default, Clone)]
struct Worker {
    job: Vec<char>,
    time: i32,
    is_working: bool,
    finish: bool,
}

impl Worker {
    fn new() -> Self {
        let job = vec![];
        let time = 0;
        let is_working = false;
        let finish = false;
        Worker {
            job, time, is_working, finish
        }
    }

    fn next(&mut self) {
        if self.time > 0 {
            self.time -= 1;
            if self.time == 0 {
                self.finish = true
            }
        }
    }
}

impl Sleigh {
    fn new() -> Self {
        let contents = get_file("src/input/q7.txt").unwrap();
        let mut parts = HashSet::new();
        let mut prereqs: HashMap<char, Vec<char>> = HashMap::new();
        let mut depen_numbers: HashMap<char, i32> = HashMap::new();
        contents.lines().for_each(|line|parse(line, &mut parts, &mut prereqs, &mut depen_numbers));
        for prereq in prereqs.values_mut() {
            prereq.sort_by(|x,y| y.cmp(x));
        }
        println!("{:?}", prereqs);
        println!("{:?}", depen_numbers);
        let workers = vec![Worker::new();5];
        Self {parts, prereqs, depen_numbers, workers}
    }

    // method 2
    //  [c] , [ , ]
    //  []  , [c, ]
    fn run2(&mut self) {
        let mut time = 0;
        let mut queue = vec![]; 

        for part in self.parts.iter() {
            if !self.depen_numbers.contains_key(&part){
                queue.push(*part);
            }
        }
        queue.sort_by(|x,y| y.cmp(x));
        
        loop {
            if queue.is_empty() && self.workers.iter().all(|w| w.time == 0) {
                break
            };
            for worker in self.workers.iter_mut() {
                if worker.time == 0 && !queue.is_empty() {
                    queue.sort_by(|x,y| y.cmp(x));
                    let job = queue.pop().unwrap();
                    worker.job.push(job);
                    worker.time =  (job as u8 - b'A' + 61) as i32;
                }
            }
            for worker in self.workers.iter_mut() {
                worker.next();
                if worker.finish {
                    let job = worker.job.pop().unwrap();
                    for dep in self.prereqs.get(&job).unwrap_or(&vec![]) {
                        let val = self.depen_numbers.get_mut(&dep).unwrap();
                        *val -= 1;
                        if *val == 0 {
                            queue.push(*dep)
                        }
                    }
                    worker.finish = false
                }

            }

            // println!("{time} - {:?} {queue:?}", self.workers);
            time += 1;
        }
        println!("{:?}", time);
    }

    // method 1
    fn run1 (&mut self) {
        let mut worker_processes = HashSet::new();
        let mut queue = vec![]; 

        for part in self.parts.iter() {
            if !self.depen_numbers.contains_key(&part){
                queue.push(*part);
            }
        }

        queue.sort_by(|x,y| y.cmp(x));
        self.distribute_jobs(&mut queue, &mut worker_processes, 0);

        while !worker_processes.is_empty() || !queue.is_empty() {
            let (time, current_job) = worker_processes.iter().min_by_key(|w| w.0).unwrap().clone();
            worker_processes.remove(&(time, current_job));
            for dep in self.prereqs.get(&current_job).unwrap_or(&vec![]) {
                let val = self.depen_numbers.get_mut(dep).unwrap();
                *val -= 1;
                if *val == 0 {
                    queue.push(*dep);
                } 
            }
            queue.sort_by(|x,y| y.cmp(x));
            self.distribute_jobs(&mut queue, &mut worker_processes, time);
        }
    }


    fn distribute_jobs(&self, queue: &mut Vec<char>, worker_processes: &mut HashSet<(i32,char)>, time:i32) {
        while worker_processes.len() < 5 && !queue.is_empty() {
            let job = queue.pop().unwrap();
            println!("{:?}", time + (job as u8 - b'A' + 61) as i32);
            worker_processes.insert((time + (job as u8 - b'A' + 61) as i32, job));
        }
    }
    
    fn topological_sort(&mut self, queue: &mut Vec<char>) {

        // distribute to workers


        // let prereqs = match self.prereqs.get(&part) {
        //     Some(values) => Some(values.clone()),
        //     None => Some(vec![])
        // };
        // for prereq_char in prereqs.unwrap().iter() {
        //     let num = self.depen_numbers.get_mut(prereq_char).unwrap();
        //     *num -= 1;
        //     if *num == 0 {
        //         queue.push(prereq_char.clone()) 
        //     }
        // }

    }
}



// Step F must be finished before step E can begin.
fn parse(line: &str, parts: &mut HashSet<char>, prereqs: &mut HashMap<char, Vec<char>>, depen_numbers: &mut HashMap<char, i32>) {
    let tokens: Vec<_>= line.split_whitespace().collect();
    let prereq_part =  tokens[1].chars().next().unwrap();
    let dep_part =  tokens[7].chars().next().unwrap();
    parts.insert(prereq_part);
    parts.insert(dep_part);
    prereqs.entry(prereq_part).or_default().push(dep_part);
    depen_numbers.entry(dep_part).and_modify(|cnt| *cnt += 1).or_insert(1);
}

pub struct Q7 {

}

impl Q7 {
    pub fn new() -> Self {
        Q7 {}
    }

    fn part1(&mut self) {
        let mut sleigh = Sleigh::new();
    }

    fn part2(&mut self) {
        let mut sleigh = Sleigh::new();
        sleigh.run2()
    }

}

impl Runner for Q7 {
    fn run(&mut self) {
        // self.part1();
        self.part2();
    }
}


#[cfg(test)]
mod test{
    #[test]
    fn Q7() {
        assert_eq!(1, 1);
    }
}