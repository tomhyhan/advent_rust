use crate::common::my_modules::get_file;
use lazy_static::lazy_static;
use regex::Regex;

fn parse(line: &str) -> Vec<i32> {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\d+").unwrap();
    }
    let mut nums: Vec<_> = RE
        .find_iter(line)
        .map(|num| num.as_str().parse::<i32>().unwrap())
        .collect();

    // println!("{nums:?}")
    // nums.sort();
    nums
}
pub fn run() {
    let content = get_file("q3.txt").unwrap();
    println! {"{}", content};

    let part1 = content
        .lines()
        .map(|line| parse(line))
        .filter(|p| p[0] + p[1] > p[2])
        .count();

    println!("{part1:?}");

    let mut part2: Vec<_> = content.lines().map(|line| parse(line)).collect();
    let mut cnt = 0;
    part2.chunks_mut(3).for_each(|line| {
        println!("{line:?}");
        for i in 0..3 {
            let mut temp = vec![line[0][i].clone(), line[1][i].clone(), line[2][i].clone()];
            // println!("{temp:?}");
            temp.sort();
            if temp[0] + temp[1] > temp[2] {
                cnt += 1;
            }
        }
    });
    println!("{cnt}")
}
