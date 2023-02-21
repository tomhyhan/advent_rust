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

    fn part1(&mut self) -> usize {
        Q9::decompress(&self.file.data, 1)
    }

    // X(8x2)(3x3)ABCY (18x9)(3x2)TWO(5x7)SEVEN
    fn decompress(line: &str, carry: usize) -> usize {
        let mut d_cnt = 0;
        let l = line.to_string();
        if l.contains('(') {
            let (left, right) = l.split_once('(').unwrap();
            d_cnt += left.len();
            let (marker, data) = right.split_once(')').unwrap();
            let (str_idx, repeat) = marker.split_once('x').unwrap();
            let (data, new_line) = data.split_at(str_idx.parse::<usize>().unwrap());
            d_cnt += Q9::decompress(&data, repeat.parse::<usize>().unwrap() * carry);
            d_cnt += Q9::decompress(&new_line, 1 * carry);
        } else {
            println!("{} {}", l, carry);
            return l.len() * carry;
        }
        d_cnt
    }
}

impl Runner for Q9 {
    fn run(&mut self) {
        println!("{:?}", self.part1());
    }
}

struct File {
    data: String,
}

impl FromStr for File {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s.to_string();
        Ok(File { data })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn returns_correct_decompress() {
        assert_eq!(Q9::decompress("A(2x2)BCD(2x2)EFG", 1), "ABCBCDEFEFG".len());
        assert_eq!(
            Q9::decompress("X(8x2)(3x3)ABCY", 1),
            "XABCABCABCABCABCABCY".len()
        );
        assert_eq!(
            Q9::decompress("(27x12)(20x12)(13x14)(7x10)(1x12)A", 1),
            241920
        );
        assert_eq!(
            Q9::decompress(
                "(25x3)(3x3)ABC(2x3)XY(5x2)PQRSTX(18x9)(3x2)TWO(5x7)SEVEN",
                1
            ),
            445
        );
    }
}
