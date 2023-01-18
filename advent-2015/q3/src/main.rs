use lazy_static::lazy_static;
use std::collections::{HashMap, HashSet};
use std::fs;
use modular::*;

lazy_static! {
    static ref DIRECTIONS: HashMap<char, (i32, i32)> =
        HashMap::from([('>', (1, 0)), ('<', (-1, 0)),('^', (0, -1)),('v', (0, 1))]);
}
// let directions = HashMap::from([('>', (1, 0)), ('<', (-1, 0))]);

fn move_santa(direction: char, coords: &mut HashSet<Vec<i32>>, cc : &mut Vec<i32>) {
    // println!("{:?}",direction);
    let move_to = DIRECTIONS.get(&direction).unwrap();
    // println!("{:?}",move_to);
    cc[0] = cc[0] + move_to.0; 
    cc[1] = cc[1] + move_to.1;
    coords.insert(cc.clone()); 
}

fn num_of_present(context: String) {
    let mut coords = HashSet::new();
    let mut cc = vec![0,0];
    let mut cr = vec![0,0];
    coords.insert(cc.clone());

    context.chars().enumerate().for_each(|(idx, direction)| {
        // let idx: i32 = idx.try_into().unwrap();
        // if let 0 = idx % 2 {
        //     move_santa(direction, &mut coords, &mut cc)
        // }

        match idx % 2 {
            0 => move_santa(direction, &mut coords, &mut cc),
            1 => move_santa(direction, &mut coords, &mut cr),
            _ => ()
        }
        
    });
    
    // assert!(coords.contains(&vec![0,-1]));
    println!("{:?}", coords.len());
}
fn main() {
    let path = "input.txt";
    let context = fs::read_to_string(path).expect("Error reading a file");

    // println!("{}", context);
    // println!("{:?}", DIRECTIONS.get(&'>').unwrap());
    // println!("{:?}", DIRECTIONS.get(&'>').unwrap());
    num_of_present(context)

}
