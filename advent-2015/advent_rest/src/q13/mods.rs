use permutohedron;

use std::cmp;
use std::fs;
use std::i32;
fn get_input() -> Vec<Vec<i32>> {
    let input = fs::read_to_string("q13.txt").expect("msg");
    let first_guest = input.split_whitespace().next().unwrap();
    let num_guests = input
        .lines()
        .take_while(|line| line.starts_with(first_guest))
        .count()
        + 1;
    let mut retval = vec![vec![0; num_guests]; num_guests];
    for (line_no, line) in input.lines().enumerate() {
        let i = line_no / (num_guests - 1);
        let mut j = line_no % (num_guests - 1);
        if j >= i {
            j += 1;
        }
        let direction = if line.split_whitespace().nth(2) == Some("gain") {
            1
        } else {
            -1
        };
        let amount: i32 = line.split_whitespace().nth(3).unwrap().parse().unwrap();
        retval[i][j] = direction * amount;
    }
    retval
}

fn best_seating(table: &Vec<Vec<i32>>) -> i32 {
    let mut order = (0..table.len()).collect::<Vec<_>>();
    let mut permutations = permutohedron::Heap::new(&mut order);
    let mut max = i32::MIN;
    while let Some(permutation) = permutations.next_permutation() {
        let mut total = 0;
        for i in 0..table.len() {
            let curr = permutation[i];
            let next = permutation[(i + 1) % permutation.len()];
            total += table[curr][next];
            total += table[next][curr];
        }
        max = cmp::max(max, total);
    }
    max
}

fn add_self(table: &mut Vec<Vec<i32>>) {
    let new_num_guests = table.len() + 1;
    for row in table.iter_mut() {
        row.push(0);
    }
    table.push(vec![0; new_num_guests]);
}

pub fn solve() {
    let mut input = get_input();
    println!("{input:?}");
    println!("First: {}", best_seating(&input));
    //     add_self(&mut input);
    //     println!("Second: {}", best_seating(&input));
}
