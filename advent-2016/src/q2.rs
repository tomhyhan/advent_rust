use std::collections::HashMap;

use crate::common::my_modules::get_file;
use lazy_static::lazy_static;

lazy_static! {
    static ref KEYPAD: [[i32; 5]; 5] = [
        [0, 0, 1, 0, 0],
        [0, 2, 3, 4, 0],
        [5, 6, 7, 8, 9],
        [0, 11, 12, 13, 0],
        [0, 0, 14, 0, 0]
    ];
}

lazy_static! {
    static ref DIRECTIONS: HashMap<char, (i32, i32)> =
        HashMap::from([('U', (-1, 0)), ('D', (1, 0)), ('L', (0, -1)), ('R', (0, 1)),]);
}

#[derive(Debug)]
struct Location {
    x: i32,
    y: i32,
}

impl Location {
    fn new() -> Self {
        Location { x: 0, y: 2 }
    }
}

fn move_btn(instruction: &str, location: &mut Location) -> i32 {
    instruction.chars().for_each(|c| {
        let nx = location.x + DIRECTIONS.get(&c).unwrap().1;
        let ny = location.y + DIRECTIONS.get(&c).unwrap().0;

        if nx >= 0 && nx < KEYPAD.len() as i32 && ny >= 0 && ny < KEYPAD.len() as i32 {
            // println!("{}", KEYPAD[ny as usize][nx as usize]);
            // println!("{}", nx);
            if !(KEYPAD[ny as usize][nx as usize] == 0) {
                println!("asdf");
                location.x = nx;
                location.y = ny;
            }
        }
    });
    println!("{location:?}");
    // println!("{:?}",);
    KEYPAD[location.y as usize][location.x as usize]
}

pub fn run() {
    let content = get_file("q2.txt").unwrap();

    let mut location: Location = Location::new();

    let password: Vec<i32> = content
        .lines()
        .map(|instruction| move_btn(instruction, &mut location))
        .collect();
    println!("{password:?}");
}

#[cfg(test)]
mod test {

    use super::*;

    fn it_moves_btn_to_right() {
        let mut location: Location = Location::new();
        move_btn("R", &mut location);
        assert_eq!((0, 2), (location.y, location.x))
    }
}
