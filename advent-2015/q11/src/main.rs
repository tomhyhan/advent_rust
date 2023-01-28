use std::{collections::HashSet, hash::Hash};

fn match_char(c: char, rev: &mut String, mut carry: bool) -> bool {
    match carry {
        true => match c {
            'z' => {
                rev.push('a');
                carry = true;
            }
            _ => {
                rev.push(((c as u8) + 1) as char);
                carry = false;
            }
        },
        _ => {
            rev.push(c);
        }
    }
    carry
}

fn increase_char(text: &str) -> String {
    let mut rev: String = String::new();
    let mut carry = true;
    text.chars().rev().for_each(|c| {
        let new_carry = match_char(c, &mut rev, carry);
        carry = new_carry
    });
    // println!("{:?}", rev.chars().rev().collect::<String>());
    rev.chars().rev().collect()
}

fn is_straight(text: &str) -> bool {
    let bytes = text.as_bytes();
    for idx in 0..text.len() - 2 {
        let first = bytes[idx] as usize;
        let second = bytes[idx + 1] as usize;
        let third = bytes[idx + 2] as usize;

        if third - 1 == second && third - 2 == first {
            return true;
        }
    }
    false
}

fn is_letters(text: &str) -> bool {
    if text.contains('i') || text.contains('o') || text.contains('l') {
        return false;
    }
    true
}

fn is_two_diff(text: &str) -> bool {
    // lazy_static! {
    //     static ref RE: Regex = Regex::new(r"[a-z]{2}").unwrap();
    // }
    // let matches = RE.captures(text);
    // println!("{matches:?}");
    //  => back ref is not supported on rust regex ;p

    let bytes = text.as_bytes();

    let mut count = HashSet::new();
    // for c in bytes.windows(2) {
    //     if c[0] == c[1] {
    //         count += 1;
    //     }
    // }
    let mut idx = 0;

    while idx < text.len() - 1 {
        let first = bytes[idx] as u8;
        let second = bytes[idx + 1] as u8;

        if first == second {
            count.insert((first as char, second as char));
            idx += 1
        }
        idx += 1
    }

    // println!("{:?}", count);
    if count.len() >= 2 {
        return true;
    }
    false
}

fn main() {
    // let context = "hepxcrrq";
    let mut text = "hepxxzaa".to_string();
    let mut straight = false;
    let mut letters = false;
    let mut two_diff = false;

    // hepxodza
    println!("{:?}", increase_char("hepxxyzz"));
    while !(straight && letters && two_diff) {
        // println!("{text:?}");
        text = increase_char(&text);
        straight = is_straight(&text);
        letters = is_letters(&text);
        two_diff = is_two_diff(&text);
    }
    assert_eq!(is_straight("ghjaaadd"), false);

    // heqaabcc
    println!("{straight} {letters} {two_diff}");
    println!("{text}");
}
