use lazy_static::lazy_static;
// use math::round;
use regex::Regex;
use std::cmp;
use std::fs;
// fix using formula

#[derive(Default, Debug)]
struct Reindeer {
    speed: i32,
    fly: i32,
    rest: i32,
    distance_so_far: i32,
    points: i32,
    is_flying: bool,
    count: i32,
}

impl Reindeer {
    //  15 10 100 ::
    //  15 * 0 + 1 -> t = 1
    //  15 (1 * 10) + 1 -> t = 111
    //
    //
    fn add_sec(&mut self, time: i32) {
        let t = time + 1;
        self.distance_so_far = self.speed
            * (((t as f32 / (self.fly + self.rest) as f32).floor()) as i32 * self.fly
                + cmp::min(t % (self.fly + self.rest), self.fly));
        // println!("{:?}", self.distance_so_far);
        // match self.is_flying {
        //     true => {
        //         self.distance_so_far += self.speed;
        //         self.count += 1;
        //         if self.count == self.fly {
        //             self.is_flying = false;
        //             self.count = 0;
        //         }
        //     }
        //     false => {
        //         self.count += 1;
        //         if self.count == self.rest {
        //             self.is_flying = true;
        //             self.count = 0;
        //         }
        //     }
        // }
    }
}

fn parse(line: &str) -> Reindeer {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d*").unwrap();
    }

    let caps: Vec<_> = RE
        .find_iter(line)
        .filter_map(|mat| mat.as_str().parse::<i32>().ok())
        .collect();
    println!("{caps:?}");
    println!("{:?}", caps[2]);

    Reindeer {
        speed: caps[0],
        fly: caps[1],
        rest: caps[2],
        is_flying: true,
        ..Default::default()
    }
}

fn get_distance(reindeer: &Reindeer) {
    let mut distance = 0;
    let mut time = 0;

    let time_limit = 1000;
    while time < time_limit {

        // for _ in 0..reindeer.fly {
        //     if time == 2503 {
        //         break;
        //     }
        //     distance += reindeer.speed;
        //     time += 1
        // }
        // for _ in 0..reindeer.rest {
        //     if time == 2503 {
        //         break;
        //     }
        //     time += 1
        // }
    }
    println!("{distance:?}");
}

pub fn run_code() {
    let path = "q14.txt";
    let context = fs::read_to_string(path).expect("file read error");

    let mut reindeers: Vec<_> = context.lines().map(|line| parse(line)).collect();
    println!("{reindeers:?}");

    let mut time = 0;
    let time_limit = 2503;
    while time < time_limit {
        for reindeer in &mut reindeers {
            reindeer.add_sec(time);
        }

        let find_max = reindeers.iter().map(|r| r.distance_so_far).max().unwrap();

        reindeers.iter_mut().for_each(|r| {
            if r.distance_so_far >= find_max {
                r.points += 1
            }
        });

        time += 1
    }

    // 1089 low
    println!("{:?}", reindeers);
    println!(
        "{:?}",
        reindeers.iter().map(|r| r.points).collect::<Vec<i32>>()
    );

    println!("{}", (1 as f32 / 100 as f32).floor());
    println!("{}", 1 % 110)
}
