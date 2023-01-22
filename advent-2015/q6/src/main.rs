use lazy_static::lazy_static;
use regex::Regex;
use std::cmp;
use std::fs;

fn parse(line: &str) -> Vec<&str> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"on|off|toggle|\d+").unwrap();
    }
    let caps: Vec<&str> = RE.find_iter(line).map(|mat| mat.as_str()).collect();

    caps
}

fn toggle_lights(line: &str, lights: &mut Vec<Vec<isize>>) -> String {
    let caps = parse(line);

    let command = caps[0];
    let x: usize = caps[1].parse().ok().unwrap();
    let y: usize = caps[2].parse().ok().unwrap();
    let end_x: usize = caps[3].parse().ok().unwrap();
    let end_y: usize = caps[4].parse().unwrap();

    // println!("{command:?}");
    // println!("{x:?}");
    // println!("{y:?}");
    // println!("{end_x:?}");
    // println!("{end_y:?}");

    match command {
        "off" => {
            for row in y..=end_y {
                for col in x..=end_x {
                    lights[row][col] = cmp::max(lights[row][col] - 1, 0)
                }
            }
        }
        "on" => {
            for row in y..=end_y {
                for col in x..=end_x {
                    lights[row][col] += 1
                }
            }
        }
        "toggle" => {
            for row in y..=end_y {
                for col in x..=end_x {
                    lights[row][col] += 2
                }
            }
        }
        __ => panic!("no command found"),
    }

    "asd".to_string()
}

fn main() {
    // let mut lights : Vec<Vec<bool>> = (0..=999).map(|_| (0..=999).map(|_| false).collect()).collect();
    // let mut lights : Vec<Vec<bool>> = (0..=2).map(|_| (0..=2).map(|_| false).collect()).collect();

    let mut lights: Vec<Vec<isize>> = (0..=999).map(|_| (0..=999).map(|_| 0).collect()).collect();

    let path = "input.txt";
    let context: Vec<String> = fs::read_to_string(path)
        .expect("cannot read a file")
        .split("\n")
        .map(|line| toggle_lights(line, &mut lights))
        .collect();

    // assert_eq!(lights[499][499], false);
    // assert_eq!(lights[0][499], false);

    let ons: isize = lights
        .iter()
        .map(|line| line.iter().sum())
        .collect::<Vec<_>>()
        .iter()
        .sum();

    println!("{:?}", ons);
}
