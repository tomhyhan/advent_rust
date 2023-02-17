use std::str::FromStr;

use crate::{common::my_modules::get_file, Runner};

pub struct Q9 {
    file: File,
}

impl Q9 {
    pub fn new() -> Q9 {
        let file: File = get_file("q9.txt").unwrap().parse().unwrap();
        Q9 { file }
    }

    fn part1(&mut self) -> Vec<String> {
        self.file
            .data
            .iter()
            .map(|line| Q9::decompress(line))
            .collect()
    }

    // X(8x2)(3x3)ABCY
    fn decompress(line: &str) -> String {
        let mut d_str = String::new();
        let mut l = line.to_string();
        while l.contains('(') {
            println!("{l}");
            let (left, right) = l.split_once('(').unwrap();
            d_str.push_str(left);
            let (marker, data) = right.split_once(')').unwrap();
            let (str_idx, repeat) = marker.split_once('x').unwrap();
            let (data, new_line) = data.split_at(str_idx.parse::<usize>().unwrap());
            d_str.push_str(&data.repeat(repeat.parse::<usize>().unwrap()));
            l = new_line.to_string();
        }
        d_str.push_str(&l);
        d_str
    }
}

impl Runner for Q9 {
    fn run(&mut self) {
        println!(
            "{:?}",
            self.part1().into_iter().map(|s| s.len()).sum::<usize>()
        );

        let mut d = "(177x14)(126x6)(3x5)WAF".to_string();
        while d.contains('(') {
            let n = Q9::decompress(&d);
            println!("{}", n.len());
            d = n
        }
        println!("{:?}", d.len());
    }
}

struct File {
    data: Vec<String>,
}

impl FromStr for File {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.lines().map(|line| line.to_string()).collect();
        Ok(File { data })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_correct_decompress() {
        assert_eq!(Q9::decompress("A(2x2)BCD(2x2)EFG"), "ABCBCDEFEFG");
        assert_eq!(Q9::decompress("(6x1)(1x3)A"), "(1x3)A");
        assert_eq!(Q9::decompress("X(8x2)(3x3)ABCY"), "X(3x3)ABC(3x3)ABCY");
    }
}
