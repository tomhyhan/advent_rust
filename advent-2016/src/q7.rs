use crate::common::my_modules::get_file;
use lazy_static::lazy_static;
use regex::Regex;

use std::{collections::HashSet, str::FromStr};

#[derive(Debug)]
struct Info {
    hyper: Vec<String>,
    abba: Vec<String>,
}
#[derive(Debug)]
struct Ips {
    info: Vec<Info>,
}

fn parse(line: &str) -> Info {
    lazy_static! {
        static ref RE: Regex = Regex::new(r"\[|\]").unwrap();
    }

    let mut hyper = Vec::new();
    let mut abba = Vec::new();

    RE.split(line)
        .enumerate()
        .for_each(|(idx, s)| match idx % 2 {
            1 => hyper.push(s.to_string()),
            0 => abba.push(s.to_string()),
            _ => panic!("not known"),
        });

    Info { hyper, abba }
}

impl Ips {
    fn tls_count(&self) -> usize {
        self.info
            .iter()
            .filter(|&info| {
                println!("{:?}", self.is_tls(info));
                self.is_tls(info)
            })
            .count()
    }

    fn is_tls(&self, info: &Info) -> bool {
        let mut matches = HashSet::new();
        let mut hypers = info.hyper.iter();
        while let Some(hyper) = hypers.next() {
            // println!("{hyper:?}");
            let hyper = hyper.as_bytes();

            for abba in hyper.windows(3) {
                if abba[0] != abba[1] && abba[0] == abba[2] {
                    matches.insert((abba[0], abba[1], abba[0]));
                }
            }
        }

        let mut abbas = info.abba.iter();
        while let Some(abba) = abbas.next() {
            let abba = abba.as_bytes();

            for ab in abba.windows(3) {
                if ab[0] != ab[1] && ab[0] == ab[2] {
                    if matches.contains(&(ab[1], ab[0], ab[1])) {
                        return true;
                    }
                }
                // if ab[0] != ab[1] && ab[0] == ab[2] {
                //     return true;
                // }
            }
        }

        false
    }
}

impl FromStr for Ips {
    type Err = ();
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let info: Vec<Info> = s.lines().map(|line| parse(line)).collect();

        Ok(Ips { info })
    }
}

pub fn run() {
    let content = get_file("q7.txt").unwrap();

    let ips: Ips = content.parse().unwrap();
    println!("{:?}", ips.tls_count());
}
