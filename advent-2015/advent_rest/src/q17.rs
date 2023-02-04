use std::fs;

#[derive(Debug)]
struct Containers {
    list: Vec<i32>,
}

impl From<String> for Containers {
    fn from(content: String) -> Self {
        let containers: Vec<i32> = content
            .lines()
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        Containers { list: containers }
    }
}

impl Containers {
    fn find_combinations(&self, liters: i32) {
        let mut combinations: Vec<Vec<i32>> = Vec::new();
        combinations.push(vec![]);
        for container in &self.list {
            let length = combinations.len();
            for idx in 0..length {
                let mut new_arr = combinations[idx].clone();
                new_arr.push(*container);
                // println!("{new_arr:?}");
                // println!("{container:?}");
                combinations.push(new_arr);
            }
        }
        // println!("{combinations:?}");
        let x: Vec<_> = combinations
            .iter()
            .filter(|p| p.iter().sum::<i32>() == liters)
            .collect();
        let min = x
            .iter()
            .min_by(|x, y| x.iter().len().cmp(&y.iter().len()))
            .unwrap();
        println!("{min:?}");
        println!(
            "{:?}",
            x.iter()
                .filter(|containers| containers.len() == min.len())
                .count()
        );
    }
}

pub fn run_code() {
    let path = "q17.txt";
    let content = fs::read_to_string(path).expect("fail to read a file");

    let containers: Containers = content.into();
    containers.find_combinations(150);
}
