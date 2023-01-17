use std::fs;

fn turn_to_num(letter: char) -> i32 {
    match letter {
        '(' => 1,
        ')' => -1,
        _ => 3,
    }
}

fn main() {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("err when reading a file");
    println!("asdf");
    let result: i32 = context.chars().map(|letter| turn_to_num(letter)).sum();

    // .collect::<Vec<i8>>();
    println!("{:?}", result);
}
