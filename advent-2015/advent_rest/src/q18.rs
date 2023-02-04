use core::panic;

use crate::common::read_file;

#[derive(Debug)]
struct Lights {
    grid: Vec<Vec<char>>,
}

const DIRECTIONS: [[i32; 2]; 8] = [
    [-1, -1],
    [0, -1],
    [1, -1],
    [-1, 0],
    [1, 0],
    [-1, 1],
    [0, 1],
    [1, 1],
];

impl Lights {
    fn new(length: usize) -> Self {
        Lights {
            grid: (0..length)
                .map(|_| (0..length).map(|_| '.').collect())
                .collect(),
        }
    }
}

impl From<String> for Lights {
    fn from(content: String) -> Self {
        // let mut new_grid = Lights::new(100);

        let init: Vec<Vec<char>> = content.lines().map(|line| line.chars().collect()).collect();

        Lights { grid: init }
    }
}

fn move_steps(lights: Lights, steps: usize, length: usize) -> usize {
    let mut current = lights.grid;
    let mut new_map = Lights::new(length);
    for _ in 0..steps {
        for row in 0..length {
            for col in 0..length {
                let luc = row == 0 && col == 0;
                let ruc = row == 0 && col == current.len() - 1;
                let ldc = row == current.len() - 1 && col == 0;
                let rdc = row == current.len() - 1 && col == current.len() - 1;
                if luc || ruc || ldc || rdc {
                    new_map.grid[row][col] = '#';
                    continue;
                }
                match current[row][col] {
                    '.' => {
                        let neighbors = find_neighbors(row as i32, col as i32, &current);
                        let lights_count = neighbors
                            .iter()
                            .filter(|coord| current[coord[0]][coord[1]] == '#')
                            .count();

                        if lights_count == 3 {
                            new_map.grid[row][col] = '#'
                        }
                    }
                    '#' => {
                        let neighbors = find_neighbors(row as i32, col as i32, &current);
                        let lights_count = neighbors
                            .iter()
                            .filter(|coord| current[coord[0]][coord[1]] == '#')
                            .count();
                        // println!("{row} {col}");
                        // println!("{lights_count:?}");
                        if lights_count == 2 || lights_count == 3 {
                            new_map.grid[row][col] = '#'
                        } else {
                            new_map.grid[row][col] = '.'
                        }
                    }
                    _ => panic!("there should not be a char other than . and #"),
                }
            }
        }
        // println!("{:?}", current);
        // println!("{:?}", new_map.grid);
        current = new_map.grid.clone();
        new_map = Lights::new(length);
    }

    // for line in current {
    //     println!("{line:?}");
    // }
    // println!("{new_map:?}");

    current
        .iter()
        .flatten()
        .filter(|p| p.to_owned().to_owned() == '#')
        .count()
}

// -> Vec<[i32; 2]>
fn find_neighbors(row: i32, col: i32, lights: &Vec<Vec<char>>) -> Vec<[usize; 2]> {
    let mut neighbor = vec![];
    for direction in DIRECTIONS {
        let ny = direction[0] + row;
        let nx = direction[1] + col;
        if (ny >= 0 && nx >= 0) && (ny < lights.len() as i32 && nx < lights[0].len() as i32) {
            neighbor.push([ny as usize, nx as usize])
        }
    }
    // println!("{neighbor:?}");
    neighbor
}

pub fn run() {
    // println!("{:?}", Lights::new(6));
    let c = read_file("q18.txt").unwrap();
    let lights: Lights = c.into();
    // println!("{lights:?}");

    let steps = 100;
    let result = move_steps(lights, steps, 100);
    println!("{result:?}");
}
