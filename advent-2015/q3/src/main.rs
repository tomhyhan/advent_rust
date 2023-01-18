use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fs;

lazy_static! {
    static ref DIRECTIONS: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('<', (-1, 0))]);
}
// let directions = HashMap::from([('>', (1, 0)), ('<', (-1, 0))]);
fn main() {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("Error reading a file");
}
