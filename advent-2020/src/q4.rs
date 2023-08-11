use std::collections::{HashMap, HashSet};

use advent_2020::{Runner, get_file};
use lazy_static::lazy_static;
use regex::Regex;

struct PassportData {
    list: Vec<HashMap<String,String>>,
    key_list: HashSet<String>
}

impl PassportData {
    fn new() -> Self {
        let content = get_file("src/input/q4.txt").unwrap();
        let mut key_list = HashSet::new();
        let raw_passports: Vec<&str>= content.split("\r\n\r\n").collect();
        let list: Vec<_> = raw_passports.iter().map(|raw_passport|{
            let mut passport = HashMap::new();
            raw_passport.lines().for_each(|line| {
                lazy_static! {
                    static ref RE: Regex = Regex::new(r"(?<key>\w+):(?<value>[\w\d#]+\S?)").unwrap();
                }
                RE.captures_iter(line).for_each(|matches| {
                    let key = matches.name("key").unwrap().as_str().to_string();
                    let value = matches.name("value").unwrap().as_str().to_string();
                    key_list.insert(key.clone());
                    passport.insert(key, value);
                });
            });
            passport
        }).collect();
        key_list.remove("cid");
        Self {list,key_list}
    }
}

struct Scanner {}

impl Scanner {
    fn new() -> Self {
        Self {}
    }

    fn is_valid_passport(&self, passport: &HashMap<String,String>, key_list: &HashSet<String>) -> bool {
        // how did other people figure out to meet the required conditions
        let mut key_list = key_list.clone(); 
        for (key, value) in passport.iter() {
            match key.as_str() {
                "byr" => if self.is_valid_byr(&value) {key_list.remove(key);}
                "iyr" => if self.is_valid_iyr(&value) {key_list.remove(key);}
                "eyr" => if self.is_valid_eyr(&value) {key_list.remove(key);}
                "hgt" => if self.is_valid_hgt(&value) {key_list.remove(key);}
                "hcl" => if self.is_valid_hcl(&value) {key_list.remove(key);}
                "ecl" => if self.is_valid_ecl(&value) {key_list.remove(key);}
                "pid" => if self.is_valid_pid(&value) {key_list.remove(key);}
                "cid" => continue,
                _ => panic!("invalid passport property!")
            }
        }
        if self.missing_more_than_one_key(&key_list) {
            return false
        }
        true
    }

    fn is_valid_pid(&self, value: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        if let Some(num) = RE.find(value) {
            if num.as_str().len() == 9 {
                return true
            }
            false
        } else {
            false
        }
    }

    fn is_valid_ecl(&self, value: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"amb|blu|brn|gry|grn|hzl|oth").unwrap();
        }
        if RE.is_match(value) {
            return true
        }
        false
    }

    fn is_valid_hcl(&self, value: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"#[a-f0-9]{6}").unwrap();
        }
        if RE.is_match(value) {
            return true
        }
        false
    }

    fn is_valid_hgt(&self, value: &str) -> bool {
        lazy_static! {
            static ref RE: Regex = Regex::new(r"\d+").unwrap();
        }
        let num = RE.find(value).unwrap().as_str().parse::<i32>().unwrap();
        if value.contains("cm") && num >= 150 && num <= 193 {
            return true
        } 
        if value.contains("in") && num >= 59 && num <= 76 {
            return true
        } 
        return false
    }

    fn is_valid_eyr(&self, value: &str) -> bool {
        let yr = value.parse::<i32>().unwrap();
        if yr >= 2020 && yr <= 2030 {
            return true
        }
        return false
    }

    fn is_valid_iyr(&self, value: &str) -> bool {
        let yr = value.parse::<i32>().unwrap();
        if yr >= 2010 && yr <= 2020 {
            return true
        }
        return false
    }
    
    fn is_valid_byr(&self, value: &str) -> bool {
        let yr = value.parse::<i32>().unwrap();
        if yr >= 1920 && yr <= 2002 {
            return true
        }
        return false
    }
    
    fn missing_more_than_one_key(&self, key_list: &HashSet<String>) -> bool {
        if key_list.len() >= 1 {
            return true
        }
        false
    }
}


pub struct Q4 {

}

impl Q4 {
    pub fn new() -> Self {
        Q4 {}
    }

    fn part1(&mut self) {
        let passports = PassportData::new();
        let scanner = Scanner::new();
        let valid_passports = passports.list.iter().filter(|passport|{
            let r = scanner.is_valid_passport(passport, &passports.key_list);
            r
        }).count();
        println!("{:?}", valid_passports);
    }

    fn part2(&mut self) {
        
    }

}

impl Runner for Q4 {
    fn part1(&mut self) {
        self.part1()
    }
    
    fn part2(&mut self) {
        self.part2()
    }
}


#[cfg(test)]
mod test{
    use super::*;

    #[test]
    fn byr_test() {
        let scanner = Scanner::new(); 
        assert_eq!(scanner.is_valid_byr(&"2002".to_string()), true);
        assert_eq!(scanner.is_valid_byr(&"2003".to_string()), false);
    }
    #[test]
    fn hgt_test() {
        let scanner = Scanner::new(); 
        assert_eq!(scanner.is_valid_hgt(&"60in".to_string()), true);
        assert_eq!(scanner.is_valid_hgt(&"190cm".to_string()), true);
        assert_eq!(scanner.is_valid_hgt(&"190in".to_string()), false);
        assert_eq!(scanner.is_valid_hgt(&"190".to_string()), false);
    }
    #[test]
    fn hcl_test() {
        let scanner = Scanner::new(); 
        assert_eq!(scanner.is_valid_hcl(&"#123abc".to_string()), true);
        assert_eq!(scanner.is_valid_hcl(&"#123abz".to_string()), false);
        assert_eq!(scanner.is_valid_hcl(&"123abz".to_string()), false);
    }
    #[test]
    fn ecl_test() {
        let scanner = Scanner::new(); 
        assert_eq!(scanner.is_valid_ecl(&"brn".to_string()), true);
        assert_eq!(scanner.is_valid_ecl(&"wat".to_string()), false);
    }
    #[test]
    fn pid_test() {
        let scanner = Scanner::new(); 
        assert_eq!(scanner.is_valid_pid(&"000000001".to_string()), true);
        assert_eq!(scanner.is_valid_pid(&"0123456789".to_string()), false);
    }
}