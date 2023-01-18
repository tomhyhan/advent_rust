use std::fs;

fn turn_to_num(letter: char) -> i32 {
    match letter {
        '(' => 1,
        ')' => -1,
        _ => 3,
    }
}

fn find_position(a: i32,b: i32) -> i32{
    a+b
}

fn main() {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("err when reading a file");
    println!("asdf");
    let mut idx = 0;
    let result: Option<i32> = context.chars().map(|letter| turn_to_num(letter)).reduce(|a,b| {
        idx += 1;
        find_position(a,b)});

    // .collect::<Vec<i8>>();
    println!("{:?}",result);
    println!("final position - {}", idx + 1);
}
