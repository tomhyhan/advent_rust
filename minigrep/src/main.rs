use std::env;
use std::fs;

struct Config {
    query: String,
    path: String,
}

impl Config {
    fn new(args: &[String]) -> Self {
        Config {
            query: args[1].clone(),
            path: args[2].clone(),
        }
    }
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let config = Config::new(&args);
    let contents = fs::read_to_string(config.path).expect("fail to read a file");

    println!("{contents:?}");
}

// fn parse_config(args: &[String]) -> Config {
//     Config {
//         query: args[1].clone(),
//         path: args[2].clone(),
//     }
// }
