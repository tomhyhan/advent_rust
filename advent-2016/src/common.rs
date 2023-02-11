use mockall::*;

#[automock]
pub mod my_modules {
    use std::fs;
    pub fn get_file(path: &str) -> Result<String, &'static str> {
        match fs::read_to_string(path) {
            Ok(content) => Ok(content),
            Err(_) => Err("fail to read a file"),
        }
    }
}

pub struct Config {
    pub quiz: usize,
}

impl Config {
    pub fn build(mut args: impl Iterator<Item = String>) -> Result<Self, &'static str> {
        args.next();

        let quiz = match args.next() {
            Some(arg) => arg.parse::<usize>().unwrap(),
            None => return Err("invalid input"),
        };
        Ok(Config { quiz })
    }
}
