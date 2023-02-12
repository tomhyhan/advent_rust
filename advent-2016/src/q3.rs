use crate::common::my_modules::get_file;

pub fn run() {
    let content = get_file("q3.txt").unwrap();
    println! {"{}", content};

    let r: Vec<_> = content
        .lines()
        .map(|line| {
            println!("num - {line}");
            let mut l: Vec<_> = line.split(" ").map(|num| num.to_string()).collect();
            println!("{l:?}");
            l.sort();
            l
        })
        .collect();
    // .filter(|p| p[0] + p[1] > p[2])
    // .count();

    println!("{r:?}")
}
