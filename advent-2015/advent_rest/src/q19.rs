use crate::common::read_file;

pub fn run() {
    let content = read_file("q15.txt").unwrap();
    println!("{content:?}")
}
