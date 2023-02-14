use crate::common::my_modules::get_file;
use lazy_static::lazy_static;
use regex::Regex;
use std::{collections::HashMap, hash::Hash, str::FromStr};

#[derive(Debug)]
struct Encrypted {
    name: String,
    sector: i32,
    checksum: String,
}

impl Encrypted {
    fn new() -> Self {
        Encrypted {
            name: String::new(),
            sector: 0,
            checksum: String::new(),
        }
    }
}

#[derive(Debug)]
struct Rooms {
    info: Vec<Encrypted>,
}

impl FromStr for Rooms {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut info: Vec<Encrypted> = s.lines().map(|line| parse(line)).collect();

        Ok(Rooms { info })
    }
}
// (?P<name>.+)(?P<sector>\d+)(?P<checksum>(?:\[)\w+\])
fn parse(line: &str) -> Encrypted {
    lazy_static! {
        static ref RE: Regex =
            Regex::new(r"(?P<name>[a-z-]+)(?P<sector>\d+)(?P<checksum>\[\w+\])").unwrap();
    }

    let test = RE.captures(line).unwrap();
    // let x = test["name"].to_string();
    // println!("t = {:?}", x);
    let mut encrypted = Encrypted::new();
    for caps in RE.captures_iter(line) {
        encrypted.name = caps["name"].to_string();
        encrypted.sector = caps["sector"].parse::<i32>().unwrap();
        encrypted.checksum = caps["checksum"].to_string();
    }

    encrypted
}

fn is_real_room(info: &Encrypted) -> bool {
    // println!("{}", info.name);
    // println!("{}", info.name);
    let mut alpha = HashMap::new();

    info.name.chars().for_each(|c| {
        if c == '-' {
            return;
        }
        alpha
            .entry(c)
            .and_modify(|counter| *counter += 1)
            .or_insert(1);
    });

    let mut char_freqs = alpha.into_iter().map(|(a, b)| (b, a)).collect::<Vec<_>>();

    println!("{char_freqs:?}");

    char_freqs.sort_by_key(|info| (info.0));

    println!("{char_freqs:?}");

    let checksum = info.checksum.trim_matches(|pat| pat == '[' || pat == ']');

    char_freqs
        .into_iter()
        .rev()
        .take(5)
        .zip(checksum.chars())
        .all(|pair| {
            // println!("{pair:?}");
            (pair.0).1 == pair.1
        })
    // println!("{alpha:?}");

    // let mut is_real = true;

    // let checksum = info.checksum.trim_matches(|pat| pat == '[' || pat == ']');
    // // println!("{checksum}");

    // for chars in checksum.as_bytes().windows(2) {
    //     let prev = chars[0] as char;
    //     let next = chars[1] as char;
    //     if !(alpha.contains_key(&prev) && alpha.contains_key(&next)) {
    //         is_real = false;
    //         break;
    //     }
    //     if alpha.get(&prev).unwrap() < alpha.get(&next).unwrap() {
    //         is_real = false;
    //         break;
    //     }
    //     if alpha.get(&prev).unwrap() == alpha.get(&next).unwrap() && prev > next {
    //         println!("{prev:?} {next:?}");
    //         is_real = false;
    //         break;
    //     }
    // }
    // // println!("{is_real}");
    // if is_real {
    //     true
    // } else {
    //     false
    // }
}

fn shift(c: char, real_name: &mut String, id: i32) {
    if c == '-' {
        real_name.push_str(" ");
        return;
    }
    let mut bc = c as u8;

    let shift_to: u8 = (id % 26).try_into().unwrap();
    //  97
    bc = (bc - 97 + shift_to) % 26 + 97;
    real_name.push(bc as char);
}

fn decrypt(info: Encrypted) -> String {
    let mut real_name = String::new();

    info.name
        .chars()
        .for_each(|c| shift(c, &mut real_name, info.sector));
    real_name.push_str(&info.sector.to_string());
    real_name
}

pub fn run() {
    let content = get_file("q4.txt").unwrap();
    let rooms: Rooms = content.parse().unwrap();
    // println!("{:?}", rooms.info);

    let r: i32 = rooms
        .info
        .into_iter()
        .filter(|info| is_real_room(info))
        .map(|info| info.sector)
        .sum();
    println!("{:?}", r);
    // let encryted: Vec<_> = r.into_iter().map(|info| decrypt(info)).collect();
    // println!(
    //     "{:?}",
    //     encryted
    //         .iter()
    //         .find(|p| p.contains("northpole object"))
    //         .unwrap()
    // );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_contains_correct_char() {
        let mut test = String::new();
        shift('q', &mut test, 343);

        assert_eq!("v", test)
    }
}
