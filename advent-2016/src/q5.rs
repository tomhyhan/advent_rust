use std::vec;

use md5;
use regex::internal::Char;

fn decode(code: &str) -> Vec<Option<char>> {
    let mut c = code.to_string();
    let mut idx = 0;
    let mut passwords: Vec<Option<char>> = vec![None, None, None, None, None, None, None, None];
    while passwords.iter().any(|p| p.is_none()) {
        let new = format!("{}{}", c, idx);

        let x = md5::compute(new);
        if format!("{:?}", x).starts_with("00000") {
            let position = format!("{:?}", x).as_bytes()[5];
            let password = format!("{:?}", x).as_bytes()[6] as char;
            println!("{position:?}");
            if position < 48 || position > 55 {
                idx += 1;
                continue;
            }

            let new_p = (position as char).to_digit(10).unwrap() as usize;

            if !passwords[new_p].is_none() {
                idx += 1;
                continue;
            }

            passwords[new_p] = Some(password);
            println!("{passwords:?}");
            // println!("{position:?}");
            // println!("{position}")

            // if position.parse::<i32>.ok() && position >= 8 {

            // }

            // println!("starting with 0 {x:?}");
            // passwords.push(format!("{:?}", x).as_bytes()[5] as char);
        }
        idx += 1
    }
    passwords
}

pub fn run() {
    // format!("{:?}", x).contains(pat);
    println!("{:?}", md5::compute("abc3231929"));

    println!("{}", b'0');
    println!("{}", b'7');
    let passwords = decode("ojvtpuvg");
    println!("{:?}", passwords);
}
