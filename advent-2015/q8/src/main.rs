use std::fs::File;
use std::io::{self, Read};

fn read_username_from_file() -> String {
    let mut username_file = File::open("input.txt").expect("asdf");
    let mut username = String::new();
    username_file.read_to_string(&mut username);
    username
    // Ok(username)
}
fn main() {
    println!("{}", read_username_from_file().unwrap());
}
