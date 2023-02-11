use std::collections::HashMap;

fn main() {
    // let s = vec![[1; 3000]; 3000];
    // let mut sum = 0;
    // for i in s.iter().flatten() {
    //     sum += i;
    //     continue;
    // }
    // println!("s is done : {sum}");

    let x: &str = "asdf";
    let y: String = "asdf".to_string();
    let mut manual = HashMap::new();

    manual.insert((1, 1), 20151125);
    let mut row = 1;
    let mut col = 1;
    let mut max_row = 1;
    while row <= 3010 || col <= 3019 {
        if row == 1 {
            let current = manual.get(&(row, col)).unwrap();
            max_row += 1;
            row = max_row;
            col = 1;
            manual.insert((max_row, 1), get_next_sum(current));
        } else {
            let current = manual.get(&(row, col)).unwrap();
            row -= 1;
            col += 1;
            manual.insert((row, col), get_next_sum(current));
        }
    }

    // let sum = get_next_sum(&20151125);
    println!("{}", manual.get(&(3010, 3019)).unwrap())
}

fn get_next_sum(code: &i64) -> i64 {
    (code * 252533) % 33554393
}
